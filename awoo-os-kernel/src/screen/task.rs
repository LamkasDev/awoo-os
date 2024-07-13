use core::cmp::max;

use super::queue::ScreenStream;
use crate::{
    driver::{
        framebuffer::framebuffer::FrameBufferWriter,
        logger::logger::{println, LOGGER_LINES},
        mouse::mouse::MOUSE_POSITION,
        timer::timer::get_total_ms,
    },
    memory::{
        heap::{HEAP_SIZE, HEAP_START},
        memory::PHYSICAL_MEMORY_OFFSET,
    },
};
use alloc::format;
use bootloader_api::info::{FrameBuffer, Optional};
use futures_util::StreamExt;

pub const KITTY_PICTURE: &[u8] = include_bytes!("../../assets/cat.tga");
pub const KITTY_PICTURE_2: &[u8] = include_bytes!("../../assets/cat_2.tga");

pub async fn screen_task(frame_buffer_optional: &'static mut Optional<FrameBuffer>) {
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
    let mut stream = ScreenStream::new();
    // print_picture(KITTY_PICTURE);
    println("hello from awoo-os!");
    println(&format!(
        "physical memory is mapped at {:#012x}..",
        *PHYSICAL_MEMORY_OFFSET.lock()
    ));
    println(&format!(
        "initialized heap at {:#012x} ({} bytes)...",
        HEAP_START, HEAP_SIZE
    ));

    // Run task
    while let Some(_ticks) = stream.next().await {
        let start = get_total_ms();
        framebuffer.clear();
        let lines = LOGGER_LINES.lock();
        for line in lines.iter() {
            framebuffer.write_str(line);
            framebuffer.newline();
        }
        drop(lines);
        let pos = MOUSE_POSITION.lock();
        framebuffer.write_pixel(pos.x as usize, pos.y as usize, 255, 0, 0);
        framebuffer.swap_buffer();
        let end = get_total_ms();
        println(&format!("frame took {} ms", end - start));
    }
}
