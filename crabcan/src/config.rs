use crate::error::Ourerror;
use crate::ipc::generate_socketpair;
use std::ffi::CString;
use std::os::unix::io::RawFd;
use std::path::PathBuf;
#[derive(Clone)]
pub struct Containeropts {
    pub path: CString,
    pub argv: Vec<CString>,
    pub uid: u32,
    pub mount_dir: PathBuf,
    pub fd: RawFd,
}
impl Containeropts {
    pub fn new(
        command: String,
        uid: u32,
        mount_dir: PathBuf,
    ) -> Result<(Containeropts, (RawFd, RawFd)), Ourerror> {
        let sockets = generate_socketpair()?;
        let argv: Vec<CString> = command
            .split_ascii_whitespace()
            .map(|s| CString::new(s).expect("Cannot read argument given"))
            .collect();
        let path = argv[0].clone();
        Ok((
            Containeropts {
                path,
                argv,
                uid,
                mount_dir,
                fd: sockets.1.clone(),
            },
            sockets,
        ))
    }
}
