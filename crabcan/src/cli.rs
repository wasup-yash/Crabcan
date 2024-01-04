use std::path::PathBuf;
use structopt::StructOpt;

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
pub fn parse_args() -> Opt{
        let args  = Opt::from_args();
        args 
    }