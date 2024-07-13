use super::{frame, heap, page};
use bootloader_api::info::{MemoryRegions, Optional};
use spin::Mutex;
use x86_64::VirtAddr;

pub static PHYSICAL_MEMORY_OFFSET: Mutex<u64> = Mutex::new(0);

pub fn init_memory(
    physical_memory_offset: Optional<u64>,
    memory_regions: &'static mut MemoryRegions,
) {
    let physical_memory_offset_raw = physical_memory_offset.into_option().unwrap();
    *PHYSICAL_MEMORY_OFFSET.lock() = physical_memory_offset_raw;
    let physical_memory_offset = VirtAddr::new(physical_memory_offset_raw);
    let mut mapper = unsafe { page::init_page_table(physical_memory_offset) };
    let mut frame_allocator = unsafe { frame::BootInfoFrameAllocator::init(memory_regions) };
    heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}
