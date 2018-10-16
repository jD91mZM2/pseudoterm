use super::*;
use std::{
    fs::File,
    io,
    os::unix::{
        io::AsRawFd,
        process::CommandExt
    },
    process::{Command, Stdio}
};

/// Returns true if the file is a TTY/PTY. This usually means the program is
/// ran interactively and not for example piped to another program.
pub fn isatty<F: AsRawFd>(fd: &F) -> bool {
    sys::isatty(fd.as_raw_fd())
}
/// Open a PTY master and slave. Optionally resized to the specified size
pub fn openpty(size: Option<Winsize>) -> io::Result<(File, File)> {
    let (master, slave) = sys::openpty()?;
    if let Some(size) = size {
        WinsizeSetter::new(&master)?.set(size)?;
    }
    Ok((master, slave))
}
/// Prepare a `Command` with a slave as stdin/stdout/stderr
pub fn prepare_cmd<'a>(slave: File, command: &'a mut Command) -> io::Result<&'a mut Command> {
    let stdin = slave.try_clone()?;
    let stdout = slave.try_clone()?;
    let stderr = slave;

    Ok(
        command
            .stdin(Stdio::from(stdin))
            .stdout(Stdio::from(stdout))
            .stderr(Stdio::from(stderr))
            .before_exec(sys::before_exec)
    )
}
