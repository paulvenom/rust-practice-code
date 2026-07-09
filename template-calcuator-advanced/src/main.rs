#![allow(unused)]

use std::io;

fn main() {
    //Opening lines
    println!("Heath's Rust Calculator");
    println!("You must select two values (x and y) and an operator.");

    //Receive a value for X
    println!("Please give me a value for X.");

    let mut x = String::new();
    io::stdin().read_line(&mut x);
    let x: i32 = x.trim().parse().expect("Entry was not an integer.");
    let float_x = x as f64;

    //Receive a value for Y
    println!("Please give me a value for Y.");

    let mut y = String::new();
    io::stdin().read_line(&mut y);
    let y: i32 = y.trim().parse().expect("Entry was not an integer.");
    let float_y = y as f64;

    //Receive an operator
    println!("Choose an operator: +, -, *, /");
    let mut operator = String::new();
    io::stdin().read_line(&mut operator);
    let operator_slice = operator.trim();

    //Match operator
    match operator_slice {
        "+" => {
            add(x,y);
        }
        "-" => {
            subtract(x,y);
        }
        "*" => {
            multiply(x,y);
        }
        "/" => {
            divide(float_x,float_y);
        }
        &_ => {
            println!("Invalid entry. Exiting program.");
        }
    }


}

//Math functions
fn add(x: i32, y:i32) {
    println!("The result of {} + {} = {}",x,y,x+y);
}
fn subtract(x: i32, y:i32) {
    println!("The result of {} - {} = {}",x,y,x-y);
}
fn multiply(x: i32, y:i32) {
    println!("The result of {} * {} = {}",x,y,x*y);
}
fn divide(x: f64, y:f64) {
    println!("The result of {} / {} = {}",x,y,x/y);
}