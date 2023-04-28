use super::memory::Memory;

pub struct Queue{
    stack: Vec<i32>
}   


impl Queue{
    pub fn new() -> Self{
        Self{
            stack: Vec::new()
        }
    }
}
impl Memory for Queue{
    fn push(&mut self, data: i32){
        self.stack.push(data);
    }

    fn pop(&mut self) -> Option<i32>{
        self.stack.pop()
    }
}