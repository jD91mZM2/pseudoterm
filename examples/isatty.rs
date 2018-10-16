extern crate pseudoterm;

use std::io;

fn main() {
    if pseudoterm::isatty(&io::stdout()) {
        println!("Stdout is a TTY");
    } else {
        println!("Stdout is *not* a TTY");
    }
}
