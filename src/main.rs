use std::cell::RefCell;
use crate::bfvm::{Error, BFVM};
use crate::bf_tokenizer::tokenize;

mod bfvm;
mod bf_tokenizer;

fn main() {
    let mut content = Vec::<char>::new();

    for line in std::io::stdin().lines().map(|line| line.unwrap()) {
        for char in line.chars() {
            content.push(char);
        }
    }

    let bfvm = BFVM::new(&tokenize(&RefCell::new(Box::new(content.into_iter()) as Box<dyn Iterator<Item = char>>)));

    match bfvm.start() {
        Err(error) => match error {
            Error::PtrOverflow => {
                eprintln!("Pointer overflowed!");
            }
            Error::MemoryOverflow => {
                eprintln!("Memory overflowed!");
            }
        }
        _ => {}
    }
}
