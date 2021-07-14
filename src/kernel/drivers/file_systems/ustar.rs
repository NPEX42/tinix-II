use core::{fmt::{Display, Write}, mem::size_of, usize};

use alloc::{collections::BTreeMap, string::String, vec::Vec};

use crate::{io::{IoReader, IoWriter}, kernel::{self, InitResult}, log};
use super::*;

use super::File;
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};
lazy_static! {
    static ref ACTIVE_PARTITION : Mutex<SectorList> = Mutex::new(load_fs(1, 1, 1));
    static ref FILE_TABLE : Mutex<FileTable> = Mutex::new(FileTable::from_sector(
        load_fs(1, 1, 1).get_sector(0).unwrap()
    ));
}

#[derive(Debug, Clone, Copy)]
pub struct MetaNode {
    name          : [ u8; 100 ],
    mode          : [ u8; 8   ],
    owner_id      : [ u8; 8   ],
    group_id      : [ u8; 8   ],
    file_size     : [ u8; 12  ],
    modif_time    : [ u8; 12  ],
    checksum      : [ u8; 8   ],
    file_type     : [ u8; 1   ],
    linked_name   : [ u8; 100 ],
    magic_id      : [ u8; 6   ],
    magic_version : [ u8; 2   ],
    owner_name    : [ u8; 32  ],
    group_name    : [ u8; 32  ],
    device_major  : [ u8; 8   ],
    device_minor  : [ u8; 8   ],
    name_prefix   : [ u8; 155 ],
    usable        : [ u8; 1   ],
    padding       : [ u8; 11  ]
}

impl MetaNode {
    pub fn from_sector(sector : [u8; 512]) -> Self {
        copy!(0, 100, sector, name);
        copy!(100, 8, sector, mode);
        copy!(108, 8, sector, owner_id);
        copy!(116, 8, sector, group_id);
        copy!(124, 12, sector, file_size);
        copy!(136, 12, sector, modif_time);
        copy!(148, 8, sector, checksum);
        copy!(156, 1, sector, file_type);
        copy!(157, 100, sector, linked_name);
        copy!(257, 6, sector, magic_id);
        copy!(263, 2, sector, magic_version);
        copy!(265, 32, sector, owner_name);
        copy!(297, 32, sector, group_name);
        copy!(329, 8, sector, device_major);
        copy!(337, 8, sector, device_minor);
        copy!(345, 155, sector, name_prefix);

        Self {
            name,
            mode,
            checksum,
            device_major,
            device_minor,
            file_size,
            file_type,
            group_id,
            group_name,
            linked_name,
            magic_id,
            magic_version,
            modif_time,
            name_prefix,
            owner_id,
            owner_name,
            padding : [0; 11],
            usable : [1; 1],
        }
    }

    pub fn new(file : &TarFile) -> Self {
        Self {
            name : str_to_slice!(file.name, 100),
            checksum : bin_to_oct!(0,8),
            device_major : bin_to_oct!(0, 8),
            device_minor : bin_to_oct!(0, 8),
            file_size : bin_to_oct!(file.data.len(), 12),
            file_type : [b'0'; 1],
            group_id : bin_to_oct!(0, 8),
            group_name : bin_to_oct!(0, 32),
            linked_name : str_to_slice!(file.name, 100),
            magic_id : str_to_slice!("USTAR\0", 6),
            magic_version : str_to_slice!("00", 2), 
            mode : bin_to_oct!(0, 8),
            modif_time: bin_to_oct!(0, 12),
            name_prefix : str_to_slice!("", 155),
            owner_id : bin_to_oct!(0, 8),
            owner_name : str_to_slice!("UNK", 32),
            padding : [0; 11],
            usable : [1 ; 1],
        }
    }

    pub fn name(&self) -> String {
        str_from_slice!(self.name)
    }

    pub fn owner(&self) -> (String, usize) {
        (str_from_slice!(self.owner_name), slice_to_bin!(self.owner_id))
    }

    pub fn group(&self) -> (String, usize) {
        (str_from_slice!(self.group_name), slice_to_bin!(self.group_id))
    }

    pub fn file_size(&self) -> usize {
        slice_to_bin!(self.file_size)
    }

    pub fn file_sector_length(&self) -> usize {
        let mut sectors = self.file_size() / 512;
        if self.file_size() % 512 > 0 {
            sectors += 1;
        }
        sectors
    }

    pub fn file_type(&self) -> String {
        str_from_slice!(self.file_type)
    }


