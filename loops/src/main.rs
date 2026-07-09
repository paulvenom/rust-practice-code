#![allow(unused)]

use std::io;
use std::cmp::Ordering;

fn main() {

//LOOPS - While, For, Infinite Loop

    //For loops - start to finish of an iterate

    let mut vegetables = ["Cucumber", "Spinach", "Cabbage"];
    for x in vegetables.iter() {
        println!("{}",x);
    }

    //While loops - execute as long as true

    let mut i = 1;
    while i< 10 {
        println!("{}", i);
        i += 1;
    }

    //loop - infinite loops

    let mut y = 0;

    println!("Counting!");
    loop {
        y += 1;

        if y == 10 {
            println!("We've reached 10!");
            continue;
        }

        if y == 20 {
            println!("Reached 20, exit program!");
            break;
        }
    }


}