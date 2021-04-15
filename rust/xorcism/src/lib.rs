use std::borrow::Borrow;
use std::io::{Read, Write};
use std::iter::Cycle;
use std::slice::Iter;

pub trait Captures<'a> {}
impl<'a, T: ?Sized> Captures<'a> for T {}

/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'k>(Cycle<Iter<'k, u8>>);

pub struct XorcismIO<'k, IO> {
    xorcism: Xorcism<'k>,
    io: IO,
}

impl<'k> Xorcism<'k> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<K>(key: &'k K) -> Xorcism<'k>
    where
        K: AsRef<[u8]> + ?Sized,
    {
        Xorcism(key.as_ref().iter().cycle())
    }

    #[inline(always)]
    fn read_key(&mut self) -> u8 {
        *self.0.next().expect("infallible")
    }

    /// XOR each byte of the input buffer with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        data.iter_mut().for_each(|d| *d ^= self.read_key())
    }

    /// XOR each byte of the data with a byte from the key.
    ///
    /// Note that this is stateful: repeated calls are likely to produce different results,
    /// even with identical inputs.
    ///
    /// Should accept anything which has a cheap conversion to a byte iterator.
    /// Shouldn't matter whether the byte iterator's values are owned or borrowed.
    pub fn munge<D, I>(&mut self, data: D) -> impl Iterator<Item = u8> + Captures<'k> + '_
    where
        D: IntoIterator<Item = I>,
        D::IntoIter: 'k,
        I: Borrow<u8>,
    {
        data.into_iter().map(move |d| d.borrow() ^ self.read_key())
    }

    #[cfg(feature = "io")]
    pub fn reader(self, reader: impl Read + 'k) -> impl Read + 'k {
        XorcismIO {
            xorcism: self,
            io: reader,
        }
    }

    #[cfg(feature = "io")]
    pub fn writer(self, writer: impl Write + 'k) -> impl Write + 'k {
        XorcismIO {
            xorcism: self,
            io: writer,
        }
    }
}

impl<'k, R: Read> Read for XorcismIO<'k, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.io.read(buf).map(|len| {
            self.xorcism.munge_in_place(buf);
            len
        })
    }
}

impl<'k, R: Write> Write for XorcismIO<'k, R> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buf = buf.to_owned();
        self.xorcism.munge_in_place(&mut buf);
        self.io.write(&buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.io.flush()
    }
}
