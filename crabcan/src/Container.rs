use crate::cli::Opt;
use crate::error::Err;
use crate::config::Containeropts;

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
    if let Err(e) = container.create(){
        container.clean_exit()?;
        log::error!("Error in creating container:  {:?}" , e);
        return Err(e);
    }
    log::debug!("Finished! , Cleaning & Exit");
    container.clean_exit()
}