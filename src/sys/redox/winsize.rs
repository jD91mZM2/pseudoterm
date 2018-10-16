use ::Winsize;
use super::*;

use std::{
    fs::File,
    io::{self, prelude::*},
    os::unix::io::{FromRawFd, RawFd}
};

pub struct WinsizeSetter(File);
impl WinsizeSetter {
    pub fn new(fd: RawFd) -> io::Result<Self> {
        let winsize = e(syscall::dup(fd, b"winsize"))?;
        Ok(WinsizeSetter(unsafe { File::from_raw_fd(winsize) }))
    }
    pub fn get(&mut self) -> io::Result<Winsize> {
        let mut winsize = redox_termios::Winsize::default();
        self.0.read(&mut winsize)?;
        Ok(Winsize {
            rows: winsize.ws_row,
            cols: winsize.ws_col
        })
    }
    pub fn set(&mut self, winsize: Winsize) -> io::Result<()> {
        self.0.write(&redox_termios::Winsize {
            ws_row: winsize.rows,
            ws_col: winsize.cols
        }).map(|_| ())
    }
}
