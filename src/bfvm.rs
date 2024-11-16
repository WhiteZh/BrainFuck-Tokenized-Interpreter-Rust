use std::cell::RefCell;
use std::fmt::Display;
use std::io::Read;

pub enum Command {
    IncPtr,
    DecPtr,
    IncVal,
    DecVal,
    Input,
    Output,
    Group(Vec<Command>),
}

impl Clone for Command {
    fn clone(&self) -> Self {
        use self::Command::*;

        match self {
            IncPtr => IncPtr,
            DecPtr => DecPtr,
            IncVal => IncVal,
            DecVal => DecVal,
            Input => Input,
            Output => Output,
            Group(commands) => Group(commands.clone())
        }
    }
}

impl Display for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use self::Command::*;

        let str = match self {
            IncPtr => String::from("IncPtr"),
            DecPtr => String::from("DecPtr"),
            IncVal => String::from("IncVal"),
            DecVal => String::from("DecVal"),
            Input => String::from("Input"),
            Output => String::from("Output"),
            Group(commands) => format!("Group: {{{}}}", commands.iter().map(|x| x.to_string()).reduce(|a, b| format!("{} {}", a, b)).unwrap_or_else(|| String::from("")))
        };
        write!(f, "{}", str)
    }
}

pub enum Error {
    PtrOverflow,
    MemoryOverflow,
}

pub struct BFVM {
    commands: Vec<Command>,
    memory: RefCell<Vec<i8>>,
    ptr: RefCell<usize>,
}

impl BFVM {
    pub fn new(commands: &Vec<Command>) -> BFVM {
        BFVM {
            commands: (*commands).clone(),
            memory: RefCell::new(vec![0]),
            ptr: RefCell::new(0),
        }
    }

    pub fn start(&self) -> Result<(), Error> {
        self.run(&(self.commands))
    }

    fn run(&self, commands: &Vec<Command>) -> Result<(), Error> {
        for command in commands {
            use self::Error::*;

            match command {
                Command::IncPtr => {
                    let mut ptr = self.ptr.borrow_mut();
                    let mut memory = self.memory.borrow_mut();

                    if usize::MAX == *ptr {
                        return Err(PtrOverflow);
                    }
                    *ptr += 1;

                    if *ptr == (*memory).len() {
                        (*memory).push(0);
                    }
                }
                Command::DecPtr => {
                    let mut ptr = self.ptr.borrow_mut();
                    if usize::MIN == *ptr {
                        return Err(PtrOverflow);
                    }
                    *ptr -= 1;
                }
                Command::IncVal => {
                    let ptr = self.ptr.borrow();
                    let mut memory = self.memory.borrow_mut();

                    if i8::MAX == (*memory)[*ptr] {
                        return Err(MemoryOverflow);
                    }
                    (*memory)[*ptr] += 1;
                }
                Command::DecVal => {
                    let ptr = self.ptr.borrow();
                    let mut memory = self.memory.borrow_mut();

                    if i8::MIN == (*memory)[*ptr] {
                        return Err(MemoryOverflow);
                    }
                    (*memory)[*ptr] -= 1;
                }
                Command::Group(subcommands) => {
                    while {
                        let ptr = self.ptr.borrow();
                        let memory = self.memory.borrow();
                        (*memory)[*ptr] != 0
                    } {
                        match self.run(subcommands) {
                            error @ Err(_) => {
                                return error;
                            }
                            _ => {}
                        }
                    }
                }
                Command::Input => {
                    let mut buf: [u8; 1] = [0u8];
                    std::io::stdin().read_exact(&mut buf).expect("Failed to read stdin");
                    let mut memory = self.memory.borrow_mut();
                    let ptr = self.ptr.borrow();
                    (*memory)[*ptr] = buf[0] as i8;
                }
                Command::Output => {
                    print!("{}", (*self.memory.borrow())[*self.ptr.borrow()] as u8 as char)
                }
            }
        }

        Ok(())
    }
}