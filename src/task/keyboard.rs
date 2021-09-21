use conquer_once::spin::OnceCell;
use crossbeam_queue::ArrayQueue;
use crate::println;
use core::result::Result::Ok;
use core::{pin::Pin, task::{Poll, Context}};
use futures_util::stream::Stream;
use core::stream::Stream;

static SCANCODE_QUEUE: OnceCell<ArrayQueue<u8>> = OnceCell::uninit();

pub(crate) fn add_scancode(scancode:u8){
    if let Ok(queue) =SCANCODE_QUEUE.try_get(){
        if let Err(_) = queue.push(scancode) {
            println!("WARNING: scancode queue full; dropping keyboard input");
        }else {
            println!("WARNING: scancode queue uninitialized");

        }
    }
}

pub struct ScancodeStream{
    _private:(),
}

impl ScancodeStream{
    pub fn new() -> Self {
        SCANCODE_QUEUE.try_init_once(||ArrayQueue::new(100)).expect("ScancodeStream::new should only be called once");
        ScancodeStream{_private:()}
    }
}

impl Stream for ScancodeStream{
    type Item = u8;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u8>> {
        let queue = SCANCODE_QUEUE.try_get().expect("not initialized");
        match queue.pop() {
            Ok(scancode)=>Poll::Ready(Some(scancode)),
            Err(crossbeam_queue::PopError)=>Poll::Pending,
        }
    }
}