pub mod cli;
pub mod error;
use std::process::exit;
use error::exit_returncode;

#[inline(always)]
pub fn main() {
    match cli::parse_args(){
        Ok(args) => {
            log::info!("{:?}", args);
            exit_returncode(Ok(()))
        },
        Err(e) => {
            log::error!("Error while parsing arguments:\n\t{}", e);
            exit(e.get_return_code());
        }
    };
}
