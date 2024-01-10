use crate::cli::Opt;
use crate::error::Ourerror;
use crate::config::Containeropts;   
use nix::sys::utsname::uname;
use nix::unistd::close;
use std::os::unix::io::RawFd;
#[allow(dead_code)]
pub struct Container{
    config:Containeropts,
    sockets:(RawFd, RawFd)
}

impl Container {
    pub fn new(args:Opt) -> Result<Container, Ourerror>{
        let (config, sockets) = Containeropts::new(
            args.command,
            args.uid,
            args.mount_dir
        )?;
        Ok(Container{
            sockets,
            config,
        })
    }

    pub fn create(&mut self ) -> Result<() , Ourerror> {
        log::debug!("Creation of container Finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<() , Ourerror> {
        if let Err(e) = close(self.sockets.0){
            log::error!("Unable to close write sockets: {:?} ", e);
            return Err(Ourerror::SocketError(3))
        }
        if let Err(e) = close(self.sockets.1){
            log::error!("Unable to close read sockets: {:?} ", e);
            return Err(Ourerror::SocketError(4))
        }
        log::debug!("Container Cleaned");
        Ok(())
    }
}
pub fn start(args:Opt) -> Result<() , Ourerror>{
    let mut container = Container::new(args)?;
    check_linux_ver()?;
    if let Err(e) = container.create(){
        container.clean_exit()?;
        log::error!("Error in creating container:  {:?}" , e);
        return Err(e);
    }
    log::debug!("Finished! , Cleaning & Exit");
    container.clean_exit()
}

pub const MINIMAL_KERNAL_VERSION: f32 = 4.8;
pub fn check_linux_ver() -> Result<(), Ourerror>{
        let host =  uname();
        log::debug!("linux Release: {}" , host.release());
        if let Ok(version) = scan_fmt!(host.release(), "{f}.{}" , f32){
            if version < MINIMAL_KERNAL_VERSION {
                return Err(Ourerror::NotSupported(0));
            }
        }else {
            return Err(Ourerror::ContainerError(0));
        }
        if host.machine() != "x86_64"{
            return   Err(Ourerror::NotSupported(0));
        }
        Ok(())
}
