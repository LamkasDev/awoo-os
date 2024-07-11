#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points
#![feature(custom_test_frameworks)]
#![test_runner(awoo_os_kernel::test::test::test_runner)]
#![reexport_test_harness_main = "test_main"]

use awoo_os_kernel::hlt_loop;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    hlt_loop();
}

#[test_case]
fn test_println() {
    use awoo_os_kernel::println;

    println!("test_println output");
}
