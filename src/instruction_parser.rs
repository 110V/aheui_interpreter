use crate::{Command, instruction::Instruction, command::{IOType, PopType, PushType, self}, movement::{Movement, Direction}};

pub struct InstructionParser{

}

impl InstructionParser{
    pub fn new() -> Self{
        Self{

        }
    }
    pub fn check_instruction_range(&self,c:char)->bool{
        '가' <= c && c <= '힣' 
    }

    pub fn parse_instruction(&self,raw:char)->Instruction{
        let (cho,jung,jong) = self.disassemble_raw(raw).unwrap();
        let command = self.parse_command(cho,jong);
        let movement = self.parse_movement(jung);

        Instruction{
            command,
            movement,
        }
    }
    pub fn parse_movement(&self,c:char)->Movement{
        match c{
            'ㅏ' => Movement::Move(Direction::Right,1),
            'ㅓ' => Movement::Move(Direction::Left,1),
            'ㅗ' => Movement::Move(Direction::Up,1),
            'ㅜ' => Movement::Move(Direction::Down,1),
            'ㅑ' => Movement::Move(Direction::Right,2),
            'ㅕ' => Movement::Move(Direction::Left,2),
            'ㅛ' => Movement::Move(Direction::Up,2),
            'ㅠ' => Movement::Move(Direction::Down,2),
            'ㅣ' => Movement::VertLine,
            'ㅡ' => Movement::HorLine,
            'ㅢ' => Movement::Bounce,
            _ => Movement::Slip,
        }
    }

    pub fn count_jong(&self,c:Option<char>)->i32{
        if c.is_none(){
            return 0;
        }
        
        match c.unwrap(){
            'ㄱ'|'ㄴ'|'ㅅ'=>2,
            'ㄷ'|'ㅈ'|'ㅋ'=>3,
            'ㅁ'|'ㅂ'|'ㅊ'|'ㅌ'|'ㅍ'|'ㄲ'|'ㄳ'|'ㅆ' =>4,
            'ㄹ'|'ㄵ'|'ㄶ' =>5,
            'ㅄ' =>6,
            'ㄺ'|'ㄽ' =>7,
            'ㅀ' =>8,
            'ㄻ'|'ㄼ'|'ㄾ'|'ㄿ' =>9,
            _=>unreachable!()
        }

    }

    pub fn parse_memory_index(&self,c:Option<char>)->usize{
        if c.is_none(){
            return 0;
        }
        match c.unwrap(){
            'ㄱ' => 1,
            'ㄴ' => 2,
            'ㄷ' => 3,
            'ㄹ' => 4,
            'ㅁ' => 5,
            'ㅂ' => 6,
            'ㅅ' => 7,
            'ㅈ' => 8,
            'ㅊ' => 9,
            'ㅋ' => 10,
            'ㅌ' => 11,
            'ㅍ' => 12,
            'ㄲ' => 13,
            'ㄳ' => 14,
            'ㄵ' => 15,
            'ㄶ' => 16,
            'ㄺ' => 17,
            'ㄻ' => 18,
            'ㄼ' => 19,
            'ㄽ' => 20,
            'ㄾ' => 21,
            'ㄿ' => 22,
            'ㅀ' => 23,
            'ㅄ' => 24,
            'ㅆ' => 25,
            'ㅇ' => 26,
            'ㅎ' => 27,
            _ => unreachable!(),
        }

    }

    pub fn parse_io_type(&self,c:Option<char>)->Option<IOType>{
        if c.is_none(){
            return None;
        }
        match c.unwrap(){
            'ㅇ' => Some(IOType::Number),
            'ㅎ' => Some(IOType::Char),
            _ => None,
        }
    }

    pub fn parse_command(&self,cho:char,jong:Option<char>)->Command{
        match cho{
            //셈
            'ㄷ' => {
                Command::Add
            }
            'ㄸ' => {
                Command::Mul
            }
            'ㅌ' => {
                Command::Sub
            }
            'ㄴ' => {
                Command::Div
            }
            'ㄹ' => {
                Command::Rem
            }

            //저장공간
            'ㅁ' => {
                let mut pop_type = PopType::Discard;
                if let Some(io_type) = self.parse_io_type(jong){
                    pop_type = PopType::Output(io_type);
                }
                Command::Pop(pop_type)
            }
            'ㅂ' => {
                if let Some(io_type) = self.parse_io_type(jong){
                    return Command::Push(PushType::Input(io_type))
                }
                Command::Push(PushType::Value(self.count_jong(jong)))
            }
            'ㅃ' => {
                Command::Dup
            }
            'ㅍ' => {
                Command::Swap
            }
            //제어, 저장공간 확장
            'ㅅ' => {
                Command::Set(self.parse_memory_index(jong))
            }
            'ㅆ' => {
                Command::Move(self.parse_memory_index(jong))
            }
            'ㅈ' => {
                Command::Comp
            }
            'ㅊ' => {
                Command::If
            },
            'ㅎ' => {
                Command::Quit
            }
            _ => Command::None,
            
        }
    }

