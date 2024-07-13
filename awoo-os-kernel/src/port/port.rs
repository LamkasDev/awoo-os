use x86_64::instructions::port::{PortGeneric, ReadWriteAccess};

pub unsafe fn port_wait_clear(port: &mut PortGeneric<u8, ReadWriteAccess>, clear: u8) {
    loop {
        let data = port.read();
        if clear & data != clear {
            break;
        }
    }
}

pub unsafe fn port_wait_set(port: &mut PortGeneric<u8, ReadWriteAccess>, set: u8) {
    loop {
        let data = port.read();
        if set & data == set {
            break;
        }
    }
}
