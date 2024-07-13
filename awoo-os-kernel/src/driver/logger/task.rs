use core::sync::atomic::Ordering;

use super::queue::{print_picture, println, LoggerStream};
use crate::{
    driver::framebuffer::framebuffer::FrameBufferWriter,
    memory::{
        heap::{HEAP_SIZE, HEAP_START},
        memory::PHYSICAL_MEMORY_OFFSET,
    },
};
use alloc::format;
use bootloader_api::info::{FrameBuffer, Optional};
use embedded_graphics::pixelcolor::RgbColor;
use futures_util::StreamExt;

pub const KITTY_PICTURE: &[u8] = include_bytes!("../../../assets/cat.tga");
pub const KITTY_PICTURE_2: &[u8] = include_bytes!("../../../assets/cat_2.tga");

pub async fn logging_task(frame_buffer_optional: &'static mut Optional<FrameBuffer>) {
    // Free the wrapped framebuffer from the FFI-safe abstraction provided by bootloader_api
    let frame_buffer_option = frame_buffer_optional.as_mut();

    // Unwrap the framebuffer
    let frame_buffer_struct = frame_buffer_option.unwrap();

    // Extract the framebuffer info and, to satisfy the borrow checker, clone it
    let frame_buffer_info = frame_buffer_struct.info().clone();

    // Get the framebuffer's mutable raw byte slice
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();

    // Construct framebuffer writer
    let mut framebuffer = FrameBufferWriter::new(raw_frame_buffer, frame_buffer_info);

    // Setup environment
    let mut stream = LoggerStream::new();
    print_picture(KITTY_PICTURE);
    println("hello from awoo-os!");
    println(&format!(
        "physical memory is mapped at {:#012x}..",
        PHYSICAL_MEMORY_OFFSET.load(Ordering::SeqCst)
    ));
    println(&format!(
        "initialized heap at {:#012x} ({} bytes)...",
        HEAP_START, HEAP_SIZE
    ));

    // Run task
    while let Some(action) = stream.next().await {
        match action.text {
            Some(v) => {
                for c in v {
                    framebuffer.write_char(c);
                }
            }
            None => {}
        }
        match action.image {
            Some(v) => {
                for ele in v {
                    framebuffer.write_pixel(
                        ele.0.x as usize,
                        ele.0.y as usize,
                        ele.1.r(),
                        ele.1.g(),
                        ele.1.b(),
                    );
                }
            }
            None => {}
        }
    }
}
