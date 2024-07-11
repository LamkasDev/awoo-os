#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(awoo_os_kernel::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use alloc::{boxed::Box, vec::Vec};
use awoo_os_kernel::{hlt_loop, memory::heap::HEAP_SIZE, BOOTLOADER_CONFIG};
use bootloader_api::{entry_point, BootInfo};

entry_point!(test_kernel_main, config = &BOOTLOADER_CONFIG);

fn test_kernel_main(boot_info: &'static mut BootInfo) -> ! {
    awoo_os_kernel::init(boot_info);
    test_main();
    hlt_loop();
}

#[test_case]
fn simple_allocation() {
    let heap_value_1 = Box::new(41);
    let heap_value_2 = Box::new(13);
    assert_eq!(*heap_value_1, 41);
    assert_eq!(*heap_value_2, 13);
}

#[test_case]
fn large_vec() {
    let n = 1000;
    let mut vec = Vec::new();
    for i in 0..n {
        vec.push(i);
    }
    assert_eq!(vec.iter().sum::<u64>(), (n - 1) * n / 2);
}

#[test_case]
fn many_boxes() {
    for i in 0..HEAP_SIZE {
        let x = Box::new(i);
        assert_eq!(*x, i);
    }
}
