use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream};

use crate::driver::logger::logger::println;

static SCREEN_WAKER: AtomicWaker = AtomicWaker::new();
static SCREEN_QUEUE: OnceCell<ArrayQueue<u64>> = OnceCell::uninit();

pub fn trigger_screen_frame(ticks: u64) {
    if let Ok(queue) = SCREEN_QUEUE.try_get() {
        if let Err(_) = queue.push(ticks) {
            println("WARNING: screen queue full; dropping frame");
        } else {
            SCREEN_WAKER.wake();
        }
    } else {
        println("WARNING: screen queue uninitialized");
    }
}

pub struct ScreenStream {
    _private: (),
}

impl ScreenStream {
    pub fn new() -> Self {
        SCREEN_QUEUE
            .try_init_once(|| ArrayQueue::new(512))
            .expect("ScreenStream::new should only be called once");
        ScreenStream { _private: () }
    }
}

impl Stream for ScreenStream {
    type Item = u64;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<u64>> {
        let queue = SCREEN_QUEUE
            .try_get()
            .expect("screen queue not initialized");

        // fast path
        if let Some(ticks) = queue.pop() {
            // println("popped screen");
            return Poll::Ready(Some(ticks));
        }

        SCREEN_WAKER.register(&cx.waker());
        match queue.pop() {
            Some(ticks) => {
                // println("popped screen");
                SCREEN_WAKER.take();
                Poll::Ready(Some(ticks))
            }
            None => Poll::Pending,
        }
    }
}
