use super::*;

use std::{
    io,
    mem,
    os::unix::io::RawFd
};

pub type Termios = libc::termios;

pub struct TermiosSetter(RawFd);
impl TermiosSetter {
    pub fn new(fd: RawFd) -> io::Result<Self> {
        Ok(TermiosSetter(fd))
    }
    pub fn get(&mut self) -> io::Result<Termios> {
        unsafe {
            // No need to worry about uninitialized memory leaking: Type is Copy
            // and can't have a destructor.
            let mut termios: libc::termios = mem::uninitialized();
            e(libc::tcgetattr(self.0, &mut termios))?;
            Ok(termios)
        }
    }
    pub fn set(&mut self, termios: &Termios) -> io::Result<()> {
        unsafe { e(libc::tcsetattr(self.0, 0, termios)).map(|_| ()) }
    }
}

pub fn make_raw(termios: &mut Termios) {
    unsafe { libc::cfmakeraw(termios); }
}
