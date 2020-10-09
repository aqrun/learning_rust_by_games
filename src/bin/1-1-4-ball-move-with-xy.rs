extern crate aqrun_lrbg;

use std::thread;
use std::time::Duration;
use aqrun_lrbg::common;

/// ball move with x and y
pub fn main() {
    let mut x = 10;
    let mut y = 5;
    let width = 20;
    let height = 10;

    let mut velocity_x = 1;
    let mut velocity_y = 1;

    let duration = Duration::from_millis(100);

    loop {
        common::clear();

        x += velocity_x;
        y += velocity_y;

        for _ in 0..y {
            println!("");
        }

        for _ in 0..x {
            print!(" ");
        }

        println!("o");

        if x <= 0 || x >= width {
            velocity_x = -velocity_x;
        }
        if y <=0 || y >= height {
            velocity_y = -velocity_y;
        }

        thread::sleep(duration);
    }
}