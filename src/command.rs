pub enum OutType{
    Char,
    Number,
    None,
}
pub enum Command{
    Add,
    Mul,
    Sub,
    Div,
    Rem,
    Pop(OutType),
    Push(i32),
    Dup,
    Swap,
    Set(i32),
    Move,
    Comp,
    If,
    Quit,
    None,
}