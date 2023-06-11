use std::io;

pub trait SizedRead {
    /// use the next two bytes as the length of the next parts to read
    /// size limit is u16::MAX
    fn sized_read(&mut self) -> Result<Vec<u8>, io::Error>;
}

impl<R: io::Read> SizedRead for R {
    fn sized_read(&mut self) -> Result<Vec<u8>, io::Error> {
        let mut len_buffer = [0u8; 2];
        self.read(&mut len_buffer)?;

        // parse the length from the next two bytes
        let len = len_buffer[0] as usize * 256 + len_buffer[1] as usize;

        let mut buffer = vec![0u8; len];
        self.read(&mut buffer)?;
        Ok(buffer)
    }
}

pub trait SizedWrite {
    /// use the next two bytes as the length of the next parts to write
    /// size limit is u16::MAX
    fn sized_write(&mut self, data: &[u8]) -> Result<(), io::Error>;
}

impl<W: io::Write> SizedWrite for W {
    fn sized_write(&mut self, data: &[u8]) -> Result<(), io::Error> {
        if data.len() > u16::MAX as usize {
            Err(io::ErrorKind::InvalidInput)?
        }

        let mut sized = Vec::with_capacity(data.len() + 2);

        sized.push((data.len() / 256) as u8); // upper
        sized.push((data.len() % 256) as u8); // lower
        sized.extend_from_slice(&data);

        self.write_all(&sized)
    }
}
