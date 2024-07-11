use super::{frame, heap, page};
use bootloader_api::info::{MemoryRegions, Optional};
use x86_64::VirtAddr;

pub fn init_memory(
    physical_memory_offset: Optional<u64>,
    memory_regions: &'static mut MemoryRegions,
) {
    let phys_mem_offset = VirtAddr::new(physical_memory_offset.into_option().unwrap());
    let mut mapper = unsafe { page::init_page_table(phys_mem_offset) };
    let mut frame_allocator = unsafe { frame::BootInfoFrameAllocator::init(memory_regions) };
    heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}
