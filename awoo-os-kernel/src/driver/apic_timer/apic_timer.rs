use crate::driver::{apic::apic::send_apic_eoi, timer::timer::common_timer_code};
use x86_64::structures::idt::InterruptStackFrame;

pub extern "x86-interrupt" fn apic_timer_handler(_stack_frame: InterruptStackFrame) {
    common_timer_code();
    unsafe { send_apic_eoi() };
}
