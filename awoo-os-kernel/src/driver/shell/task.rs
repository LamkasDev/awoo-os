use crate::{driver::framebuffer::framebuffer::FrameBufferWriter, memory::heap::{HEAP_SIZE, HEAP_START}};
use alloc::format;
use bootloader_api::info::{FrameBuffer, Optional};
use embedded_graphics::{geometry::Dimensions, pixelcolor::RgbColor};
use futures_util::StreamExt;
use tinytga::Tga;
use super::queue::{print_picture, println, LoggerStream};

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
    println("Hello from awoo-os!");
    println(&format!("initialized heap with {} bytes at {:#012x}.", HEAP_SIZE, HEAP_START));

    // Run task
    while let Some(action) = stream.next().await {
        match action.text {
            Some(v) => {
                for c in v{
                    framebuffer.write_char(c);
                }
            }
            None => {}
        }
        match action.image {
            Some(v) => {
                for ele in v {
                    framebuffer.set_pixel(
                        ele.0.x as usize,
                        ele.0.y as usize,
                        ele.1.r(),
                        ele.1.g(),
                        ele.1.b(),
                    );
                }
                framebuffer.y_pos = action.image_size.unwrap().1 as usize
            }
            None => {}
        }
    }
}