    pub fn as_slice(&self) -> [u8; 512] {
        let mut buffer : [u8; 512] = [0; 512];
        insert_slice(&self.name, &mut buffer, 0);
        insert_slice(&self.file_type, &mut buffer, 156);
        insert_slice(&self.file_size, &mut buffer, 124);
        insert_slice(&self.magic_id, &mut buffer, 257);
        insert_slice(&self.magic_version, &mut buffer, 263);
        
        insert_slice(&bin_to_oct!(MetaNode::gen_checksum(buffer), 8), &mut buffer, 148);

        buffer
    }

    pub fn gen_checksum(buffer : [u8; 512]) -> usize {
        let mut sum : usize = 0;
        for i in 0..512 {
            sum += buffer[i] as usize;
        }
        sum
    }

}

impl Display for MetaNode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "File Name: {}\n", self.name());
        write!(f, "File Size: {}\n", self.file_size());
        write!(f, "File Sectors: {}\n", self.file_sector_length());
        write!(f, "File Owner: {}\n", self.owner().0);
        write!(f, "File Type: {}\n", self.file_type())
    }
}

pub fn check_meta_block_size() {
    assert_eq!(size_of::<MetaNode>(), 512);
    log!("Passed MetaNode Size Check...\n");
}

pub fn get_file_data<'a>(_archive : &'a [u8], _name : &'a str) -> Vec<u8> {
    let data : Vec<u8> = Vec::new();

    data
}

pub fn oct2bin(value : &str) -> usize {
    usize::from_str_radix(value, 8).expect("Invalid Octal String")
}

macro copy($start : expr, $size : expr, $src : expr, $dest : ident) {
    let mut $dest = [0; $size];
    $dest.copy_from_slice(&$src[$start..($start + $size)]);
}

macro str_from_slice($slice : expr) {
    {
        let mut s = alloc::string::String::with_capacity($slice.len());
        for item in $slice {
            s.push(item as char);
        }
        s
    }
}

macro slice_to_bin($slice : expr) {
    {
        let string = str_from_slice!($slice);
        oct2bin(&string)
    }
}

macro str_to_slice($text : expr, $size : expr) {
    {
        let mut slice : [u8; $size] = [0; $size];
        for i in 0..$text.len() {
            slice[i] = ($text.as_bytes())[i]
        }
        slice
    }
}

macro bin_to_oct($val : expr, $size : expr) {
    {
        let mut s = String::new();
        s.write_fmt(format_args!("{:0width$o}", $val, width = $size)).expect("Format Error");
        str_to_slice!(s, $size)
    }
}



#[derive(Debug)]
pub struct TarFile {
    data : Vec<u8>,
    name : String,
}

impl TarFile {
    pub fn from_metanode(node : MetaNode, sectors : &SectorList, node_index : usize) -> TarFile {

        let sector_length = node.file_sector_length();
        let file_size = node.file_size();

        let mut binary = Vec::new();

        'sector_loop: for i in node_index+1..(sector_length + node_index + 1) {
            if let Some(sector) = sectors.get_sector(i) {
                //log!("sector {:02x}\n", i);
                for byte in sector.data() {
                    //log!("byte {}\n", byte as char);
                    binary.push(byte);
                    if binary.len() > file_size  {break 'sector_loop}
                }
            }
        }

        TarFile {
            data : binary,
            name : node.name(),
        }
    }
}

impl File<u8> for TarFile {
    fn open(name : &str) -> Self {
        Self {
            data : Vec::new(),
            name : String::from(name.trim_matches('\0'))
        }
    }

    //TODO: Have this call save beforehand.
    fn close(&mut self) {
        let file_table = FILE_TABLE.lock();
        save_fs(1, 1, &self.get_sectors(), file_table.table.get(self.name.as_str()).unwrap().1 as u32);

    }

    fn read_all(&mut self) -> Vec<u8> {
        let mut v = Vec::new();
        while let Some(b) = self.read() {
            v.push(b);
        }
        v.reverse();
        v
    }

    fn write_all(&mut self, data : Vec<u8>) {
        for b in data {
            self.write(b);
        }
    }

    fn get_sectors(&self) -> SectorList {
        let mut list = SectorList::new();
        for idx in 0..self.data.len() {
            list.write(idx, self.data[idx])
        }

        let mut meta_sector = SectorList::new();
        let mut idx = 0;
        for byte in MetaNode::new(self).as_slice() {
            meta_sector.write(idx, byte);
            idx += 1;
        }
        meta_sector.append(&mut list);
        meta_sector
    }

     
}

