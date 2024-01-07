pub mod cli;
pub mod error;
pub mod config;
pub mod Container;
use std::process::exit;
use error::exit_returncode;

#[inline(always)]
pub fn main() {
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_returncode(Container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_return_code());
        }
    };
}
