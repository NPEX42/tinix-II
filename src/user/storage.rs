use crate::kernel::drivers::file_systems::file_table::*;
pub use crate::kernel::drivers::file_systems::file_table::{FileTableEntry};
pub use FileTableEntry as File;
pub type DiskID = (u8, u8);
pub fn get_root(disk : DiskID) -> FileTable {
    FileTable::load_root(disk)
}

impl FileTable {
    pub fn open_file(&mut self, name : &str, mut entry_buffer : &mut FileTableEntry) {
        *entry_buffer = FileTableEntry::empty();
    } 
}