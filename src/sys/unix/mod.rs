extern crate libc;

use self::libc::c_int;

use std::io;

fn e(result: c_int) -> io::Result<c_int> {
    if result == -1 {
        Err(io::Error::last_os_error())
    } else {
        Ok(result)
    }
}

mod pty;
mod termios;
mod winsize;

pub use self::pty::*;
pub use self::termios::*;
pub use self::winsize::*;
