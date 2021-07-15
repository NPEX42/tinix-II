
use core::ops::{Index, IndexMut};

use alloc::boxed::Box;

use super::*;

pub mod file;



pub const TABLE_SIZE : usize = 480;
pub const FT_ENTRY_ALIGN : usize = 32;

pub const ENTRIES_PER_TABLE : usize = TABLE_SIZE / FT_ENTRY_ALIGN;

#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FileType {
    Null = 0x00,
    File = 0x01,
    Directory = 0x02,
    Device = 0x03,
}

pub const FILE_TYPE : [FileType; 4] = [
    FileType::Null,
    FileType::File,
    FileType::Directory,
    FileType::Device
];

impl Display for FileType {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            FileType::Null => write!(f, "Empty"),
            FileType::File => write!(f, "File"),
            FileType::Directory => write!(f, "Dir"),
            FileType::Device => write!(f, "Device"),
        }
    }
}

#[repr(C, align(512))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTable {
    disk : (u8, u8),
    sector_index : BlockIndex,
    entries : [FileTableEntry; ENTRIES_PER_TABLE]
}

impl Default for FileTable {
    fn default() -> Self {
        FileTable::new(0)
    }
}

impl FileTable {
    pub fn new(sector : BlockIndex) -> FileTable {
        FileTable {
            disk : (0,1),
            sector_index : sector,
            entries : [FileTableEntry::empty(); ENTRIES_PER_TABLE]
        }
    }

    pub fn entries(&self) -> [FileTableEntry; ENTRIES_PER_TABLE] {
        self.entries
    }

    pub fn sector_index(&self) -> BlockIndex {
        self.sector_index
    }

    pub unsafe fn raw_entries(&self) -> (*const FileTableEntry, usize) {
        (self.entries.as_ptr(), self.entries.len())
    } 

    pub fn list(&self) -> impl Iterator<Item = &FileTableEntry> {
        self.entries.iter().filter(|entry| -> bool {
            return entry.file_type != FileType::Null;
        })
    }

    pub fn search(&self, name : &str) -> Option<FileTableEntry> {
        let cached_name : SmallString = SmallString::from_str(name);
        for entry in self.list() {
            if cached_name == entry.name() {
                return Some(*entry);
            }
        }
        None
    }

    pub fn create(&mut self, name : &str) -> Option<FileTableEntry> {
        let result = if let Some(mut entry) = self.get_first_mut_empty_ref() {
            entry.1.set_filetype(FileType::File);
            entry.1.set_filename(name);
            entry.1.set_table_index(entry.0);
            Some(entry.1)
        } else {
            panic!("Unable To File Empty File Slot, '{}'",name);
        };
        result
    }

    fn get_first_mut_empty_ref(&mut self) -> Option<(usize, FileTableEntry)> {
        for (index, entry) in self.entries.iter().enumerate() {
            if entry.filetype() == FileType::Null {
                return Some((index, *entry));
            };
        }
        None
    }

    pub fn update_on_disk(&self) {
        let ptr = ConstPointer::from(self, size_of!(FileTable));
        (*ptr.cast::<Block>()).save(self.disk.0, self.disk.1, self.sector_index)
    }

    pub fn load_root(disk : (u8, u8)) -> FileTable {
        let ptr = ConstPointer::from(&Block::load(disk.0, disk.1, 0), size_of!(FileTable));
        let mut table = *ptr.cast::<FileTable>();
        table.set_disk(disk);
        table
    }

    pub fn set_disk(&mut self, disk : (u8, u8)) {
        self.disk = disk;
    }

}

#[repr(C, align(32))]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct FileTableEntry {
    file_type : FileType,
    metanode_index : BlockIndex,
    bus : u8,
    disk : u8,
    table_index : usize,
    name : SmallString,
}

impl FileTableEntry {   

    pub fn name(&self) -> SmallString {
        self.name
    }

    pub fn empty() -> Self {
        Self {
            table_index : 0,
            metanode_index : 0,
            bus : 0,
            disk : 1,
            file_type : FileType::Null,
            name : SmallString::from_str(""),
        }
    }