impl IoReader<u8> for TarFile {
    fn read(&mut self) -> Option<u8> {
        self.data.pop()
    }
}

impl IoWriter<u8> for TarFile {
    fn write(&mut self, item : u8) {
        self.data.push(item);
    }
}

impl Write for TarFile {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            self.write(byte);
        }
        Ok(())
    }
}

pub fn insert_slice(src : &[u8], dest : &mut [u8], start : usize) {
    assert!(src.len() + start < dest.len());
    for i in start..(start + src.len()) {
        dest[i] = src[i - start];
    }
}

pub fn list_files() -> Vec<(usize, String, MetaNode)> {
    let sectors = ACTIVE_PARTITION.lock();
    let mut names = Vec::new();
    let mut idx = 0;
    for sector in sectors.sectors() {
        let node = MetaNode::from_sector(sector.data());
        if !node.name().is_empty() && node.file_type() == "0" {
            names.push((idx, node.name(), node));
        }
        idx += 1;
    }
    names
}

pub fn search(name : &str) -> Option<TarFile> {
    if let Some((node,index)) = FILE_TABLE.lock().table.get(name) {
        return Some(TarFile::from_metanode(*node, &(*ACTIVE_PARTITION.lock()), *index));
    }
    None
    
}

pub fn load_fs(bus : u8, disk : u8, sector_count : usize) -> SectorList {
    let mut sectors = SectorList::with_capacity(sector_count);

    kernel::hardware::ata::read_sectors(bus, disk, 0, &mut sectors);
    sectors
}

pub fn save_fs(bus : u8, disk : u8, sectors : &SectorList, start_block : u32) {
    kernel::hardware::ata::write_sectors(bus, disk, start_block, sectors);
    
}

pub fn open_file(name : &str) -> Option<TarFile> {
    if file_exists(name) {
        search(name)
    } else {
        file_create(name)
    }
}

pub fn file_exists(name : &str) -> bool {
    FILE_TABLE.lock().table.contains_key(name)
}

pub fn file_create(name : &str) -> Option<TarFile> {
    let file = TarFile::open(name);
    Some(file)
}

#[derive(Debug)]
pub struct FileTable {
    table : BTreeMap<String, (MetaNode, usize)>
}

impl FileTable {
    pub fn new() -> Self {
        let mut table  = BTreeMap::new();

        for (index,mut name, node) in list_files() {
            let _ = 0;

            name = String::from(name.trim_end_matches('\0'));
            table.insert(name, (node, index));
        }

        Self {
            table
        }
    }

    pub fn from_sector(sector : &Sector) -> FileTable {
        let mut table : BTreeMap<String, (MetaNode, usize)>  = BTreeMap::new();

        let raw = sector.data();


        let mut entry_list : Vec<u16> = Vec::with_capacity(128);

        let mut i = 0;
        while i < 128 {
                    
        let entry = u16::from_be_bytes([
            raw[i + 0],
            raw[i + 1]
        ]);

        if entry > 0 {
            entry_list.push(entry);
        }

        i += size_of::<u16>();


        }
        let active_partition = ACTIVE_PARTITION.lock();
        for entry in entry_list {
            if let Some(sector) = active_partition.get_sector(entry as usize) {
                let node = MetaNode::from_sector(sector.data());
                let mut name = node.name();
                name = String::from(name.trim_end_matches('\0'));
                table.insert(name, (node, entry as usize));
            }

        }

        FileTable {
            table
        }
    }

    pub fn to_sector(&mut self) -> Sector {
        let mut sector = Sector::new(0);
        let mut idx = 5;
        for entry in &self.table {
            sector.data_mut()[idx + 0] = (entry.1.1 >> 8) as u8;
            sector.data_mut()[idx + 1] = (entry.1.1 >> 0) as u8;

            idx += 2;
        }
        sector
    }

    pub fn next_free_block(&self) -> usize {
        let next = 0;
        let _max_sector = 0;
        next
    }
}

pub fn file_table() -> MutexGuard<'static, FileTable> {
    FILE_TABLE.lock()
}

pub fn init() -> InitResult<()> {
    ACTIVE_PARTITION.lock();
    FILE_TABLE.lock();

    Ok(())
}


pub const FILE_TABLE_ENTRY_COUNT_INDEX : usize = 0x005;