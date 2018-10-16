use super::*;

use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, RawFd}
    },
};

pub fn isatty(fd: RawFd) -> bool {
    if let Ok(termios) = syscall::dup(fd, b"termios") {
        let _ = syscall::close(termios);
        return true;
    }
    false
}

pub fn openpty() -> io::Result<(File, File)> {
    // Open master
    let master = File::create("pty:")?;

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
