use crate::driver::{ioapic::ioapic::IO_APIC_OFFSET, pic::pic::PIC_1_OFFSET};

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

    IoApicTimer = IO_APIC_OFFSET,
    IoApicKeyboard,
    IoApic2,
    IoApic3,
    IoApic4,
    IoApic5,
    IoApic6,
    IoApic7,
    IoApic8,
    IoApic9,
    IoApic10,
    IoApic11,
    IoApic12,
    IoApic13,
    IoApic14,
    IoApic15,

    LocalApicSpurrious = 100,
    LocalApicError,
    LocalApicTimer,
}

impl InterruptIndex {
    pub fn as_u8(self) -> u8 {
        self as u8
    }

    pub fn as_usize(self) -> usize {
        usize::from(self.as_u8())
    }
}
