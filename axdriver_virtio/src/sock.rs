use crate::as_dev_err;
use axdriver_base::{BaseDriverOps, DevResult, DeviceType};
use axdriver_socket::SocketDriverOps;
use virtio_drivers::device::socket::{
    SocketError, VirtIOSocket, VsockAddr, VsockConnectionManager as InnerDev,
};
use virtio_drivers::{Error as VirtIoError, Hal, transport::Transport};

/// The VirtIO socket device driver.
pub struct VirtIoSocketDev<H: Hal, T: Transport> {
    inner: InnerDev<H, T>,
}

unsafe impl<H: Hal, T: Transport> Send for VirtIoSocketDev<H, T> {}
unsafe impl<H: Hal, T: Transport> Sync for VirtIoSocketDev<H, T> {}

impl<H: Hal, T: Transport> VirtIoSocketDev<H, T> {
    /// Creates a new driver instance and initializes the device, or returns
    /// an error if any step fails.
    pub fn try_new(transport: T) -> DevResult<Self> {
        let viotio_socket = VirtIOSocket::<H, _>::new(transport).map_err(as_dev_err)?;
        Ok(Self {
            inner: InnerDev::new(viotio_socket),
        })
    }
}

impl<H: Hal, T: Transport> BaseDriverOps for VirtIoSocketDev<H, T> {
    fn device_name(&self) -> &str {
        "virtio-vsocket"
    }

    fn device_type(&self) -> DeviceType {
        DeviceType::Socket
    }
}

impl<H: Hal, T: Transport> SocketDriverOps for VirtIoSocketDev<H, T> {
    type SocketAddr = VsockAddr;

    fn listen(&mut self, src: Self::SocketAddr) -> DevResult<()> {
        self.inner.listen(src.port);
        Ok(())
    }

    fn connect(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()> {
        self.inner.connect(peer, src.port).map_err(as_dev_err)
    }

    fn send(
        &mut self,
        peer: Self::SocketAddr,
        src: Self::SocketAddr,
        buf: &[u8],
    ) -> DevResult<usize> {
        match self.inner.send(peer, src.port, buf) {
            Ok(()) => Ok(buf.len()),
            Err(e) => Err(as_dev_err(e)),
        }
    }

    fn recv(
        &mut self,
        peer: Self::SocketAddr,
        src: Self::SocketAddr,
        buf: &mut [u8],
    ) -> DevResult<usize> {
        self.inner.recv(peer, src.port, buf).map_err(as_dev_err)
    }

    fn recv_avail(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<usize> {
        self.inner.recv_buffer_available_bytes(peer, src.port).map_err(as_dev_err)
    }

    fn disconnect(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()> {
        self.inner.shutdown(peer, src.port).map_err(as_dev_err)
    }

    fn abort(&mut self, peer: Self::SocketAddr, src: Self::SocketAddr) -> DevResult<()> {
        self.inner.force_close(peer, src.port).map_err(as_dev_err)
    }
}
