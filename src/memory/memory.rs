use super::{frame, heap, page};
use bootloader::BootInfo;
use x86_64::VirtAddr;

pub fn init_memory(boot_info: &'static BootInfo) {
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { page::init_page_table(phys_mem_offset) };
    let mut frame_allocator = unsafe { frame::BootInfoFrameAllocator::init(&boot_info.memory_map) };
    heap::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");
}
