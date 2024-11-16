use std::cell::RefCell;
use crate::bfvm::Command;

pub fn tokenize(iter: &RefCell<Box<dyn Iterator<Item = char>>>) -> Vec::<Command> {
    let mut commands = Vec::<Command>::new();
    
    loop {
        use Command::*;

        let c = match (*iter.borrow_mut()).next() {
            Some(v) => v,
            None => {
                break;
            }
        };
        
        match c {
            '+' => {
                commands.push(IncVal);
            }
            '-' => {
                commands.push(DecVal);
            }
            '>' => {
                commands.push(IncPtr);
            }
            '<' => {
                commands.push(DecPtr);
            }
            '.' => {
                commands.push(Output)
            }
            ',' => {
                commands.push(Input)
            }
            '[' => {
                commands.push(Group(tokenize(iter)))
            }
            ']' => {
                return commands;
            }
            _ => {}
        }
    }
    
    commands
}