#![allow(unused)]

use std::io;

fn main() {
    //Vector - Similar to an array
    //Slower than arrays, but more flexible

    let mut vec1 = Vec::new();
    let mut vec2 = vec![1, 2, 3];

    vec1.push(1);
    vec2.push(4);

    let second_element = vec2[1];
    println!("The second element is {}", second_element);
    println!("The length of the vector is {}", vec2.len());

    for element in vec2.iter() {
        println!("Element: {}", element);
    }
}