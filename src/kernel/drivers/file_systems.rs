use core::{fmt::{Display, UpperHex}, ops::{Index, IndexMut, Range}};

use alloc::{format, string::String, vec, vec::Vec};
use spin::Mutex;
use x86_64::instructions::interrupts::without_interrupts;

use crate::{ConstPointer, MutPointer, input::{serial_print, serial_println}, io::{IoReader, IoWriter}, kernel::hardware::ata::{self, BlockIndex}, log, size_of};

use self::{file_table::{FileTable, FileTableEntry, file::FileInfo}, ustar::search};

pub mod ustar;
pub mod file_table;

pub struct Disk {
    pub(crate) bus : u8,
    pub(crate) drive : u8,
    pub(crate) size : usize,
    pub(crate) name : String, 
}

pub fn get_disk(bus : u8, disk_num : u8) -> Option<Disk> {
        if let Some((bus_id, disk_id,model, _, _, _, sectors)) = ata::indentify_drive(bus, disk_num) {
        Some(Disk {
            bus : bus_id, 
            drive : disk_id,
            size : sectors as usize * 512,
            name : model,
        })
        } else {
            None
        }
}



pub fn get_sector(disk : &Disk, sector_num : BlockIndex) -> Sector {
    let mut sector = Sector::new(sector_num);
    //serial_println!("Reading Sector 0x{:06x} from block 0x{:06x}",sector.index, sector_num);
    ata::read(disk.bus,disk.drive, sector_num as u32, &mut sector);
    sector
}

pub fn get_sectors(disk : &Disk, sector_range : Range<BlockIndex>) -> SectorList {
    let mut list = SectorList::with_capacity(0);
    for block in sector_range {
        list.add_sector(&get_sector(disk, block));
    }
    list
}

pub fn set_sector(disk : &Disk, sector_num : BlockIndex, sector : &Sector) {
    //serial_println!("Writing Sector 0x{:06x} to block 0x{:06x}",sector.index, sector_num);
    ata::write(disk.bus, disk.drive, sector_num as u32, &sector);
}

pub fn set_sectors(disk : &Disk, start_block : BlockIndex, sectors : &SectorList) {
    let mut index = 0;
    for sector in sectors.sectors() {
        //serial_println!("Writing Sector [{}][{}]", start_block, index);
        set_sector(disk, index + start_block, sector);
        index += 1;
    }
}

pub fn copy_sectors(src_disk : &Disk, dest_disk : &Disk, start_block : BlockIndex, amount : BlockIndex, dest_block : BlockIndex) {
    let sectors = get_sectors(src_disk, start_block..(start_block + amount + 1));
    set_sectors(dest_disk, dest_block, &sectors);
}

pub fn clear_sector(disk : &Disk, sector : BlockIndex) {
    set_sector(disk, sector, &Sector::new(sector));
}

pub fn is_sector_empty(disk : &Disk, sector_num : BlockIndex) -> bool {
    let value = get_sector(disk, sector_num);
    value == Sector::new(sector_num)
}

pub fn is_range_empty(disk : &Disk, sectors : Range<BlockIndex>) -> bool {
    for i in sectors {
        if !is_sector_empty(disk, i) {return false};
    }
    return true;
}

pub fn get_disk_sector_count(bus : u8, drive : u8) -> usize {
    let disk = get_disk(bus, drive);

    if disk.is_none()  {return 0;};
    if disk.is_some() {return disk.unwrap().size / 512;};
    return 0; //Should Be Unreachable
} 

pub fn zero_format_disk(disk : &Disk) {
    log!("Formatting Disk {}\n", disk.name);
    let sector_count = disk.size / 512;
    let one_percent = sector_count / 100;
    let mut progress : f32 = 0.0;
    let dot_threshold = 0.01;

    let mut counter = 0;
    for sector in 0..sector_count {
        set_sector(disk, sector as u32, &Sector::new(sector as u32));
        if (sector % (one_percent as f32 * dot_threshold) as usize ) == 0 && sector > 0 {
            log!("{}", if counter % 2 == 0 {"."} else {"-"});
        }

        if sector % (one_percent as f32 * dot_threshold *  10.0) as usize == 0 && sector > 0 {
            progress += dot_threshold * 10.0;
            log!(" {:03.1}% ({}/{}) \r", progress,sector, sector_count );
            counter += 1;
        }
    }
    log!(" [OK]");
}



pub trait File<T> : IoReader<T> + IoWriter<T> {
    fn open(name : &str) -> Self;
    fn close(&mut self);

    fn read_all(&mut self) -> Vec<T>;
    fn write_all(&mut self, data : Vec<T>);

    fn get_sectors(&self) -> SectorList;
}



