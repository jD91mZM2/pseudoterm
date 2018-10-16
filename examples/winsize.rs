extern crate pseudoterm;

use std::io;

fn main() -> io::Result<()> {
    let size = pseudoterm::get_size(&io::stdout())?;
    println!("Your terminal size is {}x{}", size.cols, size.rows);
    Ok(())
}
