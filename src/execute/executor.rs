use crate::instruction::Instruction;
use crate::memory::Memory;
use crate::command::{Command, OutType};
use crate::memory::{Queue,Stack};

use super::command_result::CommandResult;


struct executor{
    current_memory: usize,
    memories: Vec<Box<dyn Memory>>,
    output: String,
}

impl executor{
    pub fn new() -> executor{
        let mut memories: Vec<Box<dyn Memory>> = Vec::new();
        for _ in 0..26{
            memories.push(Box::new(Stack::new()));
        }
        memories.push(Box::new(Queue::new()));
        memories.push(Box::new(Stack::new()));

        let current_memory = 0;
        executor{current_memory, memories,output:"".to_string()}
    }

    pub fn get_current_memory(&mut self) -> &mut dyn Memory{
        if self.current_memory >= self.memories.len(){
            panic!("Memory out of bounds");
        }
        &mut *self.memories[self.current_memory]
    }

    pub fn out_number(&mut self, data: i32){
        self.output.push_str(&data.to_string());
    }

    pub fn out_char(&mut self, data: i32)->CommandResult{
        let c = std::char::from_u32(data as u32).ok_or("Invalid char")?;
        self.output.push(c);
        Ok(())
    }


    pub fn run_instruction(instruction:Instruction)->CommandResult{
        
        Ok(())
    }

    pub fn run_command(&mut self, command:Command)->CommandResult{
        match command{
            Command::Push(data) => {
                let current_memory = self.get_current_memory();
                current_memory.push(data);
            },
            Command::Pop(out_type) => {
                let current_memory = self.get_current_memory();
                let value = current_memory.pop();
                match out_type{
                    OutType::Number => {
                        if let Some(value) = value{
                            self.out_number(value);
                        }
                    },
                    OutType::Char => {
                        if let Some(value) = value{
                            self.out_char(value)?;
                        }
                    },
                    OutType::None => {
                        
                    }
                }
            },
            Command::Add => {
                
            },
            Command::Sub => {

            },
            Command::Mul => {

            },
            Command::Div => {

            },
            Command::Rem => {

            },
            Command::Dup => {

            },
            Command::Swap => {

            },
            Command::Set(data) => {

            },
            Command::Move => {

            },
            Command::Comp => {

            },
            Command::If => {

            },
            Command::Quit => {

            },
            Command::None => {

            }
        }
        Ok(())
    }
}