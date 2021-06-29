use core::{usize};

use alloc::{collections::BinaryHeap, string::String, vec::Vec};
use bytes::Buf;

pub use crate::kernel::drivers::ram_fs::*; 

pub fn create_disk(name : &'static str) -> Disk {
    let mut disk = Disk::new();
    disk.write_struct(0, &FatEntry {name : String::from("Hello.txt"), sectors : Vec::new()});
    disk
}

pub fn translate_index(index : usize) -> (usize, usize) {
    to_sector_offset(index)
}

struct FileAttributeTable {
    entries : Vec<FatEntry>
}

struct FatEntry {
    name : String,
    sectors : Vec<usize>
}

impl BinarySerializable for FatEntry {
    fn serialize(&self) -> &[u8] {
        let mut bin = Vec::new();
        bin.append(&mut Vec::from(self.name.serialize()));
        bin.append(&mut Vec::from(self.sectors.serialize()));
        bin.as_slice()
    }

    fn deserialize(data : &[u8]) -> Self {
        todo!()
    }
}


pub trait BinarySerializable : Sized {
    fn serialize(&self) -> &[u8;T];
    fn deserialize(data : &[u8]) -> Self;
}

impl BinarySerializable for String {
    fn serialize(&self) -> &[u8] {
        self.as_bytes()
    }

    fn deserialize(data : &[u8]) -> Self {
        String::from_utf8(Vec::from(data)).expect("Unable To Load String...")
    }
}

impl<T : BinarySerializable> BinarySerializable for Vec<T> {
    fn serialize(&self) -> &[u8] {
        let entries = self.as_slice();
        let mut bytes : Vec<u8> = Vec::new();
        bytes.append(&mut Vec::from(self.len().serialize()));
        for entry in entries {
            bytes.append(&mut Vec::from(entry.serialize()));
        }

        bytes.as_slice()
    }

    fn deserialize(data : &[u8]) -> Self {
        todo!()
    }
}


impl BinarySerializable for usize {
    fn serialize(&self) -> &[u8] {
        self.to_be_bytes()
    }

    fn deserialize(data : &[u8]) -> usize {
        let buf = Vec::from(&data[..]);
        0
    }
}

#[cfg(arch = "x86")]
impl BinarySerializable for usize {
    fn serialize(&self) -> &[u8] { 
        self.to_be_bytes()
    }

    fn deserialize(data : &[u8]) -> Self {
        let buf = data[..];
        Buf::get_u32(buf)
    }
}