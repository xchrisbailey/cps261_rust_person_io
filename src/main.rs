use colored::*;
use serde::{Deserialize, Serialize};
use std::fs::OpenOptions;
use std::io::*;
use std::path::Path;

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

    pub fn create() -> Self {
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

        Self {
            name: name.trim().to_string(),
            age: age.trim().parse::<i32>().unwrap(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Persons(Vec<Person>);

impl Persons {
    fn add(&mut self, person: Person) {
        &self.0.push(person);
    }

    fn encode(&self) -> Vec<u8> {
        let encoded: Vec<u8> = bincode::serialize(&self).unwrap();
        encoded
    }
}

fn main() {
    let mut input: String = String::new();
    loop {
        input.clear();

        println!("{}", "~~~~~~~~~~~~~~~~~~~~".color("blue").reversed());
        println!(
            "{} {}\x20\x20\x20\x20\x20 {}",
            "1:".color("blue").reversed(),
            "add person".color("blue"),
            "\x20".color("blue").reversed()
        );
        println!(
            "{} {} {}",
            "2:".color("blue").reversed(),
            "display persons".color("blue"),
            "\x20".color("blue").reversed()
        );
        println!(
            "{} {}\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20\x20{}",
            "0:".color("blue").reversed(),
            "quit".color("blue"),
            "\x20".color("blue").reversed()
        );
        println!("{}", "~~~~~~~~~~~~~~~~~~~~".color("blue").reversed());

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
    let p: Person = Person::create();
    match write_person_to_file(p) {
        Ok(_) => println!("Person added"),
        _ => println!("Could not add person"),
    };
}

fn write_person_to_file(person: Person) -> Result<()> {
    let mut file = create_and_open_file(PERSONDAT);
    let encoded_persons: Vec<u8>;
    if Path::new(PERSONDAT).exists() {
        let persons_bytes = get_persons_bytes_from_file();
        let mut persons: Persons = bincode::deserialize(&persons_bytes[..]).unwrap();
        persons.add(person);
        encoded_persons = persons.encode();
    } else {
        let persons: Persons = Persons(vec![person]);
        encoded_persons = persons.encode();
    }

    file.write_all(&encoded_persons)?;
    Ok(())
}

fn display_persons() {
    let bytes: Vec<u8> = get_persons_bytes_from_file();
    let persons: Vec<Person> = bincode::deserialize(&bytes[..]).unwrap();

    for person in persons {
        print_person(person);
    }
}

fn print_person(person: Person) {
    println!(
        "\n{}: {}",
        "name".color("blue").reversed(),
        person.name.color("blue")
    );
    println!(
        "{}: {}\n",
        "age".color("blue").reversed(),
        person.age.to_string().color("blue")
    );
}

fn get_persons_bytes_from_file() -> Vec<u8> {
    let bytes: Vec<u8> = match std::fs::read(PERSONDAT) {
        Ok(bytes) => bytes,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::PermissionDenied {
                panic!("invalid permissions. {:?}", e);
            }
            Vec::new()
        }
    };
    bytes
}

fn create_and_open_file(path: &str) -> std::fs::File {
    let file = match OpenOptions::new().write(true).create(true).open(path) {
        Ok(file) => file,
        Err(e) => panic!("Problem opening the file: {:?}", e),
    };

    file
}
