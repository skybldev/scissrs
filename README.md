# scissrs

A string truncator and scroller written in Rust.

## Usage

`scissrs --help` covers the definitions of this program's flags.

By default, when you run `scissrs`, it will wait for input from STDIN, and quit when it receives it. You can make it listen similarly to `cut -c1-${}` using `-l`.

This is also useful in Polybar with xmonad window names or something similar. In fact, that was the original purpose for this. You could run it like this, the fancy way:

```
tail -f /tmp/.xmonad-title-log 2> /dev/null | scissrs -l -s -i 100
```

(the redirection to /dev/null (`2> /dev/null`) is necessary to hide "file truncated" messages)

...which will result in something like this (enlarged to show texture):

![demo](./repo/demo.gif)

(font: Iosevka Fixed SS03)

## Building

Simply run `cargo build --release`. Optionally, you can copy the file to a directory in your PATH, so it can be used like any other command.
