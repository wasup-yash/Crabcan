use std::path::PathBuf;
use log::LevelFilter;
use structopt::StructOpt;
use crate::error::Ourerror;

#[derive(Debug, StructOpt)]
#[structopt(name = "Crabcan", about = "Container in rust")]
pub struct Opt{
    ///Activate debug mode  
    #[structopt(short , long)]
    debug: bool, 
    /// Command to execute inside the container
    #[structopt(short, long)]
    pub command: String,
    /// User ID to create inside the container
    #[structopt(short, long)]
    pub uid: u32,
    /// Directory to mount as root of the container
    #[structopt(parse(from_os_str), short = "m", long = "mount")]
    pub mount_dir: PathBuf,

}
#[inline(always)]
pub fn setup_log(level : LevelFilter){
    env_logger::Builder::from_default_env()
        .format_timestamp_secs()
        .filter(None, level)
        .init();
}
pub fn parse_args() -> Result<Opt , Ourerror>{
        let args  = Opt::from_args();   
        if args.debug{
            setup_log(LevelFilter::Debug);
        }else {
            setup_log(LevelFilter::Info)
        }
        if !args.mount_dir.exists() || !args.mount_dir.is_dir(){
            return Err(Ourerror::ArgumentInvalid("mount"));
        }
        Ok(args)
    }