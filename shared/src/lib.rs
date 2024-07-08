use std::io;

pub mod protocol;
pub mod packet;

/// A type that is able to read and write frames of arbitrary length from a stream
pub trait Framer {
    /// Send a frame of data
    fn send_frame(&mut self, block: &[u8]) -> io::Result<()>;
    
    /// Serialize a frame of data and send it
    fn send_message(&mut self, message: &impl Serialize) -> io::Result<()> {
        self.send_frame(message.to_bytes().as_slice())
    }

    /// Receive a frame of data
    fn receive_frame(&mut self) -> io::Result<Vec<u8>>;
    
    /// Receive a frame of data and deserialize it
    fn receive_message<T: Deserialize>(&mut self) -> io::Result<Result<T, String>> {
        self.receive_frame().map(|bytes| T::from_bytes(bytes.as_slice()))
    }
}

/// A type that can be serialized to bytes
pub trait Serialize {
    fn to_bytes(&self) -> Vec<u8>;
}

/// A type that can be deserialized from bytes
pub trait Deserialize: Sized {
    fn from_bytes(bytes: &[u8]) -> Result<Self, String>;
}
