use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use x86_64::{PhysAddr, instructions::interrupts::without_interrupts, structures::paging::{Size4KiB, FrameAllocator, PhysFrame}};
use crate::{log, time};
use linked_list_allocator::LockedHeap;

#[global_allocator]
pub static ALLOCATOR: LockedHeap = LockedHeap::empty();


pub struct NullFrameAllocator;

unsafe impl FrameAllocator<Size4KiB> for NullFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        None
    }
}

pub struct BootFrameAllocator {
    memory_map: &'static MemoryMap,
    next: usize,
}

impl BootFrameAllocator {
    pub unsafe fn init(memory_map : &'static MemoryMap) -> Self {
        Self {
            memory_map,
            next : 0,
        }
    } 


    pub fn usable_frames(&self) -> impl Iterator<Item = PhysFrame> {
        let regions = self.memory_map.iter();
        let usable_regions = 
            regions.filter(|r| r.region_type == MemoryRegionType::Usable);
        let addr_ranges = usable_regions
            .map(|r| r.range.start_addr()..r.range.end_addr());

        let frame_addresses = addr_ranges
            .flat_map(|r| r.step_by(4096));
        
        frame_addresses.map(|addr| PhysFrame::containing_address(PhysAddr::new(addr)))
    }

    pub fn frame_count(&self) -> usize {
        let frames = self.usable_frames();
        frames.count()
    }
}

unsafe impl FrameAllocator<Size4KiB> for BootFrameAllocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame> {
        let frame = self.usable_frames().nth(self.next);
        self.next += 1;
        frame
    }
}




use alloc::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

pub struct Dummy;

unsafe impl GlobalAlloc for Dummy {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 {
        null_mut()
    }

    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {
        panic!("dealloc should be never called")
    }
}


use x86_64::{
    structures::paging::{
        mapper::MapToError, Mapper, Page, PageTableFlags,
    },
    VirtAddr,
};

use super::heap::*;
// use lazy_static::lazy_static;

// lazy_static! {
//     static ref MAPPER : Mutex<Option<&'static dyn Mapper<Size4KiB>>> = Mutex::new(None);
//     static ref FRAME_ALLOCATOR : Mutex<Option<&'static dyn FrameAllocator<Size4KiB>>> = Mutex::new(None);
// }

pub fn init_heap(
    mapper: &mut impl Mapper<Size4KiB>,
    frame_allocator: &mut impl FrameAllocator<Size4KiB>,
) -> Result<(), MapToError<Size4KiB>> {

    // MAPPER.lock() = Some(mapper);
    // unsafe {MAPPER.force_unlock();}

    // FRAME_ALLOCATOR.lock() = Some(frame_allocator);
    // unsafe {FRAME_ALLOCATOR.force_unlock()};
    
    let page_range = {
        let heap_start = VirtAddr::new(HEAP_START as u64);
        let heap_end = heap_start + HEAP_SIZE - 1u64;
        let heap_start_page = Page::containing_address(heap_start);
        let heap_end_page = Page::containing_address(heap_end);
        Page::range_inclusive(heap_start_page, heap_end_page)
    };
    let mut bytes_inited = 0;
    let mut tp1;
    let mut tp2;
    let mut times = Averager::new();
    for page in page_range {
        tp1 = time::ticks();
        // let frame = frame_allocator
        //     .allocate_frame()
        //     .ok_or(MapToError::FrameAllocationFailed)?;
        // let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
        // unsafe {
        //     mapper.map_to(page, frame, flags, frame_allocator)?.flush();
        // };

        allocate_frame(mapper, frame_allocator, page);

        tp2 = time::ticks();
        

        bytes_inited += 4096;
        let prog_len = 10;
        log!("Initializing {} Bytes - [", HEAP_SIZE);
        let mut remaining = prog_len;
        let fill = bytes_inited as f32 / HEAP_SIZE as f32;
        for _ in 0..=((prog_len as f32 * fill) as usize) {
            log!("=");
            remaining -= 1;
        }

        for _ in 0..=remaining {
            log!(" ");
        }

        if page != page_range.last().unwrap() {
        let pages_remaining : f64 = ((HEAP_SIZE as f64 - bytes_inited as f64) / 4096 as f64) as f64;
        let seconds_per_page = (tp2 - tp1) as f64 / 1000.0;
        let time_remaining_seconds = pages_remaining * seconds_per_page;
        times.add(time_remaining_seconds);
        log!("] - {:03.1}% - {:03.0}s - [{} left]\r", fill * 100.0,times.avg(), pages_remaining);
        
        }

        
    }
    log!("] - [OK]                              ");

    unsafe {
        without_interrupts(|| {
            ALLOCATOR.lock().init(HEAP_START,HEAP_SIZE );
        });
    }
       Ok(())
}

pub fn extend_mapping(_size : usize) {
    // let mut mapper = MAPPER.lock().unwrap();
    // let mut frame_allocator = FRAME_ALLOCATOR.lock().unwrap();
    // let pages = {
    //     let start : Page<Size4KiB> = Page::containing_address(VirtAddr::new(mem::total() as u64));
    //     let end : Page<Size4KiB> = Page::containing_address(VirtAddr::new(mem::total() as u64 + size as u64));
    // };

    // for page in pages {
    //     allocate_frame(mapper, frame_allocator, page);
    // }
}

pub fn allocate_frame(mapper: &mut impl Mapper<Size4KiB>, frame_allocator: &mut impl FrameAllocator<Size4KiB>, page : Page) {
    let frame = frame_allocator.allocate_frame().expect("Frame Allocation Failed...");
    let flags = PageTableFlags::PRESENT | PageTableFlags::WRITABLE;
    unsafe {
        mapper.map_to(page, frame, flags, frame_allocator).expect("Mapping Failed...").flush();
    };
}


const SAMPLES : usize = 16;

struct Averager {
    buffer : [f64; SAMPLES],
    pointer : usize,
}

impl Averager {
    pub fn new() -> Self {
        Averager {
            buffer : [0.0; SAMPLES],
            pointer : 0,
        }
    }

    pub fn add(&mut self, x : f64) {
        self.buffer[self.pointer] = x;
        self.inc_pointer();
    }

    pub fn avg(&self) -> f64 {
        let mut sum : f64 = 0.0;
        for x in self.buffer {
            sum += x as f64;
        }
        sum / self.buffer.len() as f64
    }

    fn inc_pointer(&mut self) {
        self.pointer += 1;
        self.pointer %= self.buffer.len(); 
    }
}