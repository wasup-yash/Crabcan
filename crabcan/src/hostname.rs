use crate::error::Ourerror;
use nix::unistd::sethostname;
use rand::seq::SliceRandom;
use rand::Rng;

pub fn generate_hostname() -> Result<String, Ourerror> {
    let mut rng = rand::thread_rng();
    let num = rng.gen::<u8>();
    let name = HOSTNAME_NAMES.choose(&mut rng).ok_or(Ourerror::RngError)?;
    let adj = HOSTNAME_ADJ.choose(&mut rng).ok_or(Ourerror::RngError)?;
    Ok(format!("{}-{}-{}", adj, name, num))
}
const HOSTNAME_NAMES: [&'static str; 8] = [
    "cat", "world", "coffee", "girl", "man", "book", "pinguin", "moon",
];
const HOSTNAME_ADJ: [&'static str; 16] = [
    "blue",
    "red",
    "green",
    "yellow",
    "big",
    "small",
    "tall",
    "thin",
    "round",
    "square",
    "triangular",
    "weird",
    "noisy",
    "silent",
    "soft",
    "irregular",
];

pub fn set_container_hostname(hostname: &String) -> Result<(), Ourerror> {
    match sethostname(hostname) {
        Ok(_) => {
            log::debug!("Container hostname is now {}", hostname);
            Ok(())
        }
        Err(_) => {
            log::error!("Hostname {} is not set for the Container", hostname);
            Err(Ourerror::HostnameError(0))
        }
    }
}