pub const SECTOR_SIZE : usize = 512;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SectorList {
    sectors : Vec<Sector>
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Sector {
    index : BlockIndex,
    data : [u8; 512]
}

impl Sector {
    pub fn new(index : BlockIndex) -> Self {
        Self {
            index,

            data : [0; 512]
        }
    }
    pub fn data(&self) -> [u8; 512] {
        self.data
    }

    pub fn data_mut(&mut self) -> &mut [u8; 512] {
        &mut self.data
    }

    pub fn write(&mut self, offset : usize, byte : u8) {
        self.data[offset] = byte;
    }

    pub fn read(&self, offset : usize) -> u8 {
        self.data[offset]
    }
}


impl SectorList {
    pub fn new() -> Self {
        Self {
            sectors : Vec::new()
        }
    }

    pub fn add_sector(&mut self, sector : &Sector) {
        self.sectors.push(*sector);
    }

    pub fn with_capacity(capacity : usize) -> Self {
        let mut sectors = Vec::with_capacity(capacity);
        for i in 0..capacity {
            sectors.push(Sector::new(i as BlockIndex))
        }
        Self {
            sectors
        }
    } 

    pub fn write(&mut self, address : usize, value : u8) {
        let sector = address / SECTOR_SIZE;
        if sector >= self.sectors.len() {self.allocate_new_sector()}
        let offset = address % SECTOR_SIZE;

        self.sectors[sector].write(offset, value);
    }

    pub fn read(&self, address : usize) -> Option<u8> {
        let sector = address / SECTOR_SIZE;
        if sector >= self.sectors.len() {return None;};
        let offset = address % SECTOR_SIZE;

        Some(self.sectors[sector].read(offset))
    }

    pub fn read_u8(&self, address : usize) -> Option<u8> {
        self.read(address)
    }

    pub fn read_u16(&self, address : usize) -> Option<u16> {
        let a = self.read_u8(address + 0).unwrap() as u16;
        let b = self.read_u8(address + 1).unwrap() as u16;

        Some(a << 8 | b)
    }

    pub fn read_u32(&self, address : usize) -> Option<u32> {
        let a = self.read_u16(address + 0).unwrap() as u32;
        let b = self.read_u16(address + 1).unwrap() as u32;

        Some(a << 16 | b)
    }

    pub fn read_u64(&self, address : usize) -> Option<u64> {
        let a = self.read_u32(address + 0).unwrap() as u64;
        let b = self.read_u32(address + 1).unwrap() as u64;

        Some(a << 32 | b)
    }

    pub fn read_asciiz(&mut self, address : usize) -> String {
        let data : Vec<u8> = self.read_until_null(address);

        let mut s = String::with_capacity(data.len());
        for byte in data {
            s.push(byte as char);
        }
        s
    }

    pub fn write_u8(&mut self, address : usize, value : u8) {
        self.write(address, value);
    }

    pub fn write_u16(&mut self, address : usize, value : u16) {
        self.write_slice(&value.to_be_bytes(), address);
    }

    pub fn write_u32(&mut self, address : usize, value : u32) {
        self.write_slice(&value.to_be_bytes(), address);
    }

    pub fn write_u64(&mut self, address : usize, value : u16) {
        self.write_slice(&value.to_be_bytes(), address);
    }

    pub fn write_asciiz(&mut self, address : usize, text : String) -> usize {
        let mut bytes_written = 0;
        for (_, chr) in text.chars().enumerate() {
            if chr <= 0x7E as char {
                self.write(address + bytes_written, chr as u8);
                bytes_written += 1;
            }
        }
        bytes_written
        
    }

    pub fn get_sector(&self, sector : usize) -> Option<&Sector> {
        if sector >= self.sectors.len() {return None;};
        Some(&self.sectors[sector])
    }

    pub fn set_sector(&mut self, sector_idx : usize, sector : Sector) -> Result<(), &'static str> {
        if sector_idx >= self.sectors.len() {return Err("Sector Index Out Of Range");};
        self.sectors[sector_idx] = sector;
        Ok(())
    }

    pub fn read_until_null(&self, address : usize) -> Vec<u8> {
        let mut v : Vec<u8> = Vec::new();
        let mut index = 0;
        while let Some(byte) = self.read_u8(address + index) {
            if byte == b'\0' {return v};
            v.push(byte);
            index += 1;
        }
        v
    }

    pub fn sectors(&self) -> &Vec<Sector> {
        &self.sectors
    }

    pub fn sectors_mut(&mut self) -> &mut Vec<Sector> {
        &mut self.sectors
    }

    fn allocate_new_sector(&mut self) {
        let next_index = self.sectors.len();
        self.sectors.push(Sector::new(next_index as BlockIndex));
        //log!("Created New Sector\n");
    }

    pub fn prepend<'a>(&mut self, other : &'a mut SectorList) -> &'a SectorList {
        other.sectors_mut().append(self.sectors_mut());
        serial_println!("Prepend Sectors...");
        other
    }

    pub fn append<'a>(&'a mut self, other : &'a mut SectorList) -> &'a SectorList {
        self.sectors_mut().append(other.sectors_mut());
        serial_println!("Append Sectors...");
        self
    }

    pub fn get_sector_begin(&self) -> u32 {
        self.get_sector(0).unwrap().index as u32
    }

    pub fn write_slice(&mut self, slice : &[u8], start_address : usize) {
        let mut idx = 0;
        for byte in slice {
            self.write(start_address + idx, *byte);
            idx += 1;
        }
    }

    
}

