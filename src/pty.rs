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

#[derive(Clone, Debug, Default)]
pub struct OpenptyOptions {
    pub size: Option<Winsize>,
    pub nonblock: bool
}
impl OpenptyOptions {
    /// Same as default()
    pub fn new() -> Self {
        Self::default()
    }
    /// Chainable function to set size
    pub fn with_size(mut self, size: Winsize) -> Self {
        self.size = Some(size);
        self
    }
    /// Chainable function to set the master file to be nonblocking
    pub fn with_nonblocking(mut self, nonblock: bool) -> Self {
        self.nonblock = nonblock;
        self
    }
}

/// Returns true if the file is a TTY/PTY. This usually means the program is
/// ran interactively and not for example piped to another program.
pub fn isatty<F: AsRawFd>(fd: &F) -> bool {
    sys::isatty(fd.as_raw_fd())
}
/// Open a PTY master and slave. Optionally resized to the specified size
pub fn openpty(options: &OpenptyOptions) -> io::Result<(File, File)> {
    let (master, slave) = sys::openpty(&options)?;
    if let Some(size) = options.size {
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
