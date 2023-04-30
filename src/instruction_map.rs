use crate::instruction_parser::InstructionParser;
type Tile = Option<char>;
pub struct InstructionMap{
    size:(usize, usize),
    data: Vec<Vec<Option<char>>>
}

impl InstructionMap{
    pub fn from_str(map:&str) -> Self{
        let parser = InstructionParser::new();
        let data: Vec<Vec<Tile>> = Vec::new();

        let mut width = 0;
        let height = data.len();

        for line in map.lines(){
            let mut line_vec: Vec<Tile> = Vec::new();
            if line.len() > width{
                width = line.len();
            }
            for instruction in line.chars(){
                if parser.check_instruction_range(instruction){
                    line_vec.push(Some(instruction));
                }
                line_vec.push(None);
            }
        }
        
        Self{
            size:(width,height),
            data,
        }
    }


}