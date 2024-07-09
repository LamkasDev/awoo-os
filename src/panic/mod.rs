use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::println;

    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::serial_println;
    use crate::special;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    special::exit_qemu(special::QemuExitCode::Failed);
    loop {}
}
