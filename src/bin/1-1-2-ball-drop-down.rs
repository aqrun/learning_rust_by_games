extern crate aqrun_lrbg;

use std::thread::sleep;
use std::time::Duration;
use aqrun_lrbg::common;

/// drop down ball
pub fn main() {
    let x = 10;
    let y = 5;
    let height = 20;
    let duration = Duration::from_millis(100);

    for i in y..height {
        common::clear();

        for _ in 0..i {
            println!("")
        }

        for _ in 0..x {
            print!(" ");
        }

        println!("o");

        sleep(duration);
    }
}