#[derive(PartialEq,Debug)]
pub enum IOType{
    Char,
    Number,
}

#[derive(PartialEq,Debug)]
pub enum PushType{
    Input(IOType),
    Value(i32),
}
#[derive(PartialEq,Debug)]
pub enum PopType{
    Output(IOType),
    Discard,
}

#[derive(PartialEq,Debug)]
pub enum Command{
    Add,
    Mul,
    Sub,
    Div,
    Rem,
    Pop(PopType),
    Push(PushType),
    Dup,
    Swap,
    Set(usize),
    Move,
    Comp,
    If,
    Quit,
    None,
}