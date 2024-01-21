use crate::error::Ourerror;
use nix::mount::{mount, MsFlags};
use nix::unistd::pivot_root;
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
