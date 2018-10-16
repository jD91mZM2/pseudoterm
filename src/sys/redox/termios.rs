use super::*;

use std::{
    fs::File,
    io::{self, prelude::*},
    os::unix::io::{FromRawFd, RawFd}
};

pub type Termios = redox_termios::Termios;

pub struct TermiosSetter(File);
impl TermiosSetter {
    pub fn new(fd: RawFd) -> io::Result<Self> {
        let termios = e(syscall::dup(fd, b"termios"))?;
        Ok(TermiosSetter(unsafe { File::from_raw_fd(termios) }))
    }
    pub fn get(&mut self) -> io::Result<Termios> {
        let mut termios = redox_termios::Termios::default();
        self.0.read(&mut termios)?;
        Ok(termios)
    }
    pub fn set(&mut self, termios: &Termios) -> io::Result<()> {
        self.0.write(&termios).map(|_| ())
    }
}

pub fn make_raw(termios: &mut Termios) {
    termios.make_raw()
}
