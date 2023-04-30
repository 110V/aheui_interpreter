pub enum Direction{
    Up,
    Down,
    Left,
    Right,
    None,
}

pub enum Movement{
    Move(Direction,usize),
    VertLine,
    HorLine,
    Bounce,
    Slip,
}