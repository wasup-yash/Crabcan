pub mod cap;
pub mod child;
pub mod cli;
pub mod config;
pub mod container;
pub mod error;
pub mod hostname;
pub mod ipc;
pub mod mount;
pub mod namespace;
pub mod syscall;
pub mod resources;
use error::exit_returncode;
use std::process::exit;
#[macro_use]
extern crate scan_fmt;

#[inline(always)]
pub fn main() {
    match cli::parse_args() {
        Ok(args) => {
            log::info!("{:?}", args);
            exit_returncode(container::start(args))
        }
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_return_code());
        }
    };
}
