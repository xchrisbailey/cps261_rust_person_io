use serde::{Deserialize, Serialize};
use std::io::*;

const PERSONDAT: &str = "person.dat";

#[derive(Serialize, Deserialize, PartialEq, Debug)]
pub struct Person {
    name: String,
    age: i32,
}

impl Person {
    pub fn new(name: String, age: i32) -> Self {
        Self { name, age }
    }
}

fn main() {
    run();
}

fn run() {
    let mut input: String = String::new();
    loop {
        input.clear();

        println!("~~~~~~~~~~~~~~~~");
        println!("1: add person");
        println!("2: display persons");
        println!("0: quit");
        println!("~~~~~~~~~~~~~~~~");

        stdout().flush().expect("Unable to clear stdout");
        stdin()
            .read_line(&mut input)
            .expect("Did not enter a valid string");

        match input.trim().parse::<i32>().unwrap() {
            0 => break,
            1 => add_person(),
            2 => display_persons(),
            _ => continue,
        }
    }
}

fn add_person() {
    let mut name = String::new();
    let mut age = String::new();

    print!("Enter persons name: ");
    stdout().flush().expect("Unable to clear stdout");
    stdin()
        .read_line(&mut name)
        .expect("Did not enter valid name");

    print!("Enter persons age: ");
    stdout().flush().expect("Unable to clear stdout");
    stdin()
        .read_line(&mut age)
        .expect("Did not enter valid age");

    let p = Person::new(name.trim().to_string(), age.trim().parse::<i32>().unwrap());

    println!("{:?}", p);
}

fn display_persons() {
    println!("display persons here");
}
