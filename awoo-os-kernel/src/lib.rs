#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader_api::entry_point;
use bootloader_api::{config::Mapping, BootInfo, BootloaderConfig};
use driver::{keyboard, shell::{queue::println, task::logging_task}, timer::{self}};

extern crate alloc;

pub mod driver;
pub mod gdt;
pub mod idt;
pub mod int;
pub mod memory;
pub mod panic;
pub mod pic;
pub mod serial;
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
    memory::memory::init_memory(
        boot_info.physical_memory_offset,
        &mut boot_info.memory_regions,
    );
    let mut executor = task::executor::Executor::new();
    executor.spawn(task::task::Task::new(logging_task(&mut boot_info.framebuffer)));
    executor.run_ready_tasks();
    gdt::gdt::init_gdt();
    idt::idt::init_idt();
    pic::pic::init_pics();
    timer::timer::setup_rtc();
    x86_64::instructions::interrupts::enable();
    println("enabled interrupts...");
    executor.spawn(task::task::Task::new(keyboard::task::scancode_task()));
    println("running loop...");
    executor.run();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
