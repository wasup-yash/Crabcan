use crate::cli::Opt;
use crate::error::Err;
use crate::config::Containeropts;
use nix::sys::utsname::uname;
#[allow(dead_code)]
pub struct Container{
    config:Containeropts,
}

impl Container {
    pub fn new(args:Opt) -> Result<Container, Err>{
        let config = Containeropts::new(
            args.command,
            args.uid,
            args.mount_dir
        )?;
        Ok(Container{
            config
        })
    }

    pub fn create(&mut self ) -> Result<() , Err> {
        log::debug!("Creation of container Finished");
        Ok(())
    }

    pub fn clean_exit(&mut self) -> Result<() , Err> {
        log::debug!("Container Cleaned");
        Ok(())
    }
}
pub fn start(args:Opt) -> Result<() , Err>{
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
pub fn check_linux_ver() -> Result<(), Err>{
        let host =  uname();
        log::debug!("linux Release: {}" , host.release());
        if let Ok(version) = scan_fmt!(host.release(), "{f}.{}" , f32){
            if version < MINIMAL_KERNAL_VERSION {
                return Err(Err::NotSupported(0));
            }
        }else {
            return Err(Err::ContainerError(0));
        }
        if host.machine() != "x86_64"{
            return   Err(Err::NotSupported(0));
        }
        Ok(())
}
