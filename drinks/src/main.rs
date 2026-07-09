#![allow(unused)]

use std::io;

fn main() {
    //if, else if, else

    println!("How much money do you have?");
    let mut input_money = String::new();
    io::stdin().read_line(&mut input_money);

    let money: i32 = input_money.trim().parse().expect("Entry was not an integer");

    println!("How old are you?");
    let mut input_age = String::new();
    io::stdin().read_line(&mut input_age);

    let age: i32 = input_age.trim().parse().expect("Entry was not an integer");

    if (age >= 21) && (money >= 5) {
        println!("We're getting a drink!");
    } else if (age >= 21) && (money < 5) {
        println!("Come back with more money!");
    } else if (age < 21) && (money >= 5) {
        println!("Nice try, kid!");
    } else {
        println!("You're too young and too poor.");
    }

}