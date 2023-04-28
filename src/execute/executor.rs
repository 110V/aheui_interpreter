use crate::memory::Memory;
use crate::command::{Command, OutType};
use crate::memory::{Queue,Stack};

struct executor{
    current_memory: usize,
    memories: Vec<Box<dyn Memory>>,
    output: String
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
        &mut *self.memories[self.current_memory]
    }

    pub fn out_number(&mut self, data: i32)->bool{
        self.output.push_str(&data.to_string());
        true
    }

    pub fn out_char(&mut self, data: i32)->bool{
        if let Some(c) = std::char::from_u32(data as u32){
            self.output.push(c);
            return true
        }
        false
    }


    pub fn run_instruction()->bool{
        false
    }
    pub fn run_command(&mut self, command:Command)->bool{
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
                            self.out_char(value);
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
        true
    }
}