# pseudoterm [![Crates.io](https://img.shields.io/crates/v/pseudoterm.svg)](https://crates.io/crates/pseudoterm)

Low-level library for creating PTYs and getting related properties. Aimed to be
cross platform, and so far has Unix and Redox OS support.

## Yet another?

I know there are lots of crates for opening PTYs, but I really wanted to make my own.

Key features of this one:
 - Low-level. You get access to the file handles directly.
 - Comes bundled with related features, like setting the terminal to raw mode.
 - Redox OS support from the start

## Examples

See the `examples/` directory.

## Getter/Setter?????

If you've seen the docs, you'll notice there are stuff like `TermiosSetter` and
`WinsizeSetter`. This is a part of the design that makes Redox OS support
special. In redox, when setting the window size you need to do the following:

 - Open a file that controls the window size
 - Get the window size
 - (Close the file?)
 - (Reopen the file?)
 - Set the window size
 - Close the file

What pseudoterm does is let you get/set without opening/closing the file each
time.
