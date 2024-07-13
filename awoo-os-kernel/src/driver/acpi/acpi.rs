use crate::{
    driver::{apic::apic::init_apic, ioapic::ioapic::init_ioapic, logger::logger::println},
    memory::memory::PHYSICAL_MEMORY_OFFSET,
};
use acpi::{AcpiHandler, AcpiTables, PhysicalMapping};
use alloc::format;
use bootloader_api::info::Optional;
use core::ptr::NonNull;

static ACPI_HANDLER: AwooAcpiHandler = AwooAcpiHandler {};

#[derive(Debug, Clone, Copy)]
pub struct AwooAcpiHandler {}

impl AcpiHandler for AwooAcpiHandler {
    unsafe fn map_physical_region<T>(
        &self,
        physical_address: usize,
        size: usize,
    ) -> acpi::PhysicalMapping<Self, T> {
        let offset = *PHYSICAL_MEMORY_OFFSET.lock();
        let address = offset + physical_address as u64;
        let mapping = PhysicalMapping::new(
            physical_address,
            NonNull::new(address as *mut T).unwrap(),
            size,
            size,
            ACPI_HANDLER,
        );

        return mapping;
    }

    fn unmap_physical_region<T>(_region: &acpi::PhysicalMapping<Self, T>) {}
}

pub unsafe fn init_acpi(rsdp_address: Optional<u64>) -> bool {
    match rsdp_address {
        bootloader_api::info::Optional::Some(rsdp_address) => {
            let acpi_tables = AcpiTables::from_rsdp(ACPI_HANDLER, rsdp_address as usize).unwrap();
            let platform_info = acpi_tables.platform_info().unwrap();
            match platform_info.interrupt_model {
                acpi::InterruptModel::Unknown => {
                    println("unknown interrupt model found in acpi platform info!");
                    return false;
                }
                acpi::InterruptModel::Apic(apic) => {
                    println(&format!(
                        "discovered local apic at {:#012x}...",
                        apic.local_apic_address
                    ));
                    for ioapic in apic.io_apics.iter() {
                        println(&format!(
                            "discovered ioapic {} with base {} at {:#012x}",
                            ioapic.id, ioapic.global_system_interrupt_base, ioapic.address
                        ));
                        init_ioapic(&apic, ioapic);
                    }
                    /* for a in apic.nmi_sources.iter() {
                        println(&format!("found nmi with gsi {}", a.global_system_interrupt));
                    }
                    for a in apic.local_apic_nmi_lines.iter() {
                        println(&format!(
                            "found lapic nmi line {:?} at {:?}",
                            a.line, a.processor
                        ));
                    } */
                    init_apic(*PHYSICAL_MEMORY_OFFSET.lock() + apic.local_apic_address);

                    return true;
                }
                _ => {
                    println("no interrupt model found in acpi platform info!");
                    return false;
                }
            }
        }
        bootloader_api::info::Optional::None => {
            println("couldn't find rsdp table!");
            return false;
        }
    }
}
