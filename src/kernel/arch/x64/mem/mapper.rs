use x86_64::{
    PhysAddr, VirtAddr,
    structures::paging::{
        Page, PhysFrame, Mapper, Size4KiB, FrameAllocator
    }
};
// [NOTE] For some reason, rustc reports this as missing, but it still works.
use x86_64::structures::paging::OffsetPageTable;