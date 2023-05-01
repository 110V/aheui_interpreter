use crate::{Command, instruction::Instruction, command::OutType};

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
        let command = self.parse_command(cho,jung,jong);
        let movement = self.parse_movement(cho,jung,jong);
        Instruction{
            command,
            movement,
        }
    }

    pub fn count_jong(&self,c:Option<char>)->i32{
        if c.is_none(){
            return 0;
        }
        match Ok(c){
            'ㄱ'|'ㄴ'|'ㅅ'=>2,
            'ㄷ'|'ㅈ'|'ㅋ'=>3,
            'ㅁ'|'ㅂ'|'ㅊ'|'ㅌ'|'ㅍ'|'ㄲ'|'ㄳ'|'ㅆ' =>4,
            'ㄹ'|'ㄵ'|'ㄶ' =>5,
            'ㅄ' =>6,
            'ㄺ'|'ㄽ' =>7,
            'ㅀ' =>8,
            'ㄻ'|'ㄼ'|'ㄾ'|'ㄿ' =>9,
        }

    }

    pub fn parse_command(&self,cho:char,jong:Option<char>){
        let mut command = Command::None;
        match cho{
            //셈
            'ㄷ' => {
                command = Command::Add;
            }
            'ㄸ' => {
                command = Command::Sub;
            }
            'ㅌ' => {
                command = Command::Mul;
            }
            'ㄴ' => {
                command = Command::Div;
            }
            'ㄹ' => {
                command = Command::Rem;
            }

            //저장공간
            'ㅁ' => {
                let mut out_type = OutType::None;
                match jong{
                    Some('ㅇ') => {
                        out_type = OutType::Number;
                    }
                    Some('ㅎ') => {
                        out_type = OutType::Char;
                    }
                    _ => {

                    }
                }
                command = Command::Pop(out_type);
            }
            'ㅂ' => {
                command = Command::Push;
            }
            'ㅃ' => {
                command = Command::Dup;
            }
            'ㅍ' => {
                command = Command::Swap;
            }

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

    pub fn parse_instruction(c:char){
        
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
}