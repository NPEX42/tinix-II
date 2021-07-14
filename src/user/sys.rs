pub mod mem {

    use linked_list_allocator::LockedHeap;
    use x86_64::instructions::interrupts::without_interrupts;
    use crate::{kernel::arch::x64::mem::allocator::ALLOCATOR};

    pub static mut TOTAL_MEMORY : u64 = 0;

    pub fn total() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().size()
        });
        total
    }

    pub fn free() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().free()
        });
        total
    }

    pub fn used() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().used()
        });
        total
    }

    pub fn total_ram() -> u64 {
        unsafe {TOTAL_MEMORY}
    }

    pub fn heap_usage() -> f32 {
        used() as f32 / total() as f32
    }

    pub unsafe fn allocator() -> &'static LockedHeap {
        &ALLOCATOR
    }

    pub fn grow() {
        //heap::grow_heap();
    }


}


pub mod programs {

    use crate::graphics::Color;

    use crate::heap::MB;
    use crate::input::{self};
    use crate::kernel::drivers::file_systems::{Disk, get_disk, get_sector, set_sector, zero_format_disk};
    use crate::kernel::hardware::ata;
    use crate::{background, clear_console, foreground, log};

    pub type ProgramStatusCode = u16;
    pub fn install() -> ProgramStatusCode {
        foreground!(Color::Red);
        background!(Color::Black);
        clear_console!();
        log!("=== WARNING ====\n");
        log!("[install] WILL ERASE ALL DATA ON THE SELECTED DISK. Backups Are Not Optional!\n");
        log!("ARE YOU SURE THAT YOU WANT TO CONTINUE?\n");
        loop {
            match input::string("(Y / N): ").as_str() {
                "N" | "NO" => return 1,
                "Y" | "YES" => { 
                    display_drives();
                    let selection = input::number("Select A Drive: ", 0..ata::DISKS.lock().len());
                    install_to(selection);
                    return 0;
                },
                _ => {}
            }
        }
    }

    fn display_drives() {
        log!("\n");
        for (index, disk) in ata::DISKS.lock().iter().enumerate() {
            log!("{}) {}\n", index, disk.name);
        }
    }

    fn install_to(disk : usize) {
        let dest = &ata::DISKS.lock()[disk];
        zero_format_disk(dest);
        copy_boot_sectors(dest);
    }

    fn copy_boot_sectors(dest_disk : &Disk) {
        let bootsector_len = ((MB * 8) / 512) as u32;
        let new_bootdisk_opt = get_disk(0,1 );

        if new_bootdisk_opt.is_none() {
            log!("Couldn't Find Boot Disk!");
            return;
        }

        let bootdisk  = dest_disk;

        let new_bootdisk = new_bootdisk_opt.unwrap();
        log!("\nCopying Sectors");
        for block in 0..bootsector_len {
            set_sector(&new_bootdisk, block, &get_sector(&bootdisk, block));
            if block % (bootsector_len / 10) == 0 && block > 0 {log!(".");}
        }

        log!(" [OK]");

        // log!("\nVerifying Sectors");
        // for block in 0..bootsector_len {
        //     let original = get_sector(&bootdisk, block);
        //     let copy = get_sector(&new_bootdisk, block);

        //     if !(original == copy) {
        //         set_sector(&new_bootdisk, block, &original)
        //     }

        //     if block % (bootsector_len / 10) == 00 && block > 0 {log!(".");}
        // }

        log!(" [OK]");
        
    }
}