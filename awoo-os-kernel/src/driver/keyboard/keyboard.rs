use super::queue;
use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::instructions::port::Port;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::De105Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::De105Key,
            HandleControl::Ignore
        ));
}

pub fn common_keyboard_code() {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    queue::add_scancode(scancode);
}
