extern crate redox_termios;
extern crate syscall;

use std::io;

fn e<T>(result: syscall::Result<T>) -> io::Result<T> {
    result.map_err(|err| io::Error::from_raw_os_error(err.errno))
}

mod pty;
mod termios;
mod winsize;

pub use self::pty::*;
pub use self::termios::*;
pub use self::winsize::*;
