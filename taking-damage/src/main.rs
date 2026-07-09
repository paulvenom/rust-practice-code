#![allow(unused)]

use std::io;

trait Damage {
    fn damage(self: &mut Self);
}

#[derive(Debug)]

struct HP {
    hp_remaining: i32,
}

impl Damage for HP {
    fn damage(self: &mut Self) {
        self.hp_remaining -= 1;
    }
}

fn main() {
    //Traits
    let mut hp = HP {hp_remaining: 100};
    hp.damage();
    println!("You took a hit! HP Remaining: {:?}", hp);

}