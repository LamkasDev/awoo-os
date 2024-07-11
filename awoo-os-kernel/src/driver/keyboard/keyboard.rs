use crate::{int::int, pic::pic};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};
use super::queue;

lazy_static! {
    static ref KEYBOARD: Mutex<Keyboard<layouts::De105Key, ScancodeSet1>> =
        Mutex::new(Keyboard::new(
            ScancodeSet1::new(),
            layouts::De105Key,
            HandleControl::Ignore
        ));
}

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    queue::add_scancode(scancode);

    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(int::InterruptIndex::Keyboard.as_u8());
    }
}
