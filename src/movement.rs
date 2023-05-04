#[derive(PartialEq,Debug,Clone)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
    None,
}

impl Direction {
    pub fn flip(&self)->Self{
        let dir = &self;
        match dir{
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::None => Direction::None,
        }
    }
}

#[derive(PartialEq,Debug,Clone)]
pub enum Movement{
    Move(Direction,usize),
    VertLine,
    HorLine,
    Bounce,
    Slip,
}