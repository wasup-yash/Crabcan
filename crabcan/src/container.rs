use crate::child::generate_child_process;
use crate::cli::Opt;
use crate::config::Containeropts;
use crate::error::Ourerror;
use crate::mount::clean_mounts;
use crate::namespace::handle_child_uid_mp;
use crate::resources::clean_cgroups;
use crate::resources::restrict_resources;
use nix::sys::utsname::uname;
use nix::sys::wait::waitpid;
use nix::unistd::close;
use nix::unistd::Pid;
use std::os::unix::io::RawFd;
#[allow(dead_code)]
pub struct Container {
    config: Containeropts,
    sockets: (RawFd, RawFd),
    child_pid: Option<Pid>,
}

impl Container {
    pub fn new(args: Opt) -> Result<Container, Ourerror> {
        let (config, sockets) = Containeropts::new(args.command, args.uid, args.mount_dir)?;
        Ok(Container {
            sockets,
            config,
            child_pid: None,
        })
    }

    pub fn create(&mut self) -> Result<(), Ourerror> {
        let pid = generate_child_process(self.config.clone())?;
        restrict_resources(&self.config.hostname, &pid)?;
        handle_child_uid_mp(pid, self.sockets.0)?;
        self.child_pid = Some(pid);
        log::debug!("Creation of container Finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<(), Ourerror> {
        if let Err(e) = close(self.sockets.0) {
            log::error!("Unable to close write sockets: {:?} ", e);
            return Err(Ourerror::SocketError(3));
        }
        clean_mounts(&self.config.mount_dir)?;
        if let Err(e) = close(self.sockets.1) {
            log::error!("Unable to close read sockets: {:?} ", e);
            return Err(Ourerror::SocketError(4));
        }
        if let Err(e) = clean_cgroups(&self.config.hostname) {
            log::error!("Cgroups cleaning failed: {}", e);
            return Err(e);
        }
        log::debug!("Container Cleaned");
        Ok(())
    }
}
pub fn start(args: Opt) -> Result<(), Ourerror> {
    let mut container = Container::new(args)?;
    check_linux_ver()?;
    if let Err(e) = container.create() {
        container.clean_exit()?;
        log::error!("Error in creating container:  {:?}", e);
        return Err(e);
    }
    log::debug!("Container Child PID: {:?}", container.child_pid);
    wait_child(container.child_pid)?;
    log::debug!("Finished! , Cleaning & Exit");
    container.clean_exit()
}
pub fn wait_child(pid: Option<Pid>) -> Result<(), Ourerror> {
    if let Some(child_pid) = pid {
        log::debug!("Waiting for child (pid {}) to finish", child_pid);
        if let Err(e) = waitpid(child_pid, None) {
            log::error!("Error while waiting for pid to finish: {:?}", e);
            return Err(Ourerror::ContainerError(1));
        }
    }
    Ok(())
}

pub const MINIMAL_KERNAL_VERSION: f32 = 4.8;
pub fn check_linux_ver() -> Result<(), Ourerror> {
    let host = uname();
    log::debug!("linux Release: {}", host.release());
    if let Ok(version) = scan_fmt!(host.release(), "{f}.{}", f32) {
        if version < MINIMAL_KERNAL_VERSION {
            return Err(Ourerror::NotSupported(0));
        }
    } else {
        return Err(Ourerror::ContainerError(0));
    }
    if host.machine() != "x86_64" {
        return Err(Ourerror::NotSupported(0));
    }
    Ok(())
}
