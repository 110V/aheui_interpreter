mod stack;
mod queue;

pub use stack::Stack;
pub use queue::Queue;

pub trait Memory{
    fn push(&mut self, data:i32);
    fn pop(&mut self)->Option<i32>;
}