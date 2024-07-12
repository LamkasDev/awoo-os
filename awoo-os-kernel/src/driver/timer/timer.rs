use core::sync::atomic::{AtomicU64, Ordering};

use embedded_graphics::geometry::Dimensions;
use tinytga::Tga;
use x86_64::{instructions::port::{Port, PortGeneric, ReadWriteAccess}, structures::idt::InterruptStackFrame};
use crate::{driver::shell::{queue::print_picture, task::{KITTY_PICTURE, KITTY_PICTURE_2}}, int::int, pic::pic};

pub static TIME: AtomicU64 = AtomicU64::new(0);
pub static TICKS: AtomicU64 = AtomicU64::new(0);
pub const RATE: u8 = 9;
pub const TICKS_PER_SECOND: u64 = 32768 >> (RATE as u64 - 1);
pub const TICKS_PER_SECOND_F: f64 = TICKS_PER_SECOND as f64;

pub extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    // print!(".");
    unsafe {
        pic::PICS
            .lock()
            .notify_end_of_interrupt(int::InterruptIndex::Timer.as_u8());
    }
}

pub extern "x86-interrupt" fn rtc_interrupt_handler(_stack_frame: InterruptStackFrame) {
    let t = TICKS.load(Ordering::SeqCst);
    match t.checked_add(1) {
        Some(v) => {
            if v % TICKS_PER_SECOND == 0 {
                TIME.fetch_add(1, Ordering::SeqCst);
                if TIME.load(Ordering::SeqCst) % 2 == 0 {
                    print_picture(KITTY_PICTURE);
                } else {
                    print_picture(KITTY_PICTURE_2);
                }
            }
            TICKS.store(v, Ordering::SeqCst);
        }
        None => {
            TICKS.store(0, Ordering::SeqCst);
        }
    }
    unsafe {
        clean_rtc();
        pic::PICS
            .lock()
            .notify_end_of_interrupt(int::InterruptIndex::RTC.as_u8());
    }
}

pub fn get_ms() -> u64 {
    return (((TICKS.load(Ordering::SeqCst) % TICKS_PER_SECOND) as f64 / TICKS_PER_SECOND_F) * 1000.0) as u64;
}

pub unsafe fn clean_rtc() {
    let mut rtc: PortGeneric<u8, ReadWriteAccess> = Port::new(0x70);
    let mut cmos: PortGeneric<u8, ReadWriteAccess> = Port::new(0x71);
    rtc.write(0x0C);
    cmos.read();
}

pub fn setup_rtc() {
    unsafe {
        // Writing to RTC port selects a register used by CMOS, where we then write data
        let mut rtc: PortGeneric<u8, ReadWriteAccess> = Port::new(0x70);
        let mut cmos: PortGeneric<u8, ReadWriteAccess> = Port::new(0x71);

        // Enable IRQ8
        rtc.write(0x8B);
        let prev = cmos.read();
        rtc.write(0x8B);
        cmos.write(prev | 0x40);

        // Setup rate
        rtc.write(0x8A);
        let prev = cmos.read();
        rtc.write(0x8A);
        cmos.write((prev & 0xF0) | RATE);
        
        /* let prev = rtc.read();
        rtc.write(prev & 0x7F);
        let _ = cmos.read(); */
    }
}
