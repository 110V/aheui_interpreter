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