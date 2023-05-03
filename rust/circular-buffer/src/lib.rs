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
        unimplemented!("Write the passed element to the CircularBuffer or return FullBuffer error if CircularBuffer is full.");
    }

    pub fn read(&mut self) -> Result<T, Error> {
        unimplemented!("Read the oldest element from the CircularBuffer or return EmptyBuffer error if CircularBuffer is empty.");
    }

    pub fn clear(&mut self) {
        self.buffer.clear();
    }

    pub fn overwrite(&mut self, _element: T) {
        unimplemented!("Write the passed element to the CircularBuffer, overwriting the existing elements if CircularBuffer is full.");
    }
}
