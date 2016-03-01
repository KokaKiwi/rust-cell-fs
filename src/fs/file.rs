use std::io;
use std::marker::PhantomData;

pub use std::io::{Read, Write};
pub trait ReadWrite: Read + Write {}

pub struct File<'a, Inner: 'a + ?Sized> {
    inner: Box<Inner>,
    _phantom: PhantomData<&'a Inner>,
}

impl<'a, Inner: ?Sized> File<'a, Inner> {
    pub fn new(inner: Box<Inner>) -> File<'a, Inner> {
        File {
            inner: inner,
            _phantom: PhantomData,
        }
    }
}

// Macros
macro_rules! impl_read {
    ($inner:ty) => {
        impl<'a> Read for File<'a, $inner> {
            fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
                self.inner.read(buf)
            }
        }
    }
}

macro_rules! impl_write {
    ($inner:ty) => {
        impl<'a> Write for File<'a, $inner> {
            fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
                self.inner.write(buf)
            }

            fn flush(&mut self) -> io::Result<()> {
                self.inner.flush()
            }
        }
    }
}

impl_read!(Read);
impl_read!(ReadWrite);

impl_write!(Write);
impl_write!(ReadWrite);
