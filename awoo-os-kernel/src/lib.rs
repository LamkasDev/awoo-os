#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![feature(allocator_api)]
#![test_runner(crate::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader_api::entry_point;
use bootloader_api::{config::Mapping, BootInfo, BootloaderConfig};
use constants::constants::FORCE_PIC;
use driver::{
    acpi::acpi::init_acpi,
    keyboard::task::keyboard_task,
    logger::logger::{init_logger, println},
    mouse::{mouse::init_mouse, task::mouse_task},
    pic::pic::init_pics,
    rtc::rtc::init_rtc,
};
use gdt::gdt::init_gdt;
use idt::idt::init_idt;
use memory::memory::init_memory;
use screen::task::screen_task;

extern crate alloc;

pub mod constants;
pub mod driver;
pub mod gdt;
pub mod idt;
pub mod int;
pub mod memory;
pub mod panic;
pub mod port;
pub mod screen;
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
    init_logger();
    let mut executor = task::executor::Executor::new();
    executor.spawn(task::task::Task::new(keyboard_task()));
    executor.spawn(task::task::Task::new(mouse_task()));
    executor.spawn(task::task::Task::new(screen_task(
        &mut boot_info.framebuffer,
    )));
    executor.run_ready_tasks();
    init_gdt();
    init_idt();
    if !FORCE_PIC && unsafe { init_acpi(boot_info.rsdp_addr) } {
        println("current interrupt source: APIC");
        println("current time source: APIC timer");
    } else {
        init_pics();
        init_rtc();
        println("current interrupt source: PIC");
        println("current time source: RTC");
    }
    x86_64::instructions::interrupts::enable();
    println("enabled interrupts...");
    init_mouse();
    println("running loop...");
    executor.run();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
