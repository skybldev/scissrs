use std::cell::Cell;
use std::io;
use std::sync::mpsc;
use std::thread::{self, sleep};
use std::time::Duration;

use std::slice::Windows;

use unicode_segmentation::UnicodeSegmentation;

#[inline]
pub fn scroll(size: usize, forever_enabled: bool, scroll_int: Duration, tail: &str, separator: &str) {
    // nonblocking stdin provider
    let (send, recv) = mpsc::channel::<String>();
    let cell_prov = thread::Builder::new()
        .name("stdin provider".into())
        .spawn(move || {
            io::stdin()
                .lines()
                .for_each(|s| send.send(s.unwrap()).unwrap());
        }).unwrap();
    let cell_prov: Cell<Option<thread::JoinHandle<_>>> = Cell::new(Some(cell_prov));
    let mut grapheme_window: Option<Windows<'_, &str>> = None;

    loop {
        // hacky way around the borrow checker
        // when provider thread panics, panic too
        match cell_prov.replace(None) {
            None => {}
            Some(prov) => {
                if prov.is_finished() {
                    prov.join().unwrap();
                    if !forever_enabled {
                        return;
                    }
                } else {
                    cell_prov.set(Some(prov))
                }
            }
        }

        if let Ok(new_string) = recv.try_recv() {
            todo!();
        }

        sleep(scroll_int);
    }
}
