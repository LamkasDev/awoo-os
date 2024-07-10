#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

#[cfg(test)]
use bootloader::entry_point;
use bootloader::BootInfo;

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
pub mod test;

#[cfg(test)]
entry_point!(test_kernel_main);

#[cfg(test)]
fn test_kernel_main(boot_info: &'static BootInfo) -> ! {
    init(boot_info);
    test_main();
    hlt_loop();
}

pub fn init(boot_info: &'static BootInfo) {
    gdt::gdt::init_gdt();
    idt::idt::init_idt();
    pic::pic::init_pics();
    x86_64::instructions::interrupts::enable();
    memory::memory::init_memory(boot_info);
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
