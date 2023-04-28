pub trait Memory{
    fn push(&mut self, data:i32);
    fn pop(&mut self)->Option<i32>;
}