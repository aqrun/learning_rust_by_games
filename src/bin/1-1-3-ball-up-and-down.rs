extern crate aqrun_lrbg;

use std::time::Duration;
use std::thread;
use aqrun_lrbg::common;

/// ball move up and down
pub fn main() {
    let x = 10;
    let mut y = 5;
    let height = 10;
    let mut velocity = 1;

    let duration = Duration::from_millis(100);

    loop {
        common::clear();
        y += velocity;

        for _ in 0..y {
            println!("");
        }

        for _ in 0..x {
            print!(" ");
        }

        println!("o");

        if y <= 0 {
            velocity = -velocity;
        }
        if y >= height {
            velocity = -velocity;
        }

        thread::sleep(duration);
    }
}