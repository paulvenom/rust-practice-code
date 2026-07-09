#![allow(unused)]

use std::io;
use std::cmp::Ordering;

fn main() {
    //Match - matching arm & all possible values must be covered


    let drinking_age = 21;

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let my_age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    match my_age.cmp(&drinking_age) {
        Ordering::Less => println!("Cannot drink!"),
        Ordering::Equal => println!("Woo, you can drink!"),
        Ordering::Greater => println!("Can drink!"),
    };

}