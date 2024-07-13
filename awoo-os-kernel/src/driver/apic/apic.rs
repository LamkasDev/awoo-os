use crate::{
    driver::{logger::logger::println, pic::pic::PICS},
    int::int::InterruptIndex,
    memory::memory::PHYSICAL_MEMORY_OFFSET,
};
use core::ptr::write_volatile;
use x2apic::lapic::LocalApicBuilder;
use x86_64::structures::idt::InterruptStackFrame;

pub unsafe fn init_apic(address: u64) {
    PICS.lock().disable();
    let mut lapic = LocalApicBuilder::new()
        .set_xapic_base(address)
        .timer_vector(InterruptIndex::LocalApicTimer as usize)
        .error_vector(InterruptIndex::LocalApicError as usize)
        .spurious_vector(InterruptIndex::LocalApicSpurrious as usize)
        .build()
        .unwrap_or_else(|err| panic!("{}", err));
    lapic.enable();
    println("initialized APIC...");
}

pub unsafe fn send_apic_eoi() {
    let address = (*PHYSICAL_MEMORY_OFFSET.lock() + 0xFEE00000 + 0xB0) as *mut u32;
    write_volatile(address, 0);
}

pub extern "x86-interrupt" fn apic_si_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SI\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn apic_err_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: LAPIC ERR\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn apic_2_handler(_stack_frame: InterruptStackFrame) {
    println("2");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_3_handler(_stack_frame: InterruptStackFrame) {
    println("3");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_4_handler(_stack_frame: InterruptStackFrame) {
    println("4");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_5_handler(_stack_frame: InterruptStackFrame) {
    println("5");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_6_handler(_stack_frame: InterruptStackFrame) {
    println("6");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_7_handler(_stack_frame: InterruptStackFrame) {
    println("7");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_8_handler(_stack_frame: InterruptStackFrame) {
    println("8");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_9_handler(_stack_frame: InterruptStackFrame) {
    println("9");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_10_handler(_stack_frame: InterruptStackFrame) {
    println("10");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_11_handler(_stack_frame: InterruptStackFrame) {
    println("11");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_12_handler(_stack_frame: InterruptStackFrame) {
    println("12");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_13_handler(_stack_frame: InterruptStackFrame) {
    println("13");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_14_handler(_stack_frame: InterruptStackFrame) {
    println("14");
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn apic_15_handler(_stack_frame: InterruptStackFrame) {
    println("15");
    unsafe { send_apic_eoi() };
}
