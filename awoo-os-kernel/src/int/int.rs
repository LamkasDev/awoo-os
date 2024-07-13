use crate::driver::{apic::apic::APIC_OFFSET, pic::pic::PIC_1_OFFSET};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
    Keyboard,
    Cascade,
    SerialCom2,
    SerialCom1,
    Parallel2,
    Floppy,
    Parallel1,
    Rtc,
    Open1,
    Open2,
    Open3,
    Mouse,
    Math,
    PrimaryATA,
    SecondaryATA,

    LapicFirst = APIC_OFFSET,
    LapicKeyboard,

    LapicSpurrious = 100,
    LapicError,
    LapicTimer,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
