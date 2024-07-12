use crate::{
    driver::{keyboard::keyboard, shell::queue::println, timer::timer},
    gdt::tss,
    hlt_loop,
    int::int,
};
use alloc::format;
use lazy_static::lazy_static;
use x86_64::{registers::control::Cr2, structures::idt::{InterruptDescriptorTable, InterruptStackFrame}};
use x86_64::structures::idt::PageFaultErrorCode;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(tss::DOUBLE_FAULT_IST_INDEX);
        }
        idt[int::InterruptIndex::Timer.as_usize()].set_handler_fn(timer::timer_interrupt_handler);
        idt[int::InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::keyboard_interrupt_handler);
        idt[int::InterruptIndex::RTC.as_usize()].set_handler_fn(timer::rtc_interrupt_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    println("initialized IDT...");
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame));
}

extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println("EXCEPTION: PAGE FAULT");
    println(&format!("Accessed Address: {:?}", Cr2::read()));
    println(&format!("Error Code: {:?}", error_code));
    println(&format!("{:#?}", stack_frame));
    hlt_loop();
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
