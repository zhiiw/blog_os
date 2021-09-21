use core::{future::Future,pin::Pin};
use alloc::boxed::Box;
use pc_keyboard::KeyCode::B;
use core::task::Context;
pub mod simple_executor;
pub mod keyboard;

pub struct Task{
    future:Pin<Box<dyn Future<Output = ()>>>,
}
use core::task::Poll;
impl Task {
    pub fn new(future:impl Future<Output = ()>+'static)->Task{
        Task{
            future:Box::pin(future),
        }
    }
    fn poll(&mut self,context:&mut Context)->Poll<()>{
        self.future.as_mut().poll(context)
    }
}