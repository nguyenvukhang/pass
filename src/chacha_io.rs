use chacha20::cipher::StreamCipher;
use chacha20::ChaCha20;

use std::io;
use std::io::{Read, Write};
use std::result::Result as SResult;

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

pub struct ChaWriter<W: Write> {
    target: W,
    cipher: ChaCha20,
}

impl<W: Write> ChaWriter<W> {
    pub fn new(target: W, cipher: ChaCha20) -> Self {
        Self { target, cipher }
    }
}

impl<W: Write> Write for ChaWriter<W> {
    fn write(&mut self, buffer: &[u8]) -> SResult<usize, io::Error> {
        let mut buffer = buffer.to_vec();
        self.cipher.apply_keystream(&mut buffer);
        self.target.write(&buffer)
    }

    fn flush(&mut self) -> SResult<(), io::Error> {
        self.target.flush()
    }
}
