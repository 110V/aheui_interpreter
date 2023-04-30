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
    pub fn get(&self, x:usize, y:usize) -> Result<Option<char>,String>{
        if x >= self.size.0 || y >= self.size.1{
            return Err(format!("Instruction Map Out of range: ({},{})",x,y));
        }
        if self.data[y].len() <= x{
            return Ok(None);
        }
        Ok(self.data[y][x])
    }
    pub fn get_width(&self) -> usize{
        self.size.0
    }
    pub fn get_height(&self) -> usize{
        self.size.1
    }

}