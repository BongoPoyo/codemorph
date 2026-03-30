use std::fs;
use std::io::{self, Read};

pub fn read_input(input: &Option<String>) -> String {
    match input {
        Some(file) => fs::read_to_string(file).expect("Failed to read file"),
        None => {
            let mut buffer = String::new();
            io::stdin().read_to_string(&mut buffer).unwrap();
            buffer
        }
    }
}

pub fn write_output(input: &Option<String>, output: &str, in_place: bool) {
    if in_place {
        if let Some(file) = input {
            fs::write(file, output).expect("Failed to write file");
        }
    } else {
        println!("{}", output);
    }
}