impl Display for SectorList {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "=== Sectors {}..{} ===\n", self.get_sector_begin(), self.get_sector_begin() + self.sectors().len() as u32);

        for sector in self.sectors() {
            write!(f, "{:X}\n", sector);
        }
        Ok(())
    }
}

impl UpperHex for Sector {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {

        write!(f, "Sector 0x{:06X}:\n", self.index);
        let bytes_per_line = 16;
        for line in (0..512).step_by(bytes_per_line) {
            write!(f, " 0x{:03X}: ", line);
            let mut char_data = String::new();
            for idx in 0..bytes_per_line {
                let value = self.data[(line + idx)];
                write!(f, " {:02X}", value);
                char_data += format!("{}", if (0x20..=0x7E).contains(&value) {value as char} else {'.'}).as_str(); 
            }
            write!(f, " | {}\n", char_data);
        }
        Ok(())
    }
}


pub const BLOCK_SIZE : usize = 512;
#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, PartialOrd, Ord)]
pub struct Block([u8; BLOCK_SIZE]);

impl Block {
    pub fn load(bus : u8, drive : u8, index : BlockIndex) -> Block {

        log!("Writing Block[{}][{}][{}]\n", bus, drive, index);

        let disk = get_disk(bus, drive).expect("Disk Not Found!");
        let data = get_sector(&disk, index).data();
        Block(data)
    }

    pub fn save(&self, bus : u8, drive : u8, index : BlockIndex) { 

        log!("Writing Block[{}][{}][{}]\n", bus, drive, index);

        let mut sector = Sector::new(index);
        for i in 0..BLOCK_SIZE {
            sector.write(i, self[i]);
        }
        let disk = get_disk(bus, drive).expect("Disk Not Found!");
        set_sector(&disk, index, &sector)
    }

    pub fn as_const_ptr(&self) -> ConstPointer<Block> {
        ConstPointer::from(self, BLOCK_SIZE)
    }

    pub fn as_mut_ptr(&mut self) -> MutPointer<Block> {
        MutPointer::from(self, BLOCK_SIZE)
    }

    pub fn set_data(&mut self, data : &[u8]) {
        assert!(data.len() <= BLOCK_SIZE);
        for i in 0..data.len() {
            self[i] = data[i];
        }
    }   

    pub fn from_const_ptr<T>(ptr : &ConstPointer<T>) -> Self {
        let mut data = vec![0; ptr.size()];
        ptr.copy_bytes(&mut data.as_mut_slice());
        let mut block = Block::default();
        block.set_data(data.as_slice());
        block
    }

    pub fn from_mut_ptr<T>(ptr : &MutPointer<T>) -> Self {
        let mut data = vec![0; ptr.size()];
        ptr.copy_bytes(&mut data.as_mut_slice());
        let mut block = Block::default();
        block.set_data(data.as_slice());
        block
    }

    pub fn from_disk_b(index : BlockIndex) -> Block {
        Block::load(0, 1, index)
    }
}

impl Default for Block {
    fn default() -> Self {
        Self([0x00; BLOCK_SIZE])
    }
}

impl Index<usize> for Block {
    type Output = u8;

    fn index(&self, index: usize) -> &Self::Output {
        assert!(index < BLOCK_SIZE);
        &self.0[index]
    }
}

impl IndexMut<usize> for Block {

    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        assert!(index < BLOCK_SIZE);
        &mut self.0[index]
    }
}





pub fn check_sizes() {
    serial_print!("Integriy Checks - ");
    assert!(size_of!(crate::kernel::drivers::file_systems::file_table::FileTable) <= 512);
    assert!(size_of!(crate::kernel::drivers::file_systems::file_table::FileTableEntry) <= 32);
    assert!(size_of!(crate::kernel::drivers::file_systems::file_table::file::FileInfo) <= 512);
    assert!(size_of!(crate::kernel::drivers::file_systems::file_table::file::IndexNode) <= 512);
    serial_println!("[OK]");
}

use lazy_static::lazy_static;
lazy_static! {
    static ref ROOT : Mutex<FileTable> = Mutex::new(FileTable::load_root((0,1)));
}

pub fn open_file(name : &str) -> usize {
    let mut file_handle : usize = 0;
    without_interrupts(|| {
        file_handle = if let Some(file) = ROOT.lock().search(name) {
            file.table_index() as usize
        } else {
            0
        }
    });
    file_handle
}

pub fn file_info<'a>(index : usize) -> &'a FileInfo {
    let mut file_handle : &FileInfo;
    without_interrupts(move || {
        file_handle = &ROOT.lock()[index].get_fileinfo();
    });
    file_handle
}