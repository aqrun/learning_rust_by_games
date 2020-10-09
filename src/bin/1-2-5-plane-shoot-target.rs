extern crate aqrun_lrbg;

use aqrun_lrbg::common;
use crossterm::event::{
    self, Event, KeyCode, KeyEvent
};

/// plane can shoot cross target
pub fn main() {
    let mut x = 10;
    let mut y = 5;
    let mut is_fire = false;

    let nx = 8;
    let mut is_killed = false;

    loop {
        common::clear();

        if !is_killed {
            for _ in 0..nx {
                print!(" ");
            }
            println!("+");
        }

        if !is_fire {
            for _ in 0..y {
                println!("");
            }
        } else {
            for _ in 0..y {
                for _ in 0..x {
                    print!(" ");
                }
                println!("  |");
            }

            if x + 2 == nx {
                is_killed = true;
            }

            is_fire = false;
        }

        for _ in 0..x {
            print!(" ");
        }
        println!("  *");
        for _ in 0..x {
            print!(" ");
        }
        println!("*****");
        for _ in 0..x {
            print!(" ");
        }
        println!(" * * ");

        if let Event::Key(KeyEvent{ code, .. }) = event::read().unwrap() {
            match code {
                KeyCode::Char('a') => x -= 1,
                KeyCode::Char('w') => y -= 1,
                KeyCode::Char('s') => y += 1,
                KeyCode::Char('d') => x += 1,
                KeyCode::Char(' ') => is_fire = true,
                _ => {}
            }
        }
    }
}