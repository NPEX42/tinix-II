pub mod arch;
pub mod drivers;
pub mod hardware;
pub mod fs;

use bootloader::BootInfo;
use x86_64::{VirtAddr};
use arch::x64::mem::{init,allocator};

use crate::{
    heap,
    input::{
        self,
        serial_println
    },
    kernel::{
        self, arch::disable_irq},
        log, 
        time
    };

pub fn boot(_boot_info : &'static BootInfo) {


    crate::clear_console!();

    kernel::drivers::file_systems::ustar::check_meta_block_size();
    //graphics::clear_screen(graphics::Color::Black);
    log!("Booting Tinix-core v{}\n", crate::version());
    init_component!(arch::x64::init, ());
    init_component!(hardware::pit::init, ());
    init_component!(hardware::pic::init, ());
    disable_irq(1);
    set_interrupt!(0, crate::time::update);
    //set_interrupt!(4, crate::kernel::hardware::uart::on_serial_interrupt);
    // for i in -1..10 {
    //     let bad = 1 / i;
    //     log!("{}", bad);
    // }
    log!("[Boot/mem::allocator::init_heap] - Initialising {} MB [{} Frames]\n", (heap::HEAP_SIZE / 1024) / 1024, heap::HEAP_SIZE / 4096);
    let phys_mem_offset = VirtAddr::new(_boot_info.physical_memory_offset);
        let mut mapper = unsafe { init(phys_mem_offset)
            .expect("Couldn't Initialize Mapper...") };
        let mut frame_allocator = unsafe {
            allocator::BootFrameAllocator::init(&_boot_info.memory_map)
        };

        let mut memory_size = 0;
        for region in _boot_info.memory_map.iter() {
            let start_addr = region.range.start_addr();
            let end_addr = region.range.end_addr();
            memory_size += end_addr - start_addr;
            serial_println!("MEM [{:#016X}-{:#016X}] {:?}", start_addr, end_addr, region.region_type);
        }

        log!("Detected {:3} MB of Memory\n", memory_size >> 20);

        unsafe {crate::sys::mem::TOTAL_MEMORY = memory_size}

     crate::kernel::allocator::init_heap(&mut mapper, &mut frame_allocator).expect("Couldn't Initialize Allocator...");
    
     init_component!(crate::kernel::hardware::ata::init, ());

     log!("Disk 0 Is Present: {}\n",kernel::hardware::ata::drive_is_present(0));

     init_component!(kernel::drivers::file_systems::ustar::init, ());
    
    init_component!(input::init, ());

    crate::println!("Press Any Key To Start...");
    while input::key().is_none() {
        time::sleep_ticks(10);
    }

    crate::reset_console!();
}

#[derive(Debug, Copy, Clone)]
pub struct InitError(&'static str);

pub type InitResult<T> = Result<T, InitError>;



pub macro init_component($path : path, $return_type : ty) {
    {
        let f : fn() -> InitResult<$return_type> = $path;
        $crate::log!("[Boot / {}] Initing - ", stringify!($path));
        let result = f().expect("Failed");
        $crate::log!("[OK]\n\r");
        result
    }
} 

pub macro set_interrupt($irq : expr, $path : path) {
    {
        let f : fn(u8) = $path;
        $crate::log!("[Boot] IRQ{} -> {} - ", $irq, stringify!($path));
        let result = $crate::kernel::arch::set_interrupt($irq, f).expect("Failed");
        $crate::kernel::arch::enable_irq($irq);
        $crate::log!("[OK]\n");
        result
    }
}