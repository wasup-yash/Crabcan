use crate::error::Ourerror;
use nix::sys::socket::{recv, send, socketpair, AddressFamily, MsgFlags, SockFlag, SockType};
use std::os::unix::io::RawFd;

pub fn generate_socketpair() -> Result<(RawFd, RawFd), Ourerror> {
    match socketpair(
        AddressFamily::Unix,
        SockType::SeqPacket,
        None,
        SockFlag::SOCK_CLOEXEC,
    ) {
        Ok(res) => Ok(res),
        Err(_) => Err(Ourerror::SocketError(0)),
    }
}

pub fn send_boolean(fd: RawFd, boolean: bool) -> Result<(), Ourerror> {
    let data: [u8; 1] = [boolean.into()];
    if let Err(e) = send(fd, &data, MsgFlags::empty()) {
        log::error!("Cannot send boolean through sockets: {:?}", e);
        return Err(Ourerror::SocketError(1));
    };
    Ok(())
}

pub fn recv_boolean(fd: RawFd) -> Result<bool, Ourerror> {
    let mut data: [u8; 1] = [0];
    if let Err(e) = recv(fd, &mut data, MsgFlags::empty()) {
        log::error!("Cannot receive boolean value through sockets: {:?}", e);
        return Err(Ourerror::SocketError(2));
    };
    Ok(data[0] == 1)
}
