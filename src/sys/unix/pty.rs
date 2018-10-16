use super::{*, libc::c_char};

use std::{
    ffi::OsStr,
    fs::{File, OpenOptions},
    io,
    os::unix::{
        ffi::OsStrExt,
        io::{AsRawFd, FromRawFd, RawFd}
    },
};

pub fn isatty(fd: RawFd) -> bool {
    unsafe { libc::isatty(fd) != 0 }
}

pub fn openpty() -> io::Result<(File, File)> {
    unsafe {
        let master = e(libc::posix_openpt(libc::O_RDWR))?;
        let master = File::from_raw_fd(master);
        e(libc::grantpt(master.as_raw_fd()))?;
        e(libc::unlockpt(master.as_raw_fd()))?;

        let mut name = [0u8; 32];
        e(libc::ptsname_r(master.as_raw_fd(), name.as_mut_ptr() as *mut c_char, name.len()))?;

        let mut len = 0;
        while len < name.len() && name[len] != 0 {
            len += 1;
        }

        let buf: &OsStr = OsStr::from_bytes(&name[..len]);
        let slave = OpenOptions::new().read(true).write(true).open(buf)?;

        Ok((master, slave))
    }
}
pub fn before_exec() -> io::Result<()> {
    unsafe { e(libc::setsid()).map(|_| ()) }
}
