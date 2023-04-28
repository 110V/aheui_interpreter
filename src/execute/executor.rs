use crate::memory::memory::Memory;
use crate::command::Command;

struct executor{
    
    memories: Vec<Box<dyn Memory>>
}

impl executor{
    pub fn new() -> executor{
        executor{}
    }
    pub fn run_instruction()->bool{

    }
    pub fn run_command()->bool{

    }
}