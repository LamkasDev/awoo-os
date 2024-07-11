use crate::driver::shell::shell::{print_kitty, KITTY_PICTURE};
use alloc::vec::Vec;
use bootloader_api::info::{FrameBuffer, FrameBufferInfo, Optional};
use bootloader_x86_64_common::logger::LockedLogger;
use conquer_once::spin::OnceCell;
use embedded_graphics::pixelcolor::Rgb888;
use tinytga::{RawTga, Tga};

pub(crate) static LOGGER: OnceCell<LockedLogger> = OnceCell::uninit();

pub(crate) fn init_logger(frame_buffer_optional: &'static mut Optional<FrameBuffer>) {
    // free the wrapped framebuffer from the FFI-safe abstraction provided by bootloader_api
    let frame_buffer_option = frame_buffer_optional.as_mut();

    // unwrap the framebuffer
    let frame_buffer_struct = frame_buffer_option.unwrap();

    // extract the framebuffer info and, to satisfy the borrow checker, clone it
    let frame_buffer_info = frame_buffer_struct.info().clone();

    // get the framebuffer's mutable raw byte slice
    let raw_frame_buffer = frame_buffer_struct.buffer_mut();

    // finally, initialize the logger using the last two variables
    //init_logger_raw(raw_frame_buffer, frame_buffer_info);
    print_kitty(raw_frame_buffer, frame_buffer_info);
}

pub(crate) fn init_logger_raw(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let logger = LOGGER.get_or_init(move || LockedLogger::new(buffer, info, true, false));
    log::set_logger(logger).expect("Logger already set");
    log::set_max_level(log::LevelFilter::Trace);
    log::info!("Hello, Kernel Mode!");
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => (log::info!($($arg)*));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
