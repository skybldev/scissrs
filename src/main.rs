use std::{io, thread, time};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread::JoinHandle;

use clap::{Arg, Command};

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
            .help("Waits MS milliseconds between every scroll (requires -s)")
    ];

    let matches = Command::new("scissrs")
        .author("skybldev - https://skybldev.eu.org")
        .version("0.1.0")
        .about("A string truncator and scroller.")
        .args(args)
        .get_matches();

    let maxlen: usize = matches
        .value_of("max")
        .unwrap()
        .parse()
        .unwrap();

    let tail: String = String::from(matches
        .value_of("tail")
        .unwrap());

    let listen_enabled = matches.is_present("listen");
    let scroll_enabled = matches.is_present("scroll");
    let forever_enabled = matches.is_present("forever");

    let scroll_int: u64 = matches
        .value_of("interval")
        .unwrap()
        .parse()
        .unwrap();

    let stdin = io::stdin();
    let mut buf = String::new();

    if !listen_enabled {
        match stdin.read_line(&mut buf) {
            Ok(_) => {
                if buf.chars().count() > maxlen {
                    println!(
                        "{}{}",
                        trunc_to_char_boundary(buf.trim_end(), maxlen),
                        tail
                    );
                } else {
                    println!("{}", buf.trim_end());
                }
            },
            Err(e) => println!("error: {}", e)
        }
    } else if scroll_enabled {
        let thread_kill = Arc::new(AtomicBool::new(false));
        let mut handle: Option<JoinHandle<()>> = Option::None;

        loop {
            match stdin.read_line(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 {
                        if forever_enabled { loop {} }
                        else { return }
                    }
                    let buf = String::from(buf.trim_end());

                    if buf.chars().count() > maxlen {
                        // wait for thread to stop
                        match handle {
                            Some(h) => {
                                thread_kill.store(true, Ordering::Relaxed);
                                h.join().unwrap();
                                handle = None;
                            },
                            None => {}
                        }
                        
                        // start a new thread
                        thread_kill.store(false, Ordering::Relaxed);
                        let string = buf.clone();
                        let kill = thread_kill.clone();
                        let tail = tail.clone();
                        handle = Some(thread::spawn(move || {
                            scroll_thread(
                                string,
                                maxlen,
                                scroll_int,
                                tail,
                                kill
                            );
                        }));
                    } else {
                        thread_kill.store(true, Ordering::Relaxed);
                        match handle {
                            Some(h) => {
                                h.join().unwrap();
                                handle = None;
                            },
                            None => {}
                        };
                        println!("{}", buf.trim_end());
                    }
                },
                Err(e) => eprintln!("error: {}", e)
            }
            buf.clear();
        }
    } else {
        loop {
            match stdin.read_line(&mut buf) {
                Ok(bytes) => {
                    if bytes == 0 { return }
                    if buf.chars().count() > maxlen {
                        println!(
                            "{}{}",
                            trunc_to_char_boundary(buf.trim_end(), maxlen),
                            tail
                        );
                    } else {
                        println!("{}", buf.trim_end());
                    }
                },
                Err(e) => eprintln!("error: {}", e)
            }
            buf.clear();
        }
    }
}

fn scroll_thread(
    buf: String,
    maxlen: usize,
    scroll_int: u64,
    tail: String,
    kill: Arc<AtomicBool>
) {
    let mut slice = format!("{} «» ", buf.trim());

    println!(
        "{}{}",
        trunc_to_char_boundary(&slice, maxlen),
        tail
    );

    loop {
        if kill.load(Ordering::Relaxed) { return () };

        let mut temp_slice = String::new();

        let mut chars = slice.chars();
        let first_char = chars.next();
        let mut current_char = chars.next();

        while current_char.is_some() {
            temp_slice.push(current_char.unwrap());
            current_char = chars.next();
        }

        temp_slice.push(first_char.unwrap());

        println!(
            "{}{}",
            trunc_to_char_boundary(&temp_slice, maxlen),
            tail
        );

        slice = temp_slice.clone();

        thread::sleep(time::Duration::from_millis(scroll_int));
    }
}

fn trunc_to_char_boundary<T: AsRef<str>>(string: T, max: usize) -> String {
    let string = string.as_ref();
    let mut chars = string.chars();
    let first_char = match chars.next() {
        Some(c) => c,
        None => { return String::from(""); }
    };

    let mut graphemes: Vec<Vec<char>> = Vec::new();
    let mut current_grapheme: Vec<char> = vec![first_char];

    for (idx, c) in chars.enumerate() {
        if string.is_char_boundary(idx) {
            graphemes.push(current_grapheme);
            current_grapheme = vec![c];
        } else {
            current_grapheme.push(c);
        }
    }

    graphemes.concat().into_iter().take(max).collect::<String>()
}
