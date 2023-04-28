use super::memory::Memory;

pub struct Queue{
    queue: Vec<i32>
}   


impl Queue{
    pub fn new() -> Self{
        Self{
            queue: Vec::new()
        }
    }
}
impl Memory for Queue{
    fn push(&mut self, data: i32){
        self.queue.push(data);
    }

    fn pop(&mut self) -> Option<i32>{
        let queue = &mut self.queue; 
        if queue.is_empty(){
            return None;
        }
        Some(queue.remove(0))
    }
}