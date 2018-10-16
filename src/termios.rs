use super::*;
use std::{
    io,
    ops::{Deref, DerefMut},
    os::unix::io::AsRawFd
};

/// A cross-platform wrapper around underlying the termios representation
#[derive(Clone, Copy)]
pub struct Termios(sys::Termios);
impl Termios {
    /// Get the inner termios representation.
    /// WARNING: Platform dependant.
    pub fn inner(&self) -> &sys::Termios {
        &self.0
    }

    /// Return the raw mode of this representation
    pub fn make_raw(mut self) -> Self {
        sys::make_raw(&mut self.0);
        self
    }
}

/// A way to get/set the termios flags. On some operating systems this struct
/// can keep internal data to avoid redundant system calls.
pub struct TermiosSetter(sys::TermiosSetter);
impl TermiosSetter {
    /// Create a new termios getter/setter
    pub fn new<F: AsRawFd>(fd: &F) -> io::Result<Self> {
        sys::TermiosSetter::new(fd.as_raw_fd()).map(TermiosSetter)
    }

    /// Get the termios data
    pub fn get(&mut self) -> io::Result<Termios> {
        self.0.get().map(Termios)
    }
    /// Set the termios data
    pub fn set(&mut self, termios: &Termios) -> io::Result<()> {
        self.0.set(termios.inner())
    }
}

/// A structure that will automatically reset terminal mode when dropped
pub struct RawFile<F: AsRawFd> {
    file: F,
    restore: Option<(TermiosSetter, Termios)>
}
impl<F: AsRawFd> RawFile<F> {
    /// Switch the terminal to raw mode and return a wrapper that will exit raw
    /// mode automatically when dropped
    pub fn new(file: F) -> io::Result<Self> {
        let mut setter = TermiosSetter::new(&file)?;
        let prev = setter.get()?;
        setter.set(&prev.make_raw())?;

        Ok(Self {
            file,
            restore: Some((setter, prev))
        })
    }
    /// Like new(), but ignores any failure. Useful for switching to raw mode
    /// only if that's actually possible.
    pub fn new_allow_failure(file: F) -> Self {
        let mut restore = None;
        if let Ok(mut setter) = TermiosSetter::new(&file) {
            if let Ok(prev) = setter.get() {
                if setter.set(&prev.make_raw()).is_ok() {
                    restore = Some((setter, prev));
                }
            }
        }

        Self { file, restore }
    }
}
impl<F: AsRawFd> Deref for RawFile<F> {
    type Target = F;

    fn deref(&self) -> &Self::Target {
        &self.file
    }
}
impl<F: AsRawFd> DerefMut for RawFile<F> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.file
    }
}
impl<F: AsRawFd> Drop for RawFile<F> {
    fn drop(&mut self) {
        if let Some((ref mut setter, ref prev)) = self.restore {
            let _ = setter.set(prev);
        }
    }
}
