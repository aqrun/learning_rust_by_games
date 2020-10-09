extern crate aqrun_lrbg;

use aqrun_lrbg::common;
use std::io::stdin;

/// wasd control simple plane
pub fn run() {
    let mut x = 10;
    let mut y = 5;

    loop {
        common::clear();

        let mut input = String::from("");

        for _ in 0..y {
            println!("");
        }
        for _ in 0..x {
            print!(" ");
        }
        println!("*");

        stdin().read_line(&mut input).unwrap();
        let dir = input.trim();

        if dir == "w" {
            y -= 1;
        } else if dir == "s" {
            y += 1;
        } else if dir == "a" {
            x -= 1;
        } else if dir == "d" {
            x += 1;
        }
        
    }
}