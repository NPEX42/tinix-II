pub const BYTES_PER_SECTOR : usize = 512;
pub const SECTORS_PER_MB   : usize = 2048;
pub const MB_PER_DISK      : usize = 1;
pub const SECTORS_PER_DISK : usize = MB_PER_DISK * SECTORS_PER_MB;

pub const MAX_STRUCTURE_SIZE : usize = BYTES_PER_SECTOR * 1;
use crate::{data::boxed::*, };//fs::BinarySerializable};


pub trait Read {
    fn read(&self, sector : usize, offset : usize) -> u8;
}

pub trait Write {
    fn write(&mut self, sector : usize, offset : usize, data : u8);
}

pub trait ReadWrite : Read + Write {}

#[derive(Debug, Clone)]
pub struct Disk {
    sectors : Box<[Sector; SECTORS_PER_DISK]>
}

#[derive(Debug, Copy, Clone)]
pub struct Sector {
    block : [u8; BYTES_PER_SECTOR]
}

impl Sector {
    pub fn empty() -> Sector {
        Self {
            block : [0; BYTES_PER_SECTOR]
        }
    }

    pub fn read(&self, index : usize) -> u8 {
        self.block[index]
    }

    pub fn write(&mut self, index : usize, data : u8) {
        self.block[index] = data
    }
}

fn to_sector_index(index : usize) -> usize {
    index / BYTES_PER_SECTOR
}

fn to_offset(index : usize) -> usize {
    index % BYTES_PER_SECTOR
}

pub fn to_sector_offset(index : usize) -> (usize, usize) {
    (to_sector_index(index), to_offset(index))
}

impl Read for Disk {
    fn read(&self, sector : usize, offset : usize) -> u8 {
        self.sectors[sector].read(offset)
    }
}

impl Write for Disk {
    fn write(&mut self, sector : usize, offset : usize, data : u8) {
        self.sectors[sector].write(offset, data);
    }
}

impl ReadWrite for Disk {}

impl Disk {
    pub fn new() -> Disk {
        Disk {
            sectors : Box::new([Sector::empty(); SECTORS_PER_DISK])
        }
    }

    pub fn sector_containing_addr(&self, address : usize) -> Sector {
        self.sectors[to_sector_index(address)]
    }

    pub fn write_slice(&mut self, address : usize, data : &[u8]) {
        for i in 0..data.len() {
            let (sector, offset) = to_sector_offset(i + address);
            self.write(sector, offset, data[i])
        }
    }

    pub fn write_sector(&mut self, sector : usize, data : [u8; BYTES_PER_SECTOR]) {
        for i in 0..BYTES_PER_SECTOR {
            self.write(sector, i, data[i]);
        }
    }

    pub unsafe fn write_ptr(&mut self, pointer : *const u8, size : usize, address : usize) {
        for i in 0..size {
            let (sector, offset) = to_sector_offset(address);
            self.write(sector, offset, *(pointer.offset(i as isize)))
        }
    }

    // pub fn write_struct<T : BinarySerializable>(&mut self, address : usize, structure : &T) {
    //     let data = structure.serialize();
    //     self.write_slice(address, data);
    // }

    pub fn read_slice(&mut self, buffer : &mut [u8], address : usize) {
        for i in 0..buffer.len() {
            let (sector, offset) = to_sector_offset(i + address);
            buffer[i] = self.read(sector, offset);
        }
    }

    // pub fn read_struct<T : BinarySerializable>(&mut self, address : usize) -> T {
    //     let data : &mut [u8] = &mut [0;MAX_STRUCTURE_SIZE]; //Maxium Structure Size 2KB
    //     self.read_slice(data, address);
    //     T::deserialize(data)
    // } 
}