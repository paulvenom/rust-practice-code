#![allow(unused)]

use std::io;

fn main() {
    //Functions - mini programs, organized blocks of code
    let (added, multiplied) = add_and_multiply(4, 3);
    println!("Added: {}", added);
    println!("Multiplied: {}", multiplied);
}

fn add_and_multiply(x: i32, y: i32) -> (i32, i32) {
    (x+y, x*y)
}

fn subtract_and_divide(x: i32, y: i32) -> (i32, i32) {
    (x-y, x*y)
}