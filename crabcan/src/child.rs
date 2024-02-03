use crate::cap::setcapabilities;
use crate::config::Containeropts;
use crate::error::Ourerror;
use crate::hostname::set_container_hostname;
use crate::mount::setmountpoint;
use crate::namespace::userns;
use crate::syscall::setsyscalls;
use nix::sched::clone;
use nix::sched::CloneFlags;
use nix::sys::signal::Signal;
use nix::unistd::close;
use nix::unistd::execve;
use nix::unistd::Pid;
use std::ffi::CString;

fn child(config: Containeropts) -> isize {
    match set_container_configuration(&config) {
        Ok(_) => log::info!("Container set up successfully"),
        Err(e) => {
            log::error!("Error while configuration container: {:?}", e);
            return -1;
        }
    }
    if let Err(_) = close(config.fd) {
        log::error!("Error while closing socket");
        return -1;
    }
    log::info!(
        " Starting with command {} and arg {:?}",
        config.path.to_str().unwrap(),
        config.argv
    );
    let retcode = match execve::<CString, CString>(&config.path, &config.argv, &[]) {
        Ok(_) => 0,
        Err(e) => {
            log::error!("Error while trying to perform execve: {:?}", e);
            return -1;
        }
    };
    retcode
}
const STACK_SIZE: usize = 1024 * 1024;
pub fn generate_child_process(config: Containeropts) -> Result<Pid, Ourerror> {
    let mut tmp_stack: [u8; STACK_SIZE] = [0; STACK_SIZE];
    //Flag defination
    let mut flags = CloneFlags::empty();
    flags.insert(CloneFlags::CLONE_NEWNS);
    flags.insert(CloneFlags::CLONE_NEWCGROUP);
    flags.insert(CloneFlags::CLONE_NEWPID);
    flags.insert(CloneFlags::CLONE_NEWIPC);
    flags.insert(CloneFlags::CLONE_NEWNET);
    flags.insert(CloneFlags::CLONE_NEWUTS);
    match clone(
        Box::new(|| child(config.clone())),
        &mut tmp_stack,
        flags,
        Some(Signal::SIGCHLD as i32),
    ) {
        Ok(pid) => Ok(pid),
        Err(_) => Err(Ourerror::ChildProcessError(0)),
    }
}

fn set_container_configuration(config: &Containeropts) -> Result<(), Ourerror> {
    set_container_hostname(&config.hostname)?;
    setmountpoint(&config.mount_dir)?;
    userns(config.fd, config.uid)?;
    setcapabilities()?;
    setsyscalls()?;
    Ok(())
}
