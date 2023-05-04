use crate::movement::{Direction, Movement};
use crate::instruction::Instruction;
use crate::instruction_map::InstructionMap;
use crate::instruction_parser::InstructionParser;
use crate::memory::Memory;
use crate::command::{Command, IOType, PushType, PopType};
use crate::memory::{Queue,Stack};
use crate::stdin::Stdin;

use super::command_result::CommandResult;


pub struct Executor{
    current_memory: usize,
    memories: Vec<Box<dyn Memory>>,
    output: String,
    input: Stdin,
    map: InstructionMap,
    dir:Direction,
    cursor:(usize,usize),
    parser: InstructionParser,
    quit:bool,
}

impl Executor{
    pub fn new(map:&str,input:&str) -> Executor{
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
        let input = Stdin::new(input);
        Executor{
            current_memory,
            memories,output:"".to_string(),
            input,
            map:InstructionMap::from_str(map),
            dir,
            cursor,
            parser,
            quit:false
        }
    }
    pub fn get_output(&self) -> &str{
        &self.output
    }
    pub fn excute(&mut self){
        while !self.quit{
            self.run_current_instruction();
        }
    }

    pub fn run_movement(&mut self, movement: Movement){
        match movement{
            Movement::Move(direction,distance) => {
                self.dir = direction;
                for _ in 0..distance{
                    self.move_cursor();
                }
            },
            Movement::Bounce =>  {
                self.flip_current_dir();
                self.move_cursor();
            },
            Movement::VertLine =>  {
                match &self.dir{
                    Direction::Left => self.dir = Direction::Right,
                    Direction::Right => self.dir = Direction::Left,
                    _ => {}
                }
                self.move_cursor();
            },
            Movement::HorLine =>  {
                match &self.dir{
                    Direction::Up => self.dir = Direction::Down,
                    Direction::Down => self.dir = Direction::Up,
                    _ => {}
                }
                self.move_cursor();
            },
            Movement::Slip => {
                self.move_cursor();
            }
        }
  
    }


    pub fn move_cursor(&mut self){
        let (x,y) = self.cursor;

        let mut _x = x as i32;
        let mut _y = y as i32;

        match &self.dir{
            Direction::Up => _y -= 1,
            Direction::Down => _y += 1,
            Direction::Left => _x -= 1,
            Direction::Right => _x += 1,
            _ => unreachable!(),
        }
        let w = self.map.get_width() as i32;
        let h = self.map.get_height() as i32;
        _x = (w +_x) % w;
        _y = (h+ _y) % h;

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

    pub fn out_char(&mut self, data: i32){
        let c = std::char::from_u32(data as u32);
        if let Some(c) = c{
            self.output.push(c);
        };
    }

    pub fn set_dir(&mut self, dir:Direction){
        if let Direction::None = dir{
            return;
        }
        self.dir = dir;
    }

    pub fn flip_current_dir(&mut self){
        self.dir = self.dir.flip();
    }

    pub fn run_current_instruction(&mut self){
        let instruction = self.map.get(self.cursor.0,self.cursor.1).unwrap_or_else(|e|panic!("{}",e));

        if let Some(instruction) = instruction{
            println!("{}",instruction);
            let instruction = self.parser.parse_instruction(instruction);
            if self.quit{
                return;
            }
            println!("{:?}",&instruction);
            self.run_instruction(instruction);
            

        }
        else{
            self.move_cursor();
        }
    }


    pub fn run_instruction(&mut self,instruction:Instruction){
        let command = instruction.command;
        let mut movement = instruction.movement;
        
        if let Err(_) = self.run_command(command){
            if let Movement::Move(dir,d) = movement{
                movement = Movement::Move(dir.flip(),d);
                println!("{:?}","갤럭시 z 플립");
            }
            else{
                self.flip_current_dir();
            }
        }
        self.get_current_memory().print();
        self.run_movement(movement)
    }

    pub fn push_current_memory(&mut self, data:i32){
        let current_memory = self.get_current_memory();
        current_memory.push(data);
    }
    

    pub fn get_input(&mut self,io_type:IOType){
        
        match io_type{
            IOType::Number => {
                let value = self.input.read_number();
                self.push_current_memory(value);
            },
            IOType::Char => {
                let value = self.input.read_char();
                self.push_current_memory(value);
            },
        }
    }

    pub fn check_elements_exist(&mut self,count:usize)->CommandResult{
        let current_memory = self.get_current_memory();
        if current_memory.len() < count{
            return Err("not enough element".to_string());
        }
        Ok(())
    }
    pub fn pop_one(&mut self)->Result<i32,String>{
        let current_memory = self.get_current_memory();
        let value = current_memory.pop().ok_or("no element")?;
        Ok(value)
    }

    pub fn pop_pair(&mut self)->Result<(i32,i32),String>{
        self.check_elements_exist(2)?;
        let current_memory = self.get_current_memory();
        let a = current_memory.pop().unwrap();
        let b = current_memory.pop().unwrap();
        Ok((a,b))
    }

    pub fn run_command(&mut self, command:Command)->CommandResult{
        match command{
            Command::Push(push_type) => {
                match push_type {
                    PushType::Input(io_type) => {
                        self.get_input(io_type);
                    },
                    PushType::Value(value) => {
                        self.push_current_memory(value);
                    },
                }
            },
            Command::Pop(pop_type) => {
                let current_memory = self.get_current_memory();
                let value = current_memory.pop().ok_or("no element")?;
                if let PopType::Output(io_type) = pop_type{
                    match io_type{
                        IOType::Number => {
                            self.out_number(value);
                            // print!("{}",value);
                        },
                        IOType::Char => {
                            self.out_char(value);
                        },
                    }
                }

            },
            Command::Add => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(a+b);
            },
            Command::Sub => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(b-a)
            },
            Command::Mul => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(a*b)
            },
            Command::Div => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(b/a)
            },
            Command::Rem => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(b%a)
            },
            Command::Dup => {
                let current_memory = self.get_current_memory();
                let value = current_memory.pop().ok_or("no element")?;
                self.push_current_memory(value);
                self.push_current_memory(value);
            },
            Command::Swap => {
                let (a,b) = self.pop_pair()?;
                self.push_current_memory(a);
                self.push_current_memory(b);
            },
            Command::Set(data) => {
                self.current_memory = data;
            },
            Command::Move(index) => {
                let data = self.pop_one()?;
                self.memories[index].push(data);
            },
            Command::Comp => {
                let (a,b) = self.pop_pair()?;
                if a > b{
                    self.push_current_memory(0);
                }
                else{
                    self.push_current_memory(1);
                }
            },
            Command::If => {
                let data = self.pop_one()?;
                if data == 0{
                    return Err("if fail".to_string())
                }
            },
            Command::Quit => {
                self.quit = true;
            },
            Command::None => {

            }
        }
        Ok(())
    }
}