use super::Memory;

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
    fn len(&self) -> usize{
        self.queue.len()
    }
    fn print(&self){
        println!("Queue: {:?}",self.queue);
    }
    fn swap(&mut self){
        let queue = &mut self.queue;
        if queue.len() < 2{
            return;
        }
        queue.swap(0,1);
    }
    fn dup(&mut self){
        let queue = &mut self.queue;
        if queue.len() < 1{
            return;
        }
        let data = queue[0];
        queue.insert(0,data);
    }
}