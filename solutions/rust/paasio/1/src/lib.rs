use std::io::{Read, Result, Write};

// the PhantomData instances in this file are just to stop compiler complaints
// about missing generics; feel free to remove them

pub struct ReadStats<R>
{
    wrapped: R,
    nbuf: usize,
    nopt: usize,
}

impl<R: Read> ReadStats<R> {
    // _wrapped is ignored because R is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: R) -> ReadStats<R> {
        Self{wrapped: wrapped, nbuf: 0, nopt: 0}
    }

    pub fn get_ref(&self) -> &R { 
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nbuf
    }

    pub fn reads(&self) -> usize {
        self.nopt
    }
}

impl<R: Read> Read for ReadStats<R> {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let n = self.wrapped.read(buf)?;
        self.nbuf += n;
        self.nopt += 1;
        Ok(n)
    }
}

pub struct WriteStats<W>
{
    wrapped: W,
    nbuf: usize,
    nopt: usize,
}

impl<W: Write> WriteStats<W> {
    // _wrapped is ignored because W is not bounded on Debug or Display and therefore
    // can't be passed through format!(). For actual implementation you will likely
    // wish to remove the leading underscore so the variable is not ignored.
    pub fn new(wrapped: W) -> WriteStats<W> {
        Self{wrapped: wrapped, nbuf: 0, nopt: 0}
    }

    pub fn get_ref(&self) -> &W {
        &self.wrapped
    }

    pub fn bytes_through(&self) -> usize {
        self.nbuf
    }

    pub fn writes(&self) -> usize {
        self.nopt
    }
}

impl<W: Write> Write for WriteStats<W> {
    fn write(&mut self, buf: &[u8]) -> Result<usize> {
        let n = self.wrapped.write(buf)?;
        self.nbuf += n;
        self.nopt += 1;
        Ok(n)
    }

    fn flush(&mut self) -> Result<()> {
        self.wrapped.flush()
    }
}
