use crate::{
    driver::{
        pic::pic::PICS,
        timer::timer::{common_timer_code, RATE},
    },
    int::int,
};
use x86_64::{
    instructions::port::{Port, PortGeneric, ReadWriteAccess},
    structures::idt::InterruptStackFrame,
};

pub fn init_rtc() {
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

pub unsafe fn send_rtc_eoi() {
    let mut rtc: PortGeneric<u8, ReadWriteAccess> = Port::new(0x70);
    let mut cmos: PortGeneric<u8, ReadWriteAccess> = Port::new(0x71);
    rtc.write(0x0C);
    cmos.read();
}

pub extern "x86-interrupt" fn rtc_interrupt_handler(_stack_frame: InterruptStackFrame) {
    common_timer_code();
    unsafe {
        send_rtc_eoi();
        PICS.lock()
            .notify_end_of_interrupt(int::InterruptIndex::Rtc.as_u8());
    }
}
