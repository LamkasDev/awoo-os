#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(awoo_os::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use awoo_os::println;

// This function is an entry point.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    awoo_os::init();

    #[cfg(test)]
    test_main();

    loop {}
}
