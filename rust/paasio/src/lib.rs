use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R> {
    _wrapped: R,
    bytes_through: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: R) -> ReadStats<R> {
        ReadStats {
            _wrapped,
            bytes_through: 0,
            reads: 0,
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn get_ref(&self) -> &R {
        &self._wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let bytes_read = self._wrapped.read(buf)?;
        self.bytes_through += bytes_read;
        self.reads += 1;
        Ok(bytes_read)
    }
}

pub struct WriteStats<W> {
    _wrapped: W,
    bytes_through: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(_wrapped: W) -> WriteStats<W> {
        WriteStats {
            _wrapped,
            bytes_through: 0,
            writes: 0,
        }
    }

    #[cfg(not(tarpaulin_include))]
    pub fn get_ref(&self) -> &W {
        &self._wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes_through
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let bytes_written = self._wrapped.write(buf)?;
        self.bytes_through += bytes_written;
        self.writes += 1;
        Ok(bytes_written)
    }

    fn flush(&mut self) -> Result<()> {
        self._wrapped.flush()
    }
}
