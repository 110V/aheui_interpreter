#[derive(PartialEq,Debug)]
pub enum Direction{
    Up,
    Down,
    Left,
    Right,
    None,
}
#[derive(PartialEq,Debug)]
pub enum Movement{
    Move(Direction,usize),
    VertLine,
    HorLine,
    Bounce,
    Slip,
}