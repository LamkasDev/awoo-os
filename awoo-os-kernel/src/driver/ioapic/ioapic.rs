use crate::{
    driver::{
        apic::apic::send_apic_eoi, keyboard::keyboard::common_keyboard_code,
        logger::logger::println, mouse::mouse::common_mouse_code,
    },
    memory::memory::PHYSICAL_MEMORY_OFFSET,
};
use acpi::platform::interrupt::{Apic, IoApic as AcpiIoApic};
use alloc::{alloc::Global, format};
use x2apic::ioapic::{IoApic, RedirectionTableEntry};
use x86_64::structures::idt::InterruptStackFrame;

pub const IO_APIC_OFFSET: u8 = 64;

pub unsafe fn init_ioapic(apic: &Apic<Global>, ioapic: &AcpiIoApic) {
    let address = *PHYSICAL_MEMORY_OFFSET.lock() + ioapic.address as u64;
    let mut ioapic = IoApic::new(address);
    ioapic.init(IO_APIC_OFFSET);
    ioapic.enable_irq(0);
    ioapic.enable_irq(1);
    ioapic.enable_irq(2);
    ioapic.enable_irq(3);
    ioapic.enable_irq(4);
    ioapic.enable_irq(5);
    ioapic.enable_irq(6);
    ioapic.enable_irq(7);
    ioapic.enable_irq(8);
    ioapic.enable_irq(9);
    ioapic.enable_irq(10);
    ioapic.enable_irq(11);
    ioapic.enable_irq(12);
    ioapic.enable_irq(13);
    ioapic.enable_irq(14);
    ioapic.enable_irq(15);
    ioapic.enable_irq(16);

    for a in apic.interrupt_source_overrides.iter() {
        println(&format!(
            "mapping legacy interrupt {} to {}...",
            a.global_system_interrupt,
            IO_APIC_OFFSET + a.isa_source
        ));
        let mut entry = RedirectionTableEntry::default();
        entry.set_vector(IO_APIC_OFFSET + a.isa_source);
        ioapic.set_table_entry(a.global_system_interrupt as u8, entry);
    }

    println("initialized IOAPIC...");
}

pub extern "x86-interrupt" fn io_apic_timer_handler(_stack_frame: InterruptStackFrame) {
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn io_apic_keyboard_handler(_stack_frame: InterruptStackFrame) {
    common_keyboard_code();
    unsafe { send_apic_eoi() };
}

pub extern "x86-interrupt" fn io_apic_mouse_handler(_stack_frame: InterruptStackFrame) {
    common_mouse_code();
    unsafe { send_apic_eoi() };
}
