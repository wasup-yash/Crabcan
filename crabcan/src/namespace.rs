use crate::error::Ourerror;
use crate::ipc::{recv_boolean, send_boolean};
use nix::sched::{unshare, CloneFlags};
use nix::unistd::Pid;
use nix::unistd::{setgroups, setresgid, setresuid};
use nix::unistd::{Gid, Uid};
use std::fs::File;
use std::io::Write;
use std::os::unix::io::RawFd;
//It is executed by the child process during its Configuration.
pub fn userns(fd: RawFd, uid: u32) -> Result<(), Ourerror> {
    log::debug!("Settings up user namespace with UID {}", uid);
    let has_userns = match unshare(CloneFlags::CLONE_NEWUSER) {
        Ok(_) => true,
        Err(_) => false,
    };
    send_boolean(fd, has_userns)?;
    if recv_boolean(fd)? {
        return Err(Ourerror::NamespacesError(0));
    }
    if has_userns {
        log::info!("User namespace set up");
    } else {
        log::info!("User namespace not supported, continuing...");
    }
    log::debug!("Switching to uid {} / gid {}...", uid, uid);
    let gid = Gid::from_raw(uid);
    let uid = Uid::from_raw(uid);
    if let Err(_) = setgroups(&[gid]) {
        return Err(Ourerror::NamespacesError(1));
    }
    if let Err(_) = setresgid(gid, gid, gid) {
        return Err(Ourerror::NamespacesError(2));
    }
    if let Err(_) = setresuid(uid, uid, uid) {
        return Err(Ourerror::NamespacesError(3));
    }
    //Switch to UID/GID provided by the user.
    Ok(())
}

const USERNS_OFFSET: u64 = 10000;
const USERNS_COUNT: u64 = 2000;

//Called by the container for UID/GID mapping;
pub fn handle_child_uid_mp(pid: Pid, fd: RawFd) -> Result<(), Ourerror> {
    if recv_boolean(fd)? {
        //perform UID/GID map here
        if let Ok(mut uid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "uid_map")) {
            if let Err(_) =
                uid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes())
            {
                return Err(Ourerror::NamespacesError(4));
            }
        } else {
            return Err(Ourerror::NamespacesError(5));
        }
        if let Ok(mut gid_map) = File::create(format!("/proc/{}/{}", pid.as_raw(), "gid_map")) {
            if let Err(_) =
                gid_map.write_all(format!("0 {} {}", USERNS_OFFSET, USERNS_COUNT).as_bytes())
            {
                return Err(Ourerror::NamespacesError(6));
            }
        } else {
            return Err(Ourerror::NamespacesError(7));
        }
    } else {
        log::info!("No user namespace set up from child process");
    }
    log::debug!("Child UID/GID map done, sending signal to child to continue...");
    send_boolean(fd, false)
}
