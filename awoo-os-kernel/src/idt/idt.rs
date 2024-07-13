use crate::{
    driver::{
        apic::apic::{apic_err_handler, apic_si_handler, apic_test_handler},
        apic_timer::apic_timer::apic_timer_handler,
        keyboard::keyboard::keyboard_interrupt_handler,
        logger::queue::println,
        rtc::rtc::rtc_interrupt_handler,
        timer::timer::timer_interrupt_handler,
    },
    gdt::tss::DOUBLE_FAULT_IST_INDEX,
    int::{
        exceptions::{
            alignment_check_handler, bound_range_exceeded_handler, breakpoint_handler,
            debug_exception_handler, device_not_available_handler, divide_error_handler,
            double_fault_handler, general_protection_fault_handler, invalid_opcode_handler,
            invalid_tss_handler, machine_check_handler, overflow_handler, page_fault_handler,
            segment_not_present_handler, simd_floating_point_handler, stack_segment_fault_handler,
            virtualization_handler, x87_floating_point_handler,
        },
        int::InterruptIndex,
    },
};
use lazy_static::lazy_static;
use x86_64::structures::idt::InterruptDescriptorTable;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        // Standart x86_64 ISA exceptions
        let mut idt = InterruptDescriptorTable::new();
        idt.divide_error.set_handler_fn(divide_error_handler);
        idt.debug.set_handler_fn(debug_exception_handler);
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.overflow.set_handler_fn(overflow_handler);
        idt.bound_range_exceeded
            .set_handler_fn(bound_range_exceeded_handler);
        idt.invalid_opcode.set_handler_fn(invalid_opcode_handler);
        idt.device_not_available
            .set_handler_fn(device_not_available_handler);
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt.invalid_tss.set_handler_fn(invalid_tss_handler);
        idt.segment_not_present
            .set_handler_fn(segment_not_present_handler);
        idt.stack_segment_fault
            .set_handler_fn(stack_segment_fault_handler);
        idt.general_protection_fault
            .set_handler_fn(general_protection_fault_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        idt.x87_floating_point
            .set_handler_fn(x87_floating_point_handler);
        idt.alignment_check.set_handler_fn(alignment_check_handler);
        idt.machine_check.set_handler_fn(machine_check_handler);
        idt.simd_floating_point
            .set_handler_fn(simd_floating_point_handler);
        idt.virtualization.set_handler_fn(virtualization_handler);

        // External interrupts for PIC
        idt[InterruptIndex::Timer.as_usize()].set_handler_fn(timer_interrupt_handler);
        idt[InterruptIndex::Keyboard.as_usize()].set_handler_fn(keyboard_interrupt_handler);
        idt[InterruptIndex::Rtc.as_usize()].set_handler_fn(rtc_interrupt_handler);

        // External interrupts for APIC
        idt[InterruptIndex::LapicKeyboard.as_usize()].set_handler_fn(apic_test_handler);
        idt[InterruptIndex::LapicSpurrious.as_usize()].set_handler_fn(apic_si_handler);
        idt[InterruptIndex::LapicError.as_usize()].set_handler_fn(apic_err_handler);
        idt[InterruptIndex::LapicTimer.as_usize()].set_handler_fn(apic_timer_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    println("initialized IDT...");
}
