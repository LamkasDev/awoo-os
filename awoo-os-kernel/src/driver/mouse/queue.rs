use super::mouse::MouseInput;
use crate::driver::logger::logger::println;
use conquer_once::spin::OnceCell;
use core::{
    pin::Pin,
    task::{Context, Poll},
};
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream};

static MOUSE_INPUT_WAKER: AtomicWaker = AtomicWaker::new();
static MOUSE_INPUT_QUEUE: OnceCell<ArrayQueue<MouseInput>> = OnceCell::uninit();

/// Called by the mouse interrupt handler
///
/// Must not block or allocate.
pub(crate) fn add_mouse_input(input: MouseInput) {
    if let Ok(queue) = MOUSE_INPUT_QUEUE.try_get() {
        if let Err(_) = queue.push(input) {
            println("WARNING: mouse input queue full; dropping mouse input");
        } else {
            MOUSE_INPUT_WAKER.wake();
        }
    } else {
        println("WARNING: mouse input queue uninitialized");
    }
}

pub struct MouseInputStream {
    _private: (),
}

impl MouseInputStream {
    pub fn new() -> Self {
        MOUSE_INPUT_QUEUE
            .try_init_once(|| ArrayQueue::new(1024))
            .expect("MouseInputStream::new should only be called once");
        MouseInputStream { _private: () }
    }
}

impl Stream for MouseInputStream {
    type Item = MouseInput;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<MouseInput>> {
        let queue = MOUSE_INPUT_QUEUE
            .try_get()
            .expect("mouse input queue not initialized");

        // fast path
        if let Some(input) = queue.pop() {
            // println("popped mouse");
            return Poll::Ready(Some(input));
        }

        MOUSE_INPUT_WAKER.register(&cx.waker());
        match queue.pop() {
            Some(input) => {
                // println("popped mouse");
                MOUSE_INPUT_WAKER.take();
                Poll::Ready(Some(input))
            }
            None => Poll::Pending,
        }
    }
}
