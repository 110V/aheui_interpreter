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
}

