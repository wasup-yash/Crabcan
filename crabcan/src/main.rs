pub mod cli;
pub mod error;
pub mod config;
pub mod container;
pub mod ipc;
pub mod child;
use std::process::exit;
use error::exit_returncode;
#[macro_use] extern crate scan_fmt;

#[inline(always)]
pub fn main() {
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_returncode(container::start(args))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_return_code());
        }
    };
}
