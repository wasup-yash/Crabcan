use crate::error::Ourerror;
use nix::mount::{mount, MsFlags};
use nix::unistd::{chdir, pivot_root};
use rand::Rng;
use std::fs::create_dir_all;
use std::path::PathBuf;

pub fn create_directory(path: &PathBuf) -> Result<(), Ourerror> {
    match create_dir_all(path) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Cannot create directory {}; {}", path.to_str().unwrap(), e);
            Err(Ourerror::MountError(2))
        }
    }
}
// Taken from https://rust-lang-nursery.github.io/rust-cookbook/algorithms/randomness.html
pub fn random_string(n: usize) -> String {
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
    abcdefghijklmnopqrstuvwxyz\
    0123456789";
    let mut rng = rand::thread_rng();
    let name: String = (0..n)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect();
    name
}

pub fn setmountpoint(mount_dir: &PathBuf) -> Result<(), Ourerror> {
    log::debug!("Setting mount points....");
    mount_directory(
        None,
        &PathBuf::from("/"),
        vec![MsFlags::MS_REC, MsFlags::MS_PRIVATE],
    )?;
    let new_root = PathBuf::from(format!("/tmp/crabcan.{}", random_string(12)));
    create_directory(&new_root)?;
    mount_directory(
        Some(&mount_dir),
        &new_root,
        vec![MsFlags::MS_BIND, MsFlags::MS_PRIVATE],
    )?;
    log::debug!("Pivoting root");
    let old_root_tail = format!("oldroot.{}", random_string(6));
    let put_old = new_root.join(PathBuf::from(old_root_tail.clone()));
    create_directory(&put_old)?;
    if let Err(_) = pivot_root(&new_root, &put_old) {
        return Err(Ourerror::MountError(4));
    }
    log::debug!("Unmounting old root");
    let old_root = PathBuf::from(format!("/{}", old_root_tail));
    // Ensure we are not inside the directory we want to umount
    if let Err(_) = chdir(&PathBuf::from("/")) {
        return Err(Ourerror::MountError(5));
    }
    unmount_path(&old_root)?;
    delete_dir(&old_root)?;
    Ok(())
}

pub fn clean_mounts(_rootpath: &PathBuf) -> Result<(), Ourerror> {
    Ok(())
}

pub fn mount_directory(
    path: Option<&PathBuf>,
    mount_point: &PathBuf,
    flags: Vec<MsFlags>,
) -> Result<(), Ourerror> {
    //Settings up the mount flags
    let mut ms_flags = MsFlags::empty();
    for f in flags.iter() {
        ms_flags.insert(*f);
    }
    //Calling the syscall ,Handling errors
    match mount::<PathBuf, PathBuf, PathBuf, PathBuf>(path, mount_point, None, ms_flags, None) {
        Ok(_) => Ok(()),
        Err(e) => {
            if let Some(p) = path {
                log::error!(
                    "Cannot mount {} to {}: {}",
                    p.to_str().unwrap(),
                    mount_point.to_str().unwrap(),
                    e
                );
            } else {
                log::error!("Cannot remount {} : {}", mount_point.to_str().unwrap(), e);
            }
            Err(Ourerror::MountError(3))
        }
    }
}

use nix::mount::{umount2, MntFlags};
pub fn unmount_path(path: &PathBuf) -> Result<(), Ourerror> {
    match umount2(path, MntFlags::MNT_DETACH) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!("Unable to umount {}: {}", path.to_str().unwrap(), e);
            Err(Ourerror::MountError(0))
        }
    }
}
use std::fs::remove_dir;
pub fn delete_dir(path: &PathBuf) -> Result<(), Ourerror> {
    match remove_dir(path.as_path()) {
        Ok(_) => Ok(()),
        Err(e) => {
            log::error!(
                "Unable to delete directory {}: {}",
                path.to_str().unwrap(),
                e
            );
            Err(Ourerror::MountError(1))
        }
    }
}
