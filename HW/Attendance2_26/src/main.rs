
use std::io::{self, Read, Write};

struct Person {
    name: String,
    ID: u32,
}

fn main() {
    let mut buffer = String::new();

    print!("What's your name? ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let name = buffer.trim().to_string();
    buffer.clear();

    print!("Enter your student ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut buffer).unwrap();
    let ID = buffer.trim().parse().unwrap();

    let person = Person { name, ID };
    println!("Hi {}, your student ID is {}!", person.name, person.ID);
}