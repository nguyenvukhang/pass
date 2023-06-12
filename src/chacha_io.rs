use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;

use std::io;
use std::io::{Read, Write};
use std::result::Result as SResult;

/// A decryption warapper for `io::Read`.
pub struct ChaReader<R: Read> {
    src: R,
    cipher: ChaCha20,
}

impl<R: Read> ChaReader<R> {
    pub fn new(src: R, cipher: ChaCha20) -> Self {
        Self { src, cipher }
    }
}

impl<R: Read> Read for ChaReader<R> {
    fn read(&mut self, buffer: &mut [u8]) -> SResult<usize, io::Error> {
        let count_result = self.src.read(buffer);
        self.cipher.apply_keystream(buffer);
        count_result
    }
}

/// A encryption warapper for `io::Write`.
pub struct ChaWriter<W: Write> {
    dst: W,
    cipher: ChaCha20,
}

impl<W: Write> ChaWriter<W> {
    pub fn new(dst: W, cipher: ChaCha20) -> Self {
        Self { dst, cipher }
    }
}

impl<W: Write> Write for ChaWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> SResult<usize, io::Error> {
        let mut buffer = buffer.to_vec();
        self.cipher.apply_keystream(&mut buffer);
        self.dst.write(&buffer)
    }

    fn flush(&mut self) -> SResult<(), io::Error> {
        self.dst.flush()
    }
}
