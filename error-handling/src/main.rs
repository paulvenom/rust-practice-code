#![allow(unused)]

use std::io;
use std::fs::{File, OpenOptions};
use std::io::prelude::*;
use std::fs;


fn main() {
    //HELPER METHODS - unwrap(), expect()
    //Twp ty[es pf errors: recoverable and unrecoverable
    //Recoverable - result enum & option enum
    //Unrecoverable - panic! macro

    //Result enum - Result<T, E>

    //? operator

    let result = read_file("src/test.txt");
    match result {
        Ok(contents) => println!("File contents: {}", contents),
        Err(err) => println!("Error reading file: {}", err),
    }


}

fn read_file(path: &str) -> Result<String, std::io::Error> {
    let mut file = File::open(path)?;

    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}