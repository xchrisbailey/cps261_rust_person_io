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
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
struct Persons(Vec<Person>);

fn main() {
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
    let p: Person = create_person();
    if Path::new(PERSONDAT).exists() {
        match append_person_to_file(p) {
            Ok(_) => println!("Person saved to disk"),
            _ => println!("Could not write file"),
        }
    } else {
        match write_person_to_file(p) {
            Ok(_) => println!("Person saved to disk"),
            _ => println!("Could not write file"),
        }
    }
}

fn write_person_to_file(person: Person) -> Result<()> {
    // open/ create file
    let mut file = match OpenOptions::new().write(true).create(true).open(PERSONDAT) {
        Ok(file) => file,
        Err(e) => panic!("Problme opening the file: {:?}", e),
    };

    let persons: Persons = Persons(vec![person]);

    // encode person struct to binary
    let bp: Vec<u8> = bincode::serialize(&persons).unwrap();

    // write binary vector to the file
    file.write_all(&bp)?;
    Ok(())
}

fn append_person_to_file(person: Person) -> Result<()> {
    // open/ create file
    let mut file = match OpenOptions::new().write(true).create(true).open(PERSONDAT) {
        Ok(file) => file,
        Err(e) => panic!("Problme opening the file: {:?}", e),
    };

    let persons_bytes = get_persons_bytes_from_file();
    let mut persons: Persons = bincode::deserialize(&persons_bytes[..]).unwrap();
    println!("{:?}", persons);
    persons.0.push(person);
    println!("{:?}", persons);
    // encode person struct to binary
    let bp: Vec<u8> = bincode::serialize(&persons).unwrap();

    // write binary vector to the file
    file.write_all(&bp)?;
    Ok(())
}

fn create_person() -> Person {
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
    p
}

fn display_persons() {
    let bytes: Vec<u8> = get_persons_bytes_from_file();
    let persons: Vec<Person> = bincode::deserialize(&bytes[..]).unwrap();
    println!("{:?}", persons);
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
