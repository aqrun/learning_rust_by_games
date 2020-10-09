extern crate aqrun_lrbg;

use aqrun_lrbg::common;

/// show a static ball
pub fn main(){
    let x = 10;
    let y = 5;

    common::clear();

    for _ in 0..y {
        println!("");
    }

    for _ in 0..x {
        print!(" ")
    }

    println!("o");
}