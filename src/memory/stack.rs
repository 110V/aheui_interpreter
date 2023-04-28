use super::memory::Memory;

pub struct Stack{
    stack: Vec<i32>
}   


impl Stack{
    pub fn new() -> Self{
        Self{
            stack: Vec::new()
        }
    }
}
impl Memory for Stack{
    fn push(&mut self, data: i32){
        self.stack.push(data);
    }

    fn pop(&mut self) -> Option<i32>{
        self.stack.pop()
    }
}