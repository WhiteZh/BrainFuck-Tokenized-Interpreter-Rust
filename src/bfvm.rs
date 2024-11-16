use std::cell::RefCell;

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

pub enum Error {
    PtrOverflow,
    MemoryOverflow,
}

pub struct BFVM {
    commands: Vec<Command>,
    memory: RefCell<Vec<u8>>,
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

                    if u8::MAX == (*memory)[*ptr] {
                        return Err(MemoryOverflow);
                    }
                    (*memory)[*ptr] += 1;
                }
                Command::DecVal => {
                    let ptr = self.ptr.borrow();
                    let mut memory = self.memory.borrow_mut();

                    if u8::MIN == (*memory)[*ptr] {
                        return Err(MemoryOverflow);
                    }
                    (*memory)[*ptr] -= 1;
                }
                Command::Group(subcommands) => {
                    match self.run(subcommands) {
                        error @ Err(_) => {
                            return error;
                        }
                        _ => {}
                    }
                }
                Command::Input => {
                    let mut buf = String::new();
                    std::io::stdin().read_line(&mut buf).unwrap();
                    let mut memory = self.memory.borrow_mut();
                    let ptr = self.ptr.borrow();
                    (*memory)[*ptr] = buf.as_bytes()[0];
                }
                Command::Output => {
                    print!("{}", (*self.memory.borrow())[*self.ptr.borrow()])
                }
            }
        }

        Ok(())
    }
}