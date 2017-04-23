extern crate json;
extern crate textwrap;

use std::fs::File;
use std::io::stdin;
use std::io::stdout;
use std::io::BufReader;
use std::io::prelude::*;
use std::env::home_dir;
use std::path::PathBuf;

use json::JsonValue;
use textwrap::fill;

fn get_location_path(name: &str) -> PathBuf {
    let mut file_name: String = name.to_string();
    file_name.push_str(".json");

    let mut home_dir = home_dir().unwrap();
    home_dir.push("Desktop");
    home_dir.push("dfa");
    home_dir.push("src");
    home_dir.push(file_name);

    home_dir
}

fn read_location(name: &str) -> JsonValue {
    let file_name = get_location_path(name);
    let file = File::open(file_name).unwrap();
    let mut buf_reader = BufReader::new(file);
    let mut contents = String::new();

    buf_reader.read_to_string(&mut contents).unwrap();
    json::parse(contents.as_str()).unwrap()
}

fn print_location(location: &JsonValue) {
    let title = location["title"].as_str().unwrap();

    println!("{}\n", title);
    for text in location["text"].members() {
        println!("{}", fill(text.as_str().unwrap(), 60));
        println!();
    }

    for option in location["options"].members() {
        println!("- {}", option["description"].as_str().unwrap())
    }
    println!();
}

fn get_input(location: &JsonValue) -> String {
    let mut input = String::new();
    print!(">>> ");
    stdout().flush();
    match stdin().read_line(&mut input) {
        Ok(n) => return input,
        Err(e) => return input,
    }
}

fn main() {
    let location = read_location("home");
    let mut input = String::new();

    print_location(&location);
    loop {
        input = get_input(&location);

        if input == "exit" {
            break;
        }
    }
}
