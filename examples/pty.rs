extern crate pseudoterm;

use pseudoterm::{RawFile, Winsize};
use std::{
    env,
    io::{self, prelude::*},
    process::Command,
    thread
};

fn main() -> io::Result<()> {
    let mut args = env::args().skip(1);

    let cmd = match args.next() {
        Some(cmd) => cmd,
        None => {
            eprintln!("Usage: pty <cmd>");
            return Ok(());
        }
    };

    // Step 1: Create PTY
    let (mut master, slave) = pseudoterm::openpty(Some(Winsize {
        rows: 32,
        cols: 80
    }))?;

    // Step 2: Launch command
    let mut child = pseudoterm::prepare_cmd(slave, &mut Command::new(cmd))?
        .args(args)
        .spawn()?;

    // Step 3: Forward I/O
    let mut clone = master.try_clone()?;
    thread::spawn(move || {
        let stdin = io::stdin();
        let mut stdin = stdin.lock();

        let mut buf = [0; 1024];
        loop {
            let res = match stdin.read(&mut buf) {
                Ok(0) => break,
                Ok(n) => clone.write_all(&buf[..n]),
                Err(err) => Err(err)
            };
            if let Err(err) = res {
                eprintln!("{}", err);
                return;
            }
        }
    });

    let stdout = io::stdout();
    //let stdout = stdout.lock();
    let mut stdout = RawFile::new_allow_failure(stdout);

    let mut buf = [0; 1024];
    loop {
        match master.read(&mut buf) {
            Ok(0) | Err(_) => break,
            Ok(n) => {
                stdout.write_all(&buf[..n])?;
                stdout.flush()?;
            }
        }
    }

    // Step 4: Wait until the process is completely dead
    child.wait()?;

    Ok(())
}