    pub fn disassemble_raw(&self,c:char)->Option<(char,char,Option<char>)>{
        self.check_instruction_range(c).then(||())?;
        let cho:u32 = (c as u32 - '가' as u32) / 28 / 21;
        let jung:u32 = (c as u32 - '가' as u32) / 28 % 21 + 'ㅏ' as u32;
        let jong:u32 = (c as u32 - '가' as u32) % 28;

        let chos = ['ㄱ','ㄲ','ㄴ','ㄷ','ㄸ','ㄹ','ㅁ',
                    'ㅂ','ㅃ','ㅅ','ㅆ','ㅇ','ㅈ','ㅉ',
                    'ㅊ','ㅋ','ㅌ','ㅍ','ㅎ'];
        let jongs = ['ㄱ','ㄲ','ㄳ','ㄴ','ㄵ','ㄶ','ㄷ','ㄹ',
                    'ㄺ','ㄻ','ㄼ','ㄽ','ㄾ','ㄿ','ㅀ','ㅁ',
                    'ㅂ','ㅄ','ㅅ','ㅆ','ㅇ','ㅈ','ㅊ','ㅋ',
                    'ㅌ','ㅍ','ㅎ'];

        let mut jong_char:Option<char> = None;
        if jong != 0
        {
            jong_char = Some(jongs[jong as usize - 1]);
        }
        Some((chos[cho as usize],char::from_u32(jung)?, jong_char))
    }
}

mod tests{
    #[test]
    fn test_disassemble_raw(){
        use super::*;
        let parser: InstructionParser = InstructionParser::new();
        assert_eq!(parser.disassemble_raw('가'),Some(('ㄱ','ㅏ',None)));
        assert_eq!(parser.disassemble_raw('나'),Some(('ㄴ','ㅏ',None)));
        assert_eq!(parser.disassemble_raw('삐'),Some(('ㅃ','ㅣ',None)));
        assert_eq!(parser.disassemble_raw('힣'),Some(('ㅎ','ㅣ',Some('ㅎ'))));
        assert_eq!(parser.disassemble_raw('ㄱ'),None);
        assert_eq!(parser.disassemble_raw('ㅏ'),None);
        assert_eq!(parser.disassemble_raw('ㅣ'),None);
        assert_eq!(parser.disassemble_raw('a'),None);
        assert_eq!(parser.disassemble_raw('A'),None);
        assert_eq!(parser.disassemble_raw(' '),None);
    }
    #[test]
    fn test_parse_command(){
        use super::*;
        use Command::*;
        let parser: InstructionParser = InstructionParser::new();
        let op_none = Option::None;
        assert_eq!(parser.parse_command('ㄷ',op_none),Add);

        assert_eq!(parser.parse_command('ㅁ',Some('ㅇ')),Pop(PopType::Output(IOType::Number)));
        assert_eq!(parser.parse_command('ㅁ',Some('ㅎ')),Pop(PopType::Output(IOType::Char)));
        
        assert_eq!(parser.parse_command('ㅂ',Some('ㅇ')),Push(PushType::Input(IOType::Number)));
        assert_eq!(parser.parse_command('ㅂ',Some('ㅎ')),Push(PushType::Input(IOType::Char)));
        assert_eq!(parser.parse_command('ㅅ',Some('ㄴ')),Set(2));
        assert_eq!(parser.parse_command('ㅆ',Some('ㅅ')),Move(7));
        
        assert_eq!(parser.parse_command('ㅁ',Some('ㅈ')),Pop(PopType::Discard));
        assert_eq!(parser.parse_command('ㅂ',Some('ㅋ')),Push(PushType::Value(3)));
        assert_eq!(parser.parse_command('ㅅ',Some('ㅌ')),Set(11));

        assert_eq!(parser.parse_command('ㅁ',op_none),Pop(PopType::Discard));
        assert_eq!(parser.parse_command('ㅂ',op_none),Push(PushType::Value(0)));
        assert_eq!(parser.parse_command('ㅅ',op_none),Set(0));
    }

    #[test]
    fn test_parse_instruction(){
        use super::*;
        use Command::*;
        let parser: InstructionParser = InstructionParser::new();

        assert_eq!(parser.parse_instruction('다'),Instruction::new(Add,Movement::Move(Direction::Right, 1)));
        assert_eq!(parser.parse_instruction('송'),Instruction::new(Set(26),Movement::Move(Direction::Up, 1)));
    }
}