use x86_64::structures::idt::InterruptStackFrame;

use crate::{int::int, pic::pic, print};

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(int::InterruptIndex::Timer.as_u8());
    }
}
