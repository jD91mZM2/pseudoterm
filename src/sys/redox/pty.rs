use ::OpenptyOptions;
use super::*;

use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io,
    os::unix::{
        ffi::OsStrExt,
        fs::OpenOptionsExt,
        io::{AsRawFd, RawFd}
    }
};

pub fn isatty(fd: RawFd) -> bool {
    if let Ok(termios) = syscall::dup(fd, b"termios") {
        let _ = syscall::close(termios);
        return true;
    }
    false
}

pub fn openpty(options: &OpenptyOptions) -> io::Result<(File, File)> {
    // Open master
    let mut openopts = OpenOptions::new();
    if options.nonblock {
        openopts.custom_flags(syscall::O_NONBLOCK as i32);
    }
    let master = openopts.read(true).write(true).open("pty:")?;

    // Open slave
    let mut path = [0; 128];
    let len = e(syscall::fpath(master.as_raw_fd(), &mut path))?;
    let path: &OsStr = OsStr::from_bytes(&path[..len]);
    let slave = OpenOptions::new().read(true).write(true).open(path)?;

    Ok((master, slave))
}
pub fn before_exec() -> io::Result<()> {
    Ok(())
}
