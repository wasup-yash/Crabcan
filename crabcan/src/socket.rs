use crate::error::Err;
use std::os::unix::io::RawFd;
use nix::sys::socket::{socketpair , AddressFamily, SocketType, SocketFlag, send, MsgFlags, recv};

pub fn generate_socketpair() -> Result<(RawFd, RawFd) , Err>{
    match socketpair(
        AddressFamily::Unix,
        SocketType::SeqPacket,
        None,
        SocketFlag::SOCK_CLOEXEC 
     ){
        Ok(res) => Ok(res),
        Err() => Err(Err::SocketError(0))
     }
     
}