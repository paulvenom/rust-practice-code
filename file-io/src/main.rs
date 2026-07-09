#![allow(unused)]

use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::fs;


fn main() {
    let mut file = File::create("src/hello.txt").expect("File failed to create.");
    file.write_all(b"Hello, world!").expect("Failed to write to file.");

    let mut file = OpenOptions::new().append(true).open("src/hello.txt").expect("Unable to open file.");
    file.write_all(b"\nHello, again!").expect("Failed to write to file.");

    let mut file = File::open("src/hello.txt").expect("Unable to open file.");
    let mut file_content = String::new();
    file.read_to_string(&mut file_content).unwrap();
    println!("{}", file_content);

    fs::remove_file("src/hello.txt").expect("Unable to delete file.");
}