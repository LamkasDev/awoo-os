use crate::{
    driver::{
        apic::apic::{
            apic_10_handler, apic_11_handler, apic_13_handler, apic_14_handler, apic_15_handler,
            apic_2_handler, apic_3_handler, apic_4_handler, apic_5_handler, apic_6_handler,
            apic_7_handler, apic_8_handler, apic_9_handler, apic_err_handler, apic_si_handler,
        },
        apic_timer::apic_timer::apic_timer_handler,
        ioapic::ioapic::{io_apic_keyboard_handler, io_apic_mouse_handler, io_apic_timer_handler},
        logger::logger::println,
        pic::pic::{pic_keyboard_handler, pic_mouse_handler, pic_timer_handler},
        rtc::rtc::rtc_interrupt_handler,
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
        idt[InterruptIndex::Timer.as_u8()].set_handler_fn(pic_timer_handler);
        idt[InterruptIndex::Keyboard.as_u8()].set_handler_fn(pic_keyboard_handler);
        idt[InterruptIndex::Rtc.as_u8()].set_handler_fn(rtc_interrupt_handler);
        idt[InterruptIndex::Mouse.as_u8()].set_handler_fn(pic_mouse_handler);

        // External interrupts for APIC
        idt[InterruptIndex::LocalApicSpurrious.as_u8()].set_handler_fn(apic_si_handler);
        idt[InterruptIndex::LocalApicError.as_u8()].set_handler_fn(apic_err_handler);
        idt[InterruptIndex::LocalApicTimer.as_u8()].set_handler_fn(apic_timer_handler);

        // External interrupts for IO APIC
        idt[InterruptIndex::IoApicTimer.as_u8()].set_handler_fn(io_apic_timer_handler);
        idt[InterruptIndex::IoApicKeyboard.as_u8()].set_handler_fn(io_apic_keyboard_handler);
        idt[InterruptIndex::IoApic2.as_u8()].set_handler_fn(apic_2_handler);
        idt[InterruptIndex::IoApic3.as_u8()].set_handler_fn(apic_3_handler);
        idt[InterruptIndex::IoApic4.as_u8()].set_handler_fn(apic_4_handler);
        idt[InterruptIndex::IoApic5.as_u8()].set_handler_fn(apic_5_handler);
        idt[InterruptIndex::IoApic6.as_u8()].set_handler_fn(apic_6_handler);
        idt[InterruptIndex::IoApic7.as_u8()].set_handler_fn(apic_7_handler);
        idt[InterruptIndex::IoApic8.as_u8()].set_handler_fn(apic_8_handler);
        idt[InterruptIndex::IoApic9.as_u8()].set_handler_fn(apic_9_handler);
        idt[InterruptIndex::IoApic10.as_u8()].set_handler_fn(apic_10_handler);
        idt[InterruptIndex::IoApic11.as_u8()].set_handler_fn(apic_11_handler);
        idt[InterruptIndex::IoApic12.as_u8()].set_handler_fn(io_apic_mouse_handler);
        idt[InterruptIndex::IoApic13.as_u8()].set_handler_fn(apic_13_handler);
        idt[InterruptIndex::IoApic14.as_u8()].set_handler_fn(apic_14_handler);
        idt[InterruptIndex::IoApic15.as_u8()].set_handler_fn(apic_15_handler);
        idt
    };
}

pub fn init_idt() {
    IDT.load();
    println("initialized IDT...");
}
