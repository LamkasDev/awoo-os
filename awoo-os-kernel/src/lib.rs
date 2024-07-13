#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader_api::entry_point;
use bootloader_api::{config::Mapping, BootInfo, BootloaderConfig};
use constants::constants::FORCE_PIC;
use driver::{
    acpi::acpi::init_acpi,
    keyboard::task::scancode_task,
    logger::{queue::println, task::logging_task},
    pic::pic::init_pics,
    rtc::rtc::init_rtc,
};
use gdt::gdt::init_gdt;
use idt::idt::init_idt;
use memory::memory::init_memory;

extern crate alloc;

pub mod constants;
pub mod driver;
pub mod gdt;
pub mod idt;
pub mod int;
pub mod memory;
pub mod panic;
pub mod special;
pub mod task;
pub mod test;

pub static BOOTLOADER_CONFIG: BootloaderConfig = {
    let mut config = BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config
};

#[cfg(test)]
entry_point!(test_kernel_main, config = &BOOTLOADER_CONFIG);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

pub fn init(boot_info: &'static mut BootInfo) {
    init_memory(
        boot_info.physical_memory_offset,
        &mut boot_info.memory_regions,
    );
    let mut executor = task::executor::Executor::new();
    executor.spawn(task::task::Task::new(logging_task(
        &mut boot_info.framebuffer,
    )));
    executor.run_ready_tasks();
    init_gdt();
    init_idt();
    if !FORCE_PIC && unsafe { init_acpi(boot_info.rsdp_addr) } {
        println("interrupt source: APIC");
        println("time source: APIC timer");
    } else {
        init_pics();
        init_rtc();
        println("interrupt source: PIC");
        println("time source: RTC");
    }
    x86_64::instructions::interrupts::enable();
    println("enabled interrupts...");
    executor.spawn(task::task::Task::new(scancode_task()));
    println("running loop...");
    executor.run();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
