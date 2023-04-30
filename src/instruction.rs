use crate::{Command, movement::Movement};


pub struct Instruction{
    pub command: Command,
    pub movement: Movement
}

impl Instruction{
    fn new(command: Command, movement: Movement) -> Self{
        Self{
            command,
            movement
        }
    }
}

