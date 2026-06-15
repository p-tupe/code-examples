pub struct CircularBuffer<T> {
    buf: Vec<T>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buf: Vec::with_capacity(capacity),
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.buf.len() == self.buf.capacity() {
            Err(Error::FullBuffer)
        } else {
            self.buf.push(_element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if self.buf.is_empty() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.buf.remove(0))
        }
    }

    pub fn clear(&mut self) {
        self.buf.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.buf.len() == self.buf.capacity() {
            self.buf.remove(0);
        }
        self.buf.push(_element);
    }
}
