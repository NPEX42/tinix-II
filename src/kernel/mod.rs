pub mod arch;
pub mod drivers;
pub mod hardware;

use bootloader::BootInfo;
use x86_64::{VirtAddr, structures::paging::{FrameAllocator, Mapper, Size4KiB}};
use arch::x64::mem::{init,allocator, heap::*, mapper::*};

use crate::{draw_string_f, graphics, heap, input, println, time, user};

pub fn boot(_boot_info : &'static BootInfo) {

    /* if cfg!(feature = "gfx480x640") */ { drivers::vga_dr::init().expect("Couldn't Initialize The VGA Driver...");}
    
    
    graphics::clear_screen(graphics::Color::Black);
    println!("Booting Tinix-core v{}", crate::version());
    init_component!(arch::x64::init, ());
    init_component!(hardware::pit::init, ());
    init_component!(hardware::pic::init, ());
    set_interrupt!(0, crate::time::update);
    crate::println!("[Boot/mem::allocator::init_heap] - Initialising {} Bytes - ", heap::HEAP_SIZE);
    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
        let mut mapper = unsafe { init(phys_mem_offset)
            .expect("Couldn't Initialize Mapper...") };
        let mut frame_allocator = unsafe {
            allocator::BootFrameAllocator::init(&_boot_info.memory_map)
        };

     crate::kernel::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Couldn't Initialize Allocator...");
     crate::println!("OK");
    
    init_component!(input::init, ());

    crate::println!("Press Any Key To Start...");
    while(input::key().is_none()) {
        time::sleep_ticks(10);
    }

    crate::reset_pos!();
}

#[derive(Debug, Copy, Clone)]
pub struct InitError(&'static str);

pub type InitResult<T> = Result<T, InitError>;



pub macro init_component($path : path, $return_type : ty) {
    {
        let f : fn() -> InitResult<$return_type> = $path;
        $crate::print!("[Boot/{}] - Initing - ", stringify!($path));
        $crate::input::serial_print!("[Boot/{}] - Initializing - ", stringify!($path));
        let result = f().expect("Failed");
        $crate::println!("OK");
        $crate::input::serial_println!("OK");
        result
    }
} 

pub macro set_interrupt($irq : expr, $path : path) {
    {
        let f : fn(u8) = $path;
        $crate::input::serial_print!("[Boot/{}] - Hooking Up Interrupt #{} - ", stringify!($path), $irq);
        let result = $crate::kernel::arch::set_interrupt($irq, f).expect("Failed");
        $crate::input::serial_println!("OK");
        result
    }
}