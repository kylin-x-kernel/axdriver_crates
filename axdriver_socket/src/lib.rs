//! Common traits and types for socket communite device drivers (i.e. disk).

#![no_std]
#![cfg_attr(doc, feature(doc_auto_cfg))]

#[doc(no_inline)]
pub use axdriver_base::{BaseDriverOps, DevError, DevResult, DeviceType};

/// Operations that require a block storage device driver to implement.
pub trait SocketDriverOps: BaseDriverOps {
    type SocketAddr: Copy;

    /// Listen on a specific port.
    fn listen(&mut self, addr: Self::SocketAddr) -> DevResult<()>;

    /// Connect to a peer socket.
    fn connect(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()>;

    /// Send data to the connected peer socket. need addr for DGRAM mode
    fn send(
        &mut self,
        peer: Self::SocketAddr,
        src: Self::SocketAddr,
        buf: &[u8],
    ) -> DevResult<usize>;

    /// Receive data from the connected peer socket.
    fn recv(
        &mut self,
        peer: Self::SocketAddr,
        src: Self::SocketAddr,
        buf: &mut [u8],
    ) -> DevResult<usize>;

    /// Returns the number of bytes in the receive buffer available to be read by recv.
    fn recv_avail(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<usize>;

    /// Disconnect from the connected peer socket.
    ///
    /// Requests to shut down the connection cleanly, telling the peer that we won't send or receive
    /// any more data.
    fn disconnect(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()>;

    /// Forcibly closes the connection without waiting for the peer.
    fn abort(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()>;
}
