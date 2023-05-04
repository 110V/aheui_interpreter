use crate::instruction_parser::InstructionParser;
type Tile = Option<char>;
pub struct InstructionMap{
    size:(usize, usize),
    data: Vec<Vec<Option<char>>>
}

impl InstructionMap{
    pub fn from_str(map:&str) -> Self{
        let parser = InstructionParser::new();
        let mut data: Vec<Vec<Tile>> = Vec::new();

        let mut width = 0;


        for line in map.lines(){
            let mut line_vec: Vec<Tile> = Vec::new();
            let chars = line.chars();
            let &count = &chars.count();
            if  count > width{
                width = count;
            }
            for instruction in line.chars(){
                if parser.check_instruction_range(instruction){
                    line_vec.push(Some(instruction));
                    continue;
                }
                line_vec.push(None);
            }
            data.push(line_vec);
        }
        let height = data.len();
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

#[test]
fn test_from_str() {
    let map_str = "가나다\n라마";
    let map = InstructionMap::from_str(map_str);
    assert_eq!(map.get_width(), 3);
    assert_eq!(map.get_height(), 2);
    assert_eq!(map.get(0, 0), Ok(Some('가')));
    assert_eq!(map.get(1, 0), Ok(Some('나')));
    assert_eq!(map.get(2, 0), Ok(Some('다')));
    assert_eq!(map.get(0, 1), Ok(Some('라')));
    assert_eq!(map.get(1, 1), Ok(Some('마')));
    assert_eq!(map.get(2, 1), Ok(None));
    assert_eq!(map.get(3, 0), Err(String::from("Instruction Map Out of range: (3,0)")));
}

#[test]
fn test_from_str2() {
    let map_str = "방방다망함";
    let map = InstructionMap::from_str(map_str);
    assert_eq!(map.get_width(), 5);
    assert_eq!(map.get_height(), 1);
    assert_eq!(map.get(0, 0), Ok(Some('방')));
    assert_eq!(map.get(1, 0), Ok(Some('방')));
    assert_eq!(map.get(2, 0), Ok(Some('다')));
    assert_eq!(map.get(3, 0), Ok(Some('망')));
    assert_eq!(map.get(4, 0), Ok(Some('함')));

}