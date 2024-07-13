use crate::{driver::logger::logger::println, port::port::port_wait_clear};
use spin::Mutex;
use x86_64::instructions::port::{Port, PortGeneric, ReadWriteAccess};

use super::queue::add_mouse_input;

pub struct MouseInput {
    pub flags: u8,
    pub x_delta: u8,
    pub y_delta: u8,
}

pub struct MousePosition {
    pub x: u16,
    pub y: u16,
}

pub static mut MOUSE_PORT_DATA: PortGeneric<u8, ReadWriteAccess> = Port::new(0x60);
pub static mut MOUSE_PORT_CMD: PortGeneric<u8, ReadWriteAccess> = Port::new(0x64);
pub static MOUSE_POSITION: Mutex<MousePosition> = Mutex::new(MousePosition { x: 0, y: 0 });

pub fn init_mouse() {
    unsafe {
        // Read status byte
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_CMD.write(0x20);
        let mut status = MOUSE_PORT_DATA.read();

        // Write modified status byte (Enable IRQ 12 and disable mouse clock)
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_CMD.write(0x60);
        status |= 0x2;
        status &= !0x20;
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_DATA.write(status);

        // Enable auxilary device
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_CMD.write(0xA8);

        // Enable mouse packets
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_CMD.write(0xD4);
        port_wait_clear(&mut MOUSE_PORT_CMD, 0x2);
        MOUSE_PORT_DATA.write(0xF4);
    }
    println("initialized mouse...");
}

pub fn common_mouse_code() {
    let flags: u8 = unsafe { MOUSE_PORT_DATA.read() };
    let x_delta: u8 = unsafe { MOUSE_PORT_DATA.read() };
    let y_delta: u8 = unsafe { MOUSE_PORT_DATA.read() };
    add_mouse_input(MouseInput {
        flags: flags,
        x_delta,
        y_delta,
    });
}
