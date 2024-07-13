use crate::{
    driver::{
        logger::{
            queue::{print_picture, println},
            task::{KITTY_PICTURE, KITTY_PICTURE_2},
        },
        pic::pic::PICS,
    },
    int::int,
};
use alloc::format;
use core::sync::atomic::{AtomicU64, Ordering};
use x86_64::structures::idt::InterruptStackFrame;

pub static TIME: AtomicU64 = AtomicU64::new(0);
pub static TICKS: AtomicU64 = AtomicU64::new(0);

pub const RATE: u8 = 9;
pub const TICKS_PER_SECOND: u64 = 32768 >> (RATE as u64 - 1);
pub const TICKS_PER_SECOND_F: f64 = TICKS_PER_SECOND as f64;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(int::InterruptIndex::Timer.as_u8());
    }
}

pub fn common_timer_code() {
    let ticks = TICKS.load(Ordering::SeqCst);
    match ticks.checked_add(1) {
        Some(ticks) => {
            if ticks % TICKS_PER_SECOND == 0 {
                let time = TIME.fetch_add(1, Ordering::SeqCst);
                if time % 2 == 0 {
                    print_picture(KITTY_PICTURE);
                } else {
                    print_picture(KITTY_PICTURE_2);
                }
                println(&format!("time check: {} seconds / {} ticks", time, ticks));
            }
            TICKS.store(ticks, Ordering::SeqCst);
        }
        None => {
            TICKS.store(0, Ordering::SeqCst);
        }
    }
}

pub fn get_ms() -> u64 {
    return (((TICKS.load(Ordering::SeqCst) % TICKS_PER_SECOND) as f64 / TICKS_PER_SECOND_F)
        * 1000.0) as u64;
}
