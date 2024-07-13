use alloc::{format, vec::Vec};
use conquer_once::spin::OnceCell;
use embedded_graphics::{geometry::Dimensions, pixelcolor::Rgb888, Pixel};
use tinytga::Tga;
use core::{
    pin::Pin, sync::atomic::Ordering, task::{Context, Poll}
};
use crossbeam_queue::ArrayQueue;
use futures_util::{task::AtomicWaker, Stream};
use crate::driver::timer::timer::{get_ms, TIME};

pub struct LoggerAction {
    pub text: Option<Vec<char>>,
    pub image: Option<Vec<Pixel<Rgb888>>>,
    pub image_size: Option<(u32, u32)>
}

impl LoggerAction {
    pub fn new(text: Option<Vec<char>>, image: Option<Vec<Pixel<Rgb888>>>, image_size: Option<(u32, u32)>) -> Self {
        let action = Self {
            text: text,
            image: image,
            image_size: image_size
        };
        action
    }
}

static LOGGER_WAKER: AtomicWaker = AtomicWaker::new();
static LOGGER_QUEUE: OnceCell<ArrayQueue<LoggerAction>> = OnceCell::uninit();

pub fn println(s: &str) {
    print(&format!("[{:04}:{:03}] ", TIME.load(Ordering::SeqCst), get_ms()));
    print(s);
    print("\n");
}

pub fn print(s: &str) {
    print_raw(s.chars().collect());
}

pub fn print_raw(text: Vec<char>) {
    if let Ok(queue) = LOGGER_QUEUE.try_get() {
        if let Err(_) = queue.push(LoggerAction::new(Some(text), None, None)) {
            println("WARNING: logger queue full; dropping log output");
        } else {
            LOGGER_WAKER.wake();
        }
    } else {
        println("WARNING: logger queue uninitialized");
    }
}

pub fn print_picture(data: &[u8]) {
    let picture = Tga::from_slice(data).unwrap();
    if let Ok(queue) = LOGGER_QUEUE.try_get() {
        if let Err(_) = queue.push(LoggerAction::new(None, Some(picture.pixels().collect()), Some((picture.bounding_box().size.width, picture.bounding_box().size.height)))) {
            println("WARNING: logger queue full; dropping log output");
        } else {
            LOGGER_WAKER.wake();
        }
    } else {
        println("WARNING: logger queue uninitialized");
    }
}

pub struct LoggerStream {
    _private: (),
}

impl LoggerStream {
    pub fn new() -> Self {
        LOGGER_QUEUE
            .try_init_once(|| ArrayQueue::new(512))
            .expect("LoggerStream::new should only be called once");
        LoggerStream { _private: () }
    }
}

impl Stream for LoggerStream {
    type Item = LoggerAction;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context) -> Poll<Option<LoggerAction>> {
        let queue = LOGGER_QUEUE
            .try_get()
            .expect("logger queue not initialized");

        // fast path
        if let Some(action) = queue.pop() {
            return Poll::Ready(Some(action));
        }

        LOGGER_WAKER.register(&cx.waker());
        match queue.pop() {
            Some(action) => {
                LOGGER_WAKER.take();
                Poll::Ready(Some(action))
            }
            None => Poll::Pending,
        }
    }
}
