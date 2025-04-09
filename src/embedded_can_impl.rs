use crate::{CanFrame, CanSocket};
use embedded_can::{nb::Can, Error, ErrorKind, Frame, Id};
use serial_core::SerialPort;
use std::fmt;
use std::io;

impl Frame for CanFrame {
    fn new(id: impl Into<Id>, data: &[u8]) -> Option<Self> {
        Some(CanFrame::new(id.into(), data.len(), data))
    }

    fn new_remote(_id: impl Into<Id>, _dlc: usize) -> Option<Self> {
        // currently unsupported
        None
    }

    fn is_extended(&self) -> bool {
        matches!(self.id, Id::Extended(_))
    }

    fn is_remote_frame(&self) -> bool {
        // currently unsupported
        false
    }

    fn id(&self) -> Id {
        self.id
    }

    fn dlc(&self) -> usize {
        self.dlc
    }

    fn data(&self) -> &[u8] {
        &self.data
    }
}

#[derive(Debug)]
pub struct IOError(io::Error);

impl fmt::Display for IOError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "CAN I/O error: {}", self.0)
    }
}

impl IOError {
    pub fn inner(&self) -> &std::io::Error {
        &self.0
    }
}

impl From<std::io::Error> for IOError {
    fn from(e: std::io::Error) -> Self {
        Self(e)
    }
}

impl Error for IOError {
    fn kind(&self) -> ErrorKind {
        ErrorKind::Other
    }
}

impl<P: SerialPort> Can for CanSocket<P> {
    type Frame = CanFrame;

    type Error = IOError;

    fn transmit(&mut self, frame: &Self::Frame) -> nb::Result<Option<Self::Frame>, Self::Error> {
        self.write(frame.id, frame.data())
            .map(|_| None)
            .map_err(|io_err| {
                if io_err.kind() == io::ErrorKind::WouldBlock {
                    nb::Error::WouldBlock.into()
                } else {
                    nb::Error::Other(io_err.into())
                }
            })
    }

    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        self.read().map_err(|io_err| {
            if io_err.kind() == io::ErrorKind::WouldBlock {
                nb::Error::WouldBlock.into()
            } else {
                nb::Error::Other(io_err.into())
            }
        })
    }
}
