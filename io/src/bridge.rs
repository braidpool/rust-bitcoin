#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// A bridging wrapper providing the IO traits for types that already implement `std` IO traits.
#[repr(transparent)]
pub struct FromStd<T>(T);

impl<T> FromStd<T> {
    /// Wraps an IO type.
    #[inline]
    pub const fn new(inner: T) -> Self { Self(inner) }

    /// Returns the wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Returns a reference to the wrapped value.
    #[inline]
    pub fn inner(&self) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the wrapped value.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Wraps a mutable reference to IO type.
    #[inline]
    pub fn new_mut(inner: &mut T) -> &mut Self {
        // SAFETY: the type is repr(transparent) and the lifetimes match
        unsafe { &mut *(inner as *mut _ as *mut Self) }
    }

    /// Wraps a boxed IO type.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn new_boxed(inner: Box<T>) -> Box<Self> {
        // SAFETY: the type is repr(transparent) and the pointer is created from Box
        unsafe { Box::from_raw(Box::into_raw(inner) as *mut Self) }
    }
}

impl<T: std::io::Read> super::Read for FromStd<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> {
        self.0.read(buf).map_err(Into::into)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> super::Result<()> {
        self.0.read_exact(buf).map_err(Into::into)
    }
}

impl<T: std::io::BufRead> super::BufRead for FromStd<T> {
    #[inline]
    fn fill_buf(&mut self) -> super::Result<&[u8]> {
        self.0.fill_buf().map_err(Into::into)
    }

    #[inline]
    fn consume(&mut self, amount: usize) {
        self.0.consume(amount)
    }
}

impl<T: std::io::Write> super::Write for FromStd<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> super::Result<usize> {
        self.0.write(buf).map_err(Into::into)
    }

    #[inline]
    fn flush(&mut self) -> super::Result<()> {
        self.0.flush().map_err(Into::into)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> super::Result<()> {
        self.0.write_all(buf).map_err(Into::into)
    }
}

// We also impl std traits so that mixing the calls is not annoying.

impl<T: std::io::Read> std::io::Read for FromStd<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.0.read_exact(buf)
    }
}

impl<T: std::io::BufRead> std::io::BufRead for FromStd<T> {
    #[inline]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amount: usize) {
        self.0.consume(amount)
    }
}

impl<T: std::io::Write> std::io::Write for FromStd<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.0.write_all(buf)
    }
}

/// A bridging wrapper providing the std traits for types that already implement our traits.
#[repr(transparent)]
pub struct ToStd<T>(T);

impl<T> ToStd<T> {
    /// Wraps an IO type.
    #[inline]
    pub const fn new(inner: T) -> Self { Self(inner) }

    /// Returns the wrapped value.
    #[inline]
    pub fn into_inner(self) -> T {
        self.0
    }

    /// Returns a reference to the wrapped value.
    #[inline]
    pub fn inner(&self) -> &T {
        &self.0
    }

    /// Returns a mutable reference to the wrapped value.
    #[inline]
    pub fn inner_mut(&mut self) -> &mut T {
        &mut self.0
    }

    /// Wraps a mutable reference to IO type.
    #[inline]
    pub fn new_mut(inner: &mut T) -> &mut Self {
        // SAFETY: the type is repr(transparent) and the lifetimes match
        unsafe { &mut *(inner as *mut _ as *mut Self) }
    }

    /// Wraps a boxed IO type.
    #[cfg(feature = "alloc")]
    #[inline]
    pub fn new_boxed(inner: Box<T>) -> Box<Self> {
        // SAFETY: the type is repr(transparent) and the pointer is created from Box
        unsafe { Box::from_raw(Box::into_raw(inner) as *mut Self) }
    }
}

impl<T: super::Read> std::io::Read for ToStd<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        self.0.read(buf).map_err(Into::into)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> std::io::Result<()> {
        self.0.read_exact(buf).map_err(Into::into)
    }
}

impl<T: super::BufRead> std::io::BufRead for ToStd<T> {
    #[inline]
    fn fill_buf(&mut self) -> std::io::Result<&[u8]> {
        self.0.fill_buf().map_err(Into::into)
    }

    #[inline]
    fn consume(&mut self, amount: usize) {
        self.0.consume(amount)
    }
}

impl<T: super::Write> std::io::Write for ToStd<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.write(buf).map_err(Into::into)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        self.0.flush().map_err(Into::into)
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.0.write_all(buf).map_err(Into::into)
    }
}

// We also impl our traits so that mixing the calls is not annoying.

impl<T: super::Read> super::Read for ToStd<T> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> {
        self.0.read(buf)
    }

    #[inline]
    fn read_exact(&mut self, buf: &mut [u8]) -> super::Result<()> {
        self.0.read_exact(buf)
    }
}

impl<T: super::BufRead> super::BufRead for ToStd<T> {
    #[inline]
    fn fill_buf(&mut self) -> super::Result<&[u8]> {
        self.0.fill_buf()
    }

    #[inline]
    fn consume(&mut self, amount: usize) {
        self.0.consume(amount)
    }
}

impl<T: super::Write> super::Write for ToStd<T> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> super::Result<usize> {
        self.0.write(buf)
    }

    #[inline]
    fn flush(&mut self) -> super::Result<()> {
        self.0.flush()
    }

    #[inline]
    fn write_all(&mut self, buf: &[u8]) -> super::Result<()> {
        self.0.write_all(buf)
    }
}

impl<R: std::io::Read> super::Read for std::io::BufReader<R> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> super::Result<usize> { Ok(std::io::Read::read(self, buf)?) }
}

impl<R: std::io::Read> super::BufRead for std::io::BufReader<R> {
    #[inline]
    fn fill_buf(&mut self) -> super::Result<&[u8]> { Ok(std::io::BufRead::fill_buf(self)?) }

    #[inline]
    fn consume(&mut self, amount: usize) { std::io::BufRead::consume(self, amount) }
}

impl std::io::Write for super::Sink {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> { Ok(buf.len()) }

    #[inline]
    fn write_all(&mut self, _: &[u8]) -> std::io::Result<()> { Ok(()) }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> { Ok(()) }
}

impl<W: std::io::Write> super::Write for std::io::BufWriter<W> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> super::Result<usize> { Ok(std::io::Write::write(self, buf)?) }

    #[inline]
    fn flush(&mut self) -> super::Result<()> { Ok(std::io::Write::flush(self)?) }
}
