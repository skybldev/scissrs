use std::io;
use std::time::Duration;

use clap::{Arg, Command};
use unicode_segmentation::UnicodeSegmentation;

mod scroll;
use scroll::scroll;

fn main() {
    let args = [
        Arg::new("max")
            .value_name("MAX")
            .short('x')
            .takes_value(true)
            .default_value("80")
            .help("Truncate to MAX characters"),
        Arg::new("tail")
            .value_name("TAIL")
            .short('t')
            .takes_value(true)
            .default_value("...")
            .help("Adds TAIL to the end of a truncated string"),
        Arg::new("listen")
            .short('l')
            .takes_value(false)
            .required(false)
            .help("If enabled, this will constantly listen to STDIN until it is manually stopped"),
        Arg::new("scroll")
            .short('s')
            .takes_value(false)
            .required(false)
            .requires("listen")
            .help("Enables scrolling through truncated text (requires -l)"),
        Arg::new("forever")
            .short('f')
            .takes_value(false)
            .required(false)
            .requires("scroll")
            .help("Scroll forever even after EOF"),
        Arg::new("interval")
            .value_name("MS")
            .short('i')
            .takes_value(true)
            .default_value("1000")
            .requires("scroll")
            .help("Waits MS milliseconds between every scroll (requires -s)"),
    ];

    let matches = Command::new("scissrs")
        .author("skybldev - https://skybldev.eu.org")
        .version("0.1.0")
        .about("A string truncator and scroller.")
        .args(args)
        .get_matches();

    let maxlen: usize = matches.value_of("max").unwrap().parse().unwrap();

    let tail: String = String::from(matches.value_of("tail").unwrap());

    let listen_enabled = matches.is_present("listen");
    let scroll_enabled = matches.is_present("scroll");
    let forever_enabled = matches.is_present("forever");

    let scroll_int: u64 = matches.value_of("interval").unwrap().parse().unwrap();

    if !listen_enabled {
        match io::stdin().lines().next() {
            Some(Ok(line)) => show_ordinary(&line, maxlen, &tail),
            Some(Err(e)) => panic!("Got an error: {}", e),
            None => {}
        }
    } else if scroll_enabled {
        scroll(maxlen, forever_enabled, Duration::from_millis(scroll_int), &tail, " «» ");
    } else {
        let mut lines = io::stdin().lines();
        loop {
            match lines.next() {
                Some(Ok(line)) => show_ordinary(&line, maxlen, &tail),
                Some(Err(e)) => panic!("Got an error: {}", e),
                None => return, // EOF
            }
        }
    }
}

#[inline]
fn show_ordinary(line: &str, maxlen: usize, tail: &str) {
    let graphemes = line.grapheme_indices(true).collect::<Vec<(usize, &str)>>();
    if graphemes.len() > maxlen {
        println!("{}{}", &line[..graphemes.get(maxlen).unwrap().0], tail)
    } else {
        println!("{}", line)
    }
}

