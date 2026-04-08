use std::{borrow::Borrow, io::{Cursor, Read, Write, BufWriter}};
// use std::mem;
/// A munger which XORs a key with some data
#[derive(Clone)]
pub struct Xorcism<'a> {
    // This field is just to suppress compiler complaints;
    // feel free to delete it at any point.
    key: &'a [u8],
    pos: usize,
}

impl<'a> Xorcism<'a> {
    /// Create a new Xorcism munger from a key
    ///
    /// Should accept anything which has a cheap conversion to a byte slice.
    pub fn new<Key: AsRef<[u8]> + ?Sized >(key: &'a Key) -> Xorcism<'a>
    {
        Self {key: key.as_ref(), pos: 0}
    }

    fn munge_one(&mut self, item: &u8) -> u8 {
        let result = item ^ self.key[self.pos];
        self.pos = (self.pos + 1) % self.key.len();
        result
    }

    pub fn munge_in_place(&mut self, data: &mut [u8]) {
        for item in data.iter_mut() {
            *item = self.munge_one(item);
        }
    }

    pub fn munge<Data>(&mut self, data: Data) -> impl Iterator<Item = u8>
    where
        Data: IntoIterator + 'a, // 可能时
        Data::Item: Borrow<u8>, // 可以处理Vec<u8> 和 &[u8], 绑定数据结构，也是可以申明为泛型？
    {
        data.into_iter().map(|b| self.munge_one(b.borrow()))
    }

    pub fn reader(self, data: impl Read) -> impl Read + {
        XorcismReader{xor: self, inner: data}
    }

    pub fn writer(self, output: impl Write) -> impl Write {
        XorcismWriter{xor: self, inner: output}
    }

}


pub struct XorcismReader<'a, R: Read> {
    xor: Xorcism<'a>,
    inner: R,
}

impl<R: Read> Read for XorcismReader<'_, R> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        let n = self.inner.read(buf)?;
        self.xor.munge_in_place(buf);
        Ok(n)
    }
}

pub struct XorcismWriter<'a,  W: Write> {
    xor: Xorcism<'a>,
    inner: W,
}

impl<W: Write> Write for XorcismWriter<'_,  W> {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut buf = buf.to_vec();
        self.xor.munge_in_place(&mut  buf);
        self.inner.write(&buf)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        self.inner.flush()
    }
}
