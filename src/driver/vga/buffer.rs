use crate::driver::vga;
use volatile::Volatile;

pub const VGA_BUFFER_HEIGHT: usize = 25;
pub const VGA_BUFFER_WIDTH: usize = 80;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct VgaScreenChar {
    pub ascii_character: u8,
    pub color_code: vga::color::VgaColorCode,
}

#[repr(transparent)]
pub struct VgaBuffer {
    pub chars: [[Volatile<VgaScreenChar>; VGA_BUFFER_WIDTH]; VGA_BUFFER_HEIGHT],
}
