pub mod cli;
#[inline(always)]
fn main() {
let args = cli::parse_args();
    println!("{:?}\n" , args);
    log::info!("{:?}" , args);

}
