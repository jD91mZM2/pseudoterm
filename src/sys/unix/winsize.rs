use ::Winsize;
use super::*;

use std::{
    io,
    mem,
    os::unix::io::RawFd
};

pub struct WinsizeSetter(RawFd);
impl WinsizeSetter {
    pub fn new(fd: RawFd) -> io::Result<Self> {
        Ok(WinsizeSetter(fd))
    }
    pub fn get(&mut self) -> io::Result<Winsize> {
        unsafe {
            // No need to worry about uninitialized memory leaking: Type is Copy
            // and can't have a destructor.
            let mut winsize: libc::winsize = mem::uninitialized();
            e(libc::ioctl(self.0, libc::TIOCGWINSZ, &mut winsize))?;

            Ok(Winsize {
                rows: winsize.ws_row,
                cols: winsize.ws_col,
            })
        }
    }
    pub fn set(&mut self, winsize: Winsize) -> io::Result<()> {
        unsafe {
            let winsize = libc::winsize {
                ws_row: winsize.rows,
                ws_col: winsize.cols,
                ws_xpixel: 0,
                ws_ypixel: 0
            };
            e(libc::ioctl(self.0, libc::TIOCSWINSZ, &winsize))?;
            Ok(())
        }
    }
}
