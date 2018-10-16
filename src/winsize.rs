use super::*;
use std::{io, os::unix::io::AsRawFd};

/// How big a terminal window is
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, Hash)]
pub struct Winsize {
    pub rows: u16,
    pub cols: u16
}

/// A way to get/set the window size. On some operating systems this struct
/// can keep internal data to avoid redundant system calls.
pub struct WinsizeSetter(sys::WinsizeSetter);
impl WinsizeSetter {
    /// Create a new window size getter/setter
    pub fn new<F: AsRawFd>(fd: &F) -> io::Result<Self> {
        sys::WinsizeSetter::new(fd.as_raw_fd()).map(WinsizeSetter)
    }

    /// Get the window size
    pub fn get(&mut self) -> io::Result<Winsize> {
        self.0.get()
    }
    /// Set the window size
    pub fn set(&mut self, winsize: Winsize) -> io::Result<()> {
        self.0.set(winsize)
    }
}

/// Get the window size for the specified file. This is a shortcut for:
/// ```rust
/// WinsizeSetter::new(fd)?.get()
/// ```
pub fn get_size<F: AsRawFd>(fd: &F) -> io::Result<Winsize> {
    WinsizeSetter::new(fd).and_then(|mut setter| setter.get())
}
