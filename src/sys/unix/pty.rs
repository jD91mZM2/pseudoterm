use ::OpenptyOptions;
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

pub fn openpty(options: &OpenptyOptions) -> io::Result<(File, File)> {
    unsafe {
        let maybe_nonblock = if options.nonblock { libc::O_NONBLOCK } else { 0 };
        // Open master
        let master = e(libc::posix_openpt(libc::O_RDWR | maybe_nonblock))?;
        let master = File::from_raw_fd(master);
        e(libc::grantpt(master.as_raw_fd()))?;
        e(libc::unlockpt(master.as_raw_fd()))?;

        // Open slave
        // there's no length parameter to the TIOCPTYGNAME call on mac, instead it just assumes the
        // buffer is 128 bytes
        let mut name = [0u8; 128];
        #[cfg(not(target_os = "macos"))]
        e(libc::ptsname_r(master.as_raw_fd(), name.as_mut_ptr() as *mut c_char, name.len()))?;
        #[cfg(target_os = "macos")]
        e(libc::ioctl(master.as_raw_fd(), libc::TIOCPTYGNAME as u64, name.as_mut_ptr() as *mut c_char))?;

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
    unsafe {
        // Make process group leader
        e(libc::setsid())?;

        // Make STDIN (which is our slave) the controlling terminal
        e(libc::ioctl(0, libc::TIOCSCTTY, 1))?;
    }
    Ok(())
}