    pub fn new_file<'a>(table_index : usize, name : &'a str, disk : &Disk, index : BlockIndex) -> Self {
        Self {
            metanode_index : index,
            table_index,
            file_type : FileType::File,
            bus : disk.bus,
            disk : disk.drive,
            name : SmallString::from_str(name),
        }
    }

    pub fn index(&self) -> BlockIndex {
        self.metanode_index
    }

    pub fn table_index(&self) -> usize {
        self.table_index
    }

    pub fn is_file(&self) -> bool {
        self.file_type == FileType::File
    }

    pub fn is_device(&self) -> bool {
        self.file_type == FileType::Device
    }

    pub fn is_dir(&self) -> bool {
        self.file_type == FileType::Directory
    }


    pub fn update_index(&mut self, index : BlockIndex) {
        self.metanode_index = index;
    }

    pub fn set_filetype(&mut self, filetype : FileType) {
        self.file_type = filetype;
    }

    pub fn set_filename(&mut self, name : &str) {
        self.name = SmallString::from_str(name);
    }

    pub fn set_table_index(&mut self, index : usize) {
        self.table_index = index;
    }



    pub fn filetype(&self) -> FileType {
        self.file_type
    }

    pub fn disk_id(&self) -> u8 {
        self.disk
    }

    pub fn bus_id(&self) -> u8 {
        self.bus
    }

    pub fn get_fileinfo(&self) -> FileInfo {
        let mut ptr = ConstPointer::from(&Block::from_disk_b(self.metanode_index), size_of!(Block));
        *ptr.cast::<FileInfo>()
    }

    

}

impl Into<FileTable> for FileTableEntry {
    fn into(self) -> FileTable {
        let disk = get_disk(self.bus,self.disk).unwrap();
        let sector = get_sector(&disk, self.metanode_index);
        sector.into()
    }
}

impl<> Into<FileTable> for Sector {
    fn into(self) -> FileTable {
       FileTable::new(0)
    }
}

impl Into<Sector> for FileTable {
    fn into(self) -> Sector {
        let sector = Sector::new(self.sector_index());
        
        sector
    }
}

impl Into<FileTableEntry> for [u8; FT_ENTRY_ALIGN] {
    fn into(self) -> FileTableEntry {
        let ptr = ConstPointer::from(&self, FT_ENTRY_ALIGN);
        *ptr.cast::<FileTableEntry>()
    }
}

impl Into<[u8; FT_ENTRY_ALIGN]> for FileTableEntry {
    fn into(self) -> [u8; FT_ENTRY_ALIGN] {
        let ptr = ConstPointer::from(&self, FT_ENTRY_ALIGN);
        let mut buffer = [0; FT_ENTRY_ALIGN];
        ptr.copy_bytes(&mut buffer);
        buffer
    }
}

impl Index<usize> for FileTable {
    type Output = FileTableEntry;
    fn index(&self, index: usize) -> &Self::Output {
        &self.entries[index]
    }
}

impl IndexMut<usize> for FileTable {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.entries[index]
    }
}

const SMALL_STR_LEN : usize = 8;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct SmallString {
    data : [u8; SMALL_STR_LEN]
}

impl SmallString {
    pub fn from_str(text : &str) -> SmallString {
        assert!(text.len() <= SMALL_STR_LEN);
        let mut buffer : [u8; SMALL_STR_LEN] = [0;SMALL_STR_LEN];
        for (index, byte) in text.bytes().enumerate() {
            buffer[index] = byte;
        }

        SmallString {
            data : buffer
        }
    }
    /// Converts this into a &[String] by appending the character data onto the end of the
    /// provided &[String]
    pub fn to_string(&self, text : &mut String) {
        for byte in self.data {
            if byte == 0 {return}
            text.push(byte as char);
        }
    }
}

impl Display for SmallString {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let mut s = String::new();
        self.to_string(&mut s);
        write!(f, "{}", s)
    }
}