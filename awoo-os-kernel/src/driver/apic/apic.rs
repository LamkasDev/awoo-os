use core::{ptr::write_volatile, sync::atomic::Ordering};

use crate::{
    driver::{keyboard::queue, logger::queue::println, pic::pic::PICS},
    int::int::InterruptIndex,
    memory::memory::PHYSICAL_MEMORY_OFFSET,
};
use x2apic::lapic::LocalApicBuilder;
use x86_64::{instructions::port::Port, structures::idt::InterruptStackFrame};

pub const APIC_OFFSET: u8 = 64;

pub unsafe fn init_apic(address: u64) {
    PICS.lock().disable();
    let mut lapic = LocalApicBuilder::new()
        .set_xapic_base(address)
        .timer_vector(InterruptIndex::LapicTimer as usize)
        .error_vector(InterruptIndex::LapicError as usize)
        .spurious_vector(InterruptIndex::LapicSpurrious as usize)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));
    lapic.enable();
    println("initialized APIC...");
}

pub unsafe fn send_apic_eoi() {
    let address = (PHYSICAL_MEMORY_OFFSET.load(Ordering::SeqCst) + 0xFEE00000 + 0xB0) as *mut u32;
    write_volatile(address, 0);
}

pub extern "x86-interrupt" fn apic_si_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SI\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn apic_err_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: LAPIC ERR\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn apic_test_handler(_stack_frame: InterruptStackFrame) {
    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };
    queue::add_scancode(scancode);

    unsafe { send_apic_eoi() };
}
