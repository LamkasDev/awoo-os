#[cfg(test)]
use crate::driver::vga::buffer;
#[cfg(test)]
use crate::driver::vga::vga;
#[cfg(test)]
use crate::println;
#[cfg(test)]
use core::fmt::Write;
#[cfg(test)]
use x86_64::instructions::interrupts;

#[test_case]
fn test_println_simple() {
    println!("test_println_simple output");
}

#[test_case]
fn test_println_many() {
    for _ in 0..200 {
        println!("test_println_many output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    interrupts::without_interrupts(|| {
        let mut writer = vga::VGA_WRITER.lock();
        writeln!(writer, "\n{}", s).expect("writeln failed");
        for (i, c) in s.chars().enumerate() {
            let screen_char = writer.buffer.chars[buffer::VGA_BUFFER_HEIGHT - 2][i].read();
            assert_eq!(char::from(screen_char.ascii_character), c);
        }
    });
}
