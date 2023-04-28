use crate::{Command, direction::Direction};


struct Instruction{
    command: Command,
    direction: Direction
}

impl Instruction{
    fn new(command: Command, direction: Direction) -> Self{
        Self{
            command,
            direction
        }
    }
}

