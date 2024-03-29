#[derive(Debug, PartialEq, Eq)]
pub struct CircularBuffer<T> {
    buffer: Vec<T>,
    capacity: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    EmptyBuffer,
    FullBuffer,
}

impl<T> CircularBuffer<T> {
    pub fn new(capacity: usize) -> Self {
        CircularBuffer {
            buffer: Vec::with_capacity(capacity),
            capacity,
        }
    }

    pub fn write(&mut self, _element: T) -> Result<(), Error> {
        if self.capacity == self.buffer.len() {
            Err(Error::FullBuffer)
        } else {
            self.buffer.push(_element);
            Ok(())
        }
    }

    pub fn read(&mut self) -> Result<T, Error> {
        if 0 == self.buffer.len() {
            Err(Error::EmptyBuffer)
        } else {
            Ok(self.buffer.remove(0))
        }
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        if self.capacity == self.buffer.len() {
            self.buffer.remove(0);
        }
        self.buffer.push(_element);
    }
}
