use std::cell::RefCell;
use std::io::{Read, Write};
use std::fs::File;
use crate::bfvm::{Error, BFVM};
use crate::bf_tokenizer::tokenize;

mod bfvm;
mod bf_tokenizer;

const DEBUG: bool = false;

fn main() {
    let mut content = String::new();

    let mut file_name = String::new();
    print!("File name: ");
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut file_name).expect("Failed to read from stdin");
    let file_name: String = file_name.chars().take_while(|c| !c.is_whitespace()).collect();

    let mut file = File::open(file_name).expect("File not found!");

    file.read_to_string(&mut content).expect("Failed to read file!");

    let commands = tokenize(&RefCell::new(Box::new(content.chars()) as Box<dyn Iterator<Item = char>>));

    if DEBUG {
        for each in commands.iter().by_ref() {
            println!("{}", each.to_string());
        }
    }

    let bfvm = BFVM::new(&commands);

    match bfvm.start() {
        Err(error) => match error {
            Error::PtrOverflow => {
                panic!("Pointer overflowed!");
            }
            Error::MemoryOverflow => {
                panic!("Memory overflowed!");
            }
        }
        _ => {}
    }
}
