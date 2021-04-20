use std::io::{Read, Result, Write};

pub struct ReadStats<R> {
    inner: R,
    bytes: usize,
    reads: usize,
}

impl<R: Read> ReadStats<R> {
    pub fn new(inner: R) -> ReadStats<R> {
        Self {
            inner,
            bytes: 0,
            reads: 0,
        }
    }

    pub fn get_ref(&self) -> &R {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn reads(&self) -> usize {
        self.reads
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        self.reads += 1;
        self.inner.read(buf).map(|bytes| {
            self.bytes += bytes;
            bytes
        })
    }
}

pub struct WriteStats<W> {
    inner: W,
    bytes: usize,
    writes: usize,
}

impl<W: Write> WriteStats<W> {
    pub fn new(inner: W) -> WriteStats<W> {
        Self {
            inner,
            bytes: 0,
            writes: 0,
        }
    }

    pub fn get_ref(&self) -> &W {
        &self.inner
    }

    pub fn bytes_through(&self) -> usize {
        self.bytes
    }

    pub fn writes(&self) -> usize {
        self.writes
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        self.writes += 1;
        self.inner.write(buf).map(|bytes| {
            self.bytes += bytes;
            bytes
        })
    }

    fn flush(&mut self) -> Result<()> {
        self.inner.flush()
    }
}
