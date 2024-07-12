use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use alloc::format;
    use crate::{driver::shell::queue::println, hlt_loop};

    println(&format!("{}", info));
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    use crate::hlt_loop;
    use crate::serial_println;
    use crate::special;

    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    special::qemu::exit_qemu(special::qemu::QemuExitCode::Failed);
    hlt_loop();
}
