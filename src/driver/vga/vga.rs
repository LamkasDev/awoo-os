use super::{buffer, color};
use core::fmt;
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::instructions::interrupts;

lazy_static! {
    pub static ref VGA_WRITER: Mutex<VgaWriter> = Mutex::new(VgaWriter {
        column_position: 0,
        color_code: color::VgaColorCode::new(color::VgaColor::Yellow, color::VgaColor::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut buffer::VgaBuffer) },
    });
}

pub struct VgaWriter {
    pub column_position: usize,
    pub color_code: color::VgaColorCode,
    pub buffer: &'static mut buffer::VgaBuffer,
}

impl VgaWriter {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= buffer::VGA_BUFFER_WIDTH {
                    self.new_line();
                }

                let row = buffer::VGA_BUFFER_HEIGHT - 1;
                let col = self.column_position;
                let color_code = self.color_code;
                self.buffer.chars[row][col].write(buffer::VgaScreenChar {
                    ascii_character: byte,
                    color_code,
                });
                self.column_position += 1;
            }
        }
    }

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                // printable ASCII byte or newline
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                // not part of printable ASCII range
                _ => self.write_byte(0xfe),
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..buffer::VGA_BUFFER_HEIGHT {
            for col in 0..buffer::VGA_BUFFER_WIDTH {
                let character = self.buffer.chars[row][col].read();
                self.buffer.chars[row - 1][col].write(character);
            }
        }
        self.clear_row(buffer::VGA_BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = buffer::VgaScreenChar {
            ascii_character: b' ',
            color_code: self.color_code,
        };
        for col in 0..buffer::VGA_BUFFER_WIDTH {
            self.buffer.chars[row][col].write(blank);
        }
    }
}

impl fmt::Write for VgaWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::driver::vga::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    interrupts::without_interrupts(|| {
        VGA_WRITER.lock().write_fmt(args).unwrap();
    });
}
