use crate::{driver::logger::logger::println, hlt_loop};
use alloc::format;
use x86_64::{
    registers::control::Cr2,
    structures::idt::{InterruptStackFrame, PageFaultErrorCode},
};

pub extern "x86-interrupt" fn divide_error_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: DIVIDE ERROR\n{:#?}", stack_frame));
}

pub extern "x86-interrupt" fn debug_exception_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: DEBUG EXCEPTION\n{:#?}", stack_frame));
}

pub extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame));
}

pub extern "x86-interrupt" fn overflow_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: OVERFLOW\n{:#?}", stack_frame));
}

pub extern "x86-interrupt" fn bound_range_exceeded_handler(stack_frame: InterruptStackFrame) {
    println(&format!(
        "EXCEPTION: BOUND RANGE EXCEEDED\n{:#?}",
        stack_frame
    ));
}

pub extern "x86-interrupt" fn invalid_opcode_handler(stack_frame: InterruptStackFrame) {
    println(&format!("EXCEPTION: INVALID OPCODE\n{:#?}", stack_frame));
}

pub extern "x86-interrupt" fn device_not_available_handler(stack_frame: InterruptStackFrame) {
    println(&format!(
        "EXCEPTION: DEVICE NOT AVAILABLE\n{:#?}",
        stack_frame
    ));
}

pub extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn invalid_tss_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: INVALID TSS\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn segment_not_present_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: SEGMENT NOT PRESENT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn stack_segment_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: STACK SEGMENT FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn general_protection_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: GENERAL PROTECTION FAULT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    println("EXCEPTION: PAGE FAULT");
    println(&format!("Accessed Address: {:?}", Cr2::read()));
    println(&format!("Error Code: {:?}", error_code));
    println(&format!("{:#?}", stack_frame));
    hlt_loop();
}

pub extern "x86-interrupt" fn x87_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: X87 FLOATING POINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn alignment_check_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) {
    panic!("EXCEPTION: ALIGNMENT CHECK\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn machine_check_handler(stack_frame: InterruptStackFrame) -> ! {
    panic!("EXCEPTION: MACHINE CHECK\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn simd_floating_point_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: SIMD FLOATING POINT\n{:#?}", stack_frame);
}

pub extern "x86-interrupt" fn virtualization_handler(stack_frame: InterruptStackFrame) {
    panic!("EXCEPTION: VIRTUALIZATION\n{:#?}", stack_frame);
}
