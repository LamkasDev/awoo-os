use crate::{
    driver::{
        keyboard::keyboard::common_keyboard_code, logger::logger::println,
        mouse::mouse::common_mouse_code,
    },
    int::int::InterruptIndex,
};
use pic8259::ChainedPics;
use spin;
use x86_64::structures::idt::InterruptStackFrame;

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

pub fn init_pics() {
    unsafe { PICS.lock().initialize() };
    println("initialized PIC...");
}

pub extern "x86-interrupt" fn pic_timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

pub extern "x86-interrupt" fn pic_keyboard_handler(_stack_frame: InterruptStackFrame) {
    common_keyboard_code();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

pub extern "x86-interrupt" fn pic_mouse_handler(_stack_frame: InterruptStackFrame) {
    common_mouse_code();
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}
