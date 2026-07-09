#![allow(unused)]

use std::io;

fn main() {
    //Structures

    struct Car {
        make: String,
        model: String,
        year: u32,
        price: f64,
    }

    let mut huracan = Car {
        make: String::from("Lamborghini"),
        model: String::from("Huracan"),
        year: 2020,
        price: 320000.0
    };

    println!("The cost of a {} {} {} is ${}.", huracan.year, huracan.make, huracan.model, huracan.price);


    struct Rectangle {
        width: u32,
        height: u32,
    }

    impl Rectangle {
        fn area(&self) -> u32 {
            self.width * self.height
        }
    }

    let rect= Rectangle { width: 30, height: 50 };

    let area = rect.area();

    println!("The area of a rectangle is {}", area);

}