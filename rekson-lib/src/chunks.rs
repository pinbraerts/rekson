use std::io::{BufReader, Read};

pub struct ChunkReader<T: Read> {
    reader: BufReader<T>,
    size: usize,
}

impl<T: Read> Iterator for ChunkReader<T> {
    type Item = Box<[u8]>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut buffer = vec![0; self.size];
        match self.reader.read(&mut buffer) {
            Ok(0) => None,
            Ok(n) => {
                buffer.resize(n, 0);
                Some(buffer.into_boxed_slice())
            }
            Err(_) => None,
        }
    }
}

impl<T: Read> ChunkReader<T> {
    pub fn new(reader: BufReader<T>, size: usize) -> Self {
        Self { reader, size }
    }
}
