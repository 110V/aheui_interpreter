use super::Memory;


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
    fn len(&self) -> usize{
        self.stack.len()
    }
    fn print(&self){
        println!("Stack: {:?}",self.stack);
    }
    fn swap(&mut self){
        let stack = &mut self.stack;
        if stack.len() < 2{
            return;
        }
        let l = stack.len();
        stack.swap(l-1,l-2);
    }

}