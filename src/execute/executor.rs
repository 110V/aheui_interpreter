use crate::movement::{Direction, Movement};
use crate::instruction::Instruction;
use crate::instruction_map::InstructionMap;
use crate::instruction_parser::InstructionParser;
use crate::memory::Memory;
use crate::command::{Command, OutType};
use crate::memory::{Queue,Stack};

use super::command_result::CommandResult;


struct executor{
    current_memory: usize,
    memories: Vec<Box<dyn Memory>>,
    output: String,
    map: InstructionMap,
    dir:Direction,
    cursor:(usize,usize),
    parser: InstructionParser,
}

impl executor{
    pub fn new(map:&str) -> executor{
        let mut memories: Vec<Box<dyn Memory>> = Vec::new();
        for _ in 0..26{
            memories.push(Box::new(Stack::new()));
        }
        memories.push(Box::new(Queue::new()));
        memories.push(Box::new(Stack::new()));

        let current_memory = 0;
        let dir = Direction::Down;
        let cursor = (0,0);
        let parser = InstructionParser::new();
        executor{
            current_memory,
            memories,output:"".to_string(),
            map:InstructionMap::from_str(map),
            dir,
            cursor,
            parser,
        }
    }
    pub fn move_cursor(&mut self){
        let (x,y) = self.cursor;

        let mut _x = x as i32;
        let mut _y = y as i32;

        _x = _x % self.map.get_width() as i32;
        _y = _y % self.map.get_height() as i32;

        self.cursor = (_x as usize,_y as usize);
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

    pub fn set_dir(&mut self, dir:Direction){
        if let Direction::None = dir{
            return;
        }
        self.dir = dir;
    }

    pub fn flip_dir(&mut self){
        let dir = &self.dir;
        let new_dir = match dir{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        };
        self.dir = new_dir;
    }

    pub fn run_current_instruction(&mut self){
        // let instruction = self.map.get(self.cursor.0,self.cursor.1)?;
        // if let Some(instruction) = instruction{
        //     let instruction = self.parser.parse_instruction(instruction)?;
        //     self.run_instruction(instruction)?;
        // }
    }

    pub fn run_instruction(&mut self,instruction:Instruction)->CommandResult{
        let command = instruction.command;
        let movement = instruction.movement;
        let mut distance = 1;

        if let Movement::Move(dir,d) = movement{
            self.set_dir(dir);
            distance = d;
        }

        if let Err(_) = self.run_command(command){
            self.flip_dir();
        }

        self.move_cursor();
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