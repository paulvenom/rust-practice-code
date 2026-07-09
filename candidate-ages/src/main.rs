#![allow(unused)]

use std::io;

fn main() {
    //Match - matching arm & all possible values must be covered

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let candidacy_age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    match candidacy_age {
        1..=24 => println!("Cannot hold office."),
        25..=29 => println!("Can run for the House."),
        30..=34 => println!("Can run for the Senate."),
        35..=i32::MAX => println!("Can run for President."),
        _ => println!("Are you an infant?")
    };

}