use crate::driver::framebuffer::framebuffer::FrameBufferWriter;
use alloc::vec::Vec;
use bootloader_api::info::FrameBufferInfo;
use conquer_once::spin::OnceCell;
use core::fmt::Write;
use embedded_graphics::{
    geometry::Dimensions,
    pixelcolor::{Rgb888, RgbColor},
};
use tinytga::Tga;

pub const KITTY_PICTURE: &[u8; 63044] = include_bytes!("../../../assets/cat.tga");

pub fn print_kitty(buffer: &'static mut [u8], info: FrameBufferInfo) {
    let mut framebuffer = FrameBufferWriter::new(buffer, info);
    let img: Tga<Rgb888> = Tga::from_slice(KITTY_PICTURE).unwrap();
    let pixels: Vec<_> = img.pixels().collect();
    for ele in pixels {
        framebuffer.set_pixel(
            ele.0.x as usize,
            ele.0.y as usize,
            ele.1.r(),
            ele.1.g(),
            ele.1.b(),
        );
    }
    framebuffer.y_pos += img.bounding_box().size.height as usize;

    let _ = framebuffer.write_str("hewwo !!!");
    let _ = framebuffer.write_str("this is a very cool os :3");
}
