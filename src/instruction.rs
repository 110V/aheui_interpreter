use crate::{Command, movement::Movement};

#[derive(PartialEq,Debug)]
pub struct Instruction{
    pub command: Command,
    pub movement: Movement
}

impl Instruction{
    pub fn new(command: Command, movement: Movement) -> Self{
        Self{
            command,
            movement
        }
    }
}

