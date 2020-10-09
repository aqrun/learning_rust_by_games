extern crate aqrun_lrbg;

use aqrun_lrbg::common;
use crossterm::{
    event::{self, Event, KeyCode, KeyEvent}
};

/// move plane by keyboard
pub fn main() {
    let mut x = 10;
    let mut y = 5;

    loop {
        common::clear();

        for _ in 0..y {
            println!("");
        }

        for _ in 0..x {
            print!(" ");
        }
        println!("*");

        if let Event::Key(KeyEvent{ code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('a') => x -= 1,
                KeyCode::Char('w') => y -= 1,
                KeyCode::Char('s') => y += 1,
                KeyCode::Char('d') => x += 1,
                _ => {}
            }
        }
    }
}