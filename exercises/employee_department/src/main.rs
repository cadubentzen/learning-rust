use std::collections::HashMap;
use std::io;

extern crate regex;
use regex::Regex;

enum Command {
    Print,
    Stop,
    Add(String, String),
    Ignore
}

fn main() {
    let mut map: HashMap<String, Vec<String> > = HashMap::new();
    
    loop {
        let mut input = String::new();

        io::stdin().read_line(&mut input).unwrap();

        match parse_input(input.trim().to_lowercase()) {
            Command::Print => println!("{:?}", map),
            Command::Stop => break,
            Command::Add(employee, department) => {
                add_employee(employee, 
                             department,
                             &mut map)
            },
            Command::Ignore => {
                // Add some msg
            }
        };
    }

    println!("{:?}", map);
}

fn parse_input(input: String) -> Command {
    if input == "print" {
        Command::Print
    } else if input == "stop" || input == "quit" {
        Command::Stop
    } else {
        let re = Regex::new(r"^add ([a-z]+) to ([a-z]+)$").unwrap();
        match re.captures(&input) {
            None => Command::Ignore,
            Some(c) => {
                Command::Add(String::from(&c[1]), 
                             String::from(&c[2]))
            }   
        }
    }
}

fn add_employee(employee: String,
                department: String,
                map: &mut HashMap<String, Vec<String> >) {
    
    let list = map.entry(department).or_insert(Vec::new());
    list.push(employee);
}
