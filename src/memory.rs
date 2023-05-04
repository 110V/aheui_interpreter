mod stack;
mod queue;

pub use stack::Stack;
pub use queue::Queue;


pub trait Memory{
    fn push(&mut self, data:i32);
    fn pop(&mut self)->Option<i32>;
    fn len(&self)->usize;
    fn swap(&mut self);
    fn dup(&mut self);
    fn print(&self);
}