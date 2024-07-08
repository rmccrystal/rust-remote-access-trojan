use std::io;
use std::io::{Write, Read};
use crate::Framer;

/// A framing protocol that delimits messages with a length prefix.
pub struct LengthDelineated<R: Read, W: Write> {
    read: R,
    write: W,
}

impl<R: Read, W: Write> LengthDelineated<R, W> {
    pub fn new(read: R, write: W) -> Self {
        Self { read, write }
    }
}

impl<R: Read, W: Write> Framer for LengthDelineated<R, W> {
    fn send_frame(&mut self, block: &[u8]) -> io::Result<()> {
        let len_bytes = (block.len() as u32).to_le_bytes();
        self.write.write_all(len_bytes.as_slice())?;
        self.write.write_all(block)?;
        self.write.flush()?;
        Ok(())
    }

    fn receive_frame(&mut self) -> io::Result<Vec<u8>> {
        let mut len_bytes = [0; 4];
        self.read.read_exact(&mut len_bytes)?;
        let len = u32::from_le_bytes(len_bytes) as usize;
        let mut block = vec![0; len];
        self.read.read_exact(&mut block)?;
        Ok(block)
    }
}

#[cfg(test)]
mod test {
    fn length_delineated() {
        // TODO: Test
    }
}