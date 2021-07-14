# Tinix FS - Reference

## For Tinix V0.10.3 -> Latest

## Overview

## Components

- File Table - 512 bytes
- File Table Entry - 16 bytes.
- File Info - 512 bytes.
- Index Node - 512 bytes
- Sector - 512 bytes
- Sector List - 512 bytes / item.

## Structures

```rust
pub type BlockIndex = u32;

pub struct FilePermissions(u8);

pub struct File {
    table_index : usize,
    data : Vec<u8>
}
#[repr(u8)]
pub enum FileType {
    Null,
    File,
    Directory,
    Device,
    Pipe,
}

#[repr(C, align(512))]
pub struct FileTable {
    sector_num : BlockIndex, //Location On The Disk
    next_table_num : BlockIndex, //Location of the next table. 0 if this is the last one.
    next_free_entry : usize, // Where To put the next file, if it is greater than
    						 // The length of file_entries, add to the next table.
    file_entries : [FileTableEntry; 32] //32 Files per table.
}

#[repr(C, align(16))]
pub struct FileTableEntry {
    file_info_index : BlockIndex, //Location of the file info on the disk
    // What permissions does this file have? [Not Implemented.]
    permissions : FilePermissions,
    owner_id : u8, // Who Owns this file? [Not Implemented]
}

#[repr(C, align(512))]
pub struct FileInfo {
    sector_num : BlockIndex, // Location On Disk
    name : String, // Name of the file
    filetype : FileType,
    
    index_nodes : BlockList<IndexNode>, // The First Index Node in the list
}

#[repr(C, align(512))]
pub struct IndexNode {
    next_node : BlockIndex, // The Next Index Node in the list
    indexes : [BlockIndex, 63], // All the blocks containing file data
}

#[repr(C, align(512))]
pub struct DataNode {
    sector_num : BlockIndex,
    data : [u8; 512] // The File Data
}

#[repr(C, align(512))]
pub struct Block {
    index : BlockIndex,
    data : [u8; 512]
}

pub struct ConstPointer<T> {
    raw_ptr : *const T
}

pub struct MutPointer<T> {
    raw_ptr : *mut T,
}

pub struct BlockList<T> {
    begin : BlockIndex,
    length : usize,
    blocks : Vec<T>,
}

```



## Component Functions

```rust
fn FileTable::new(sector : BlockIndex) -> FileTable;
fn FileTable::index(&self, index : usize) -> &FileTableEntry;
fn FileTable::index_mut(&mut self, index : usize) -> &mut FileTableEntry

fn File::new(path : String) -> File
fn File::delete(&mut self);
fn File::close(&mut self);
fn File::read_all<'a>(&self) -> &'a [u8];
fn File::write(&mut self, data : &[u8]);

fn FilePermissions::new() -> FilePermissions;
fn FilePermissions::set_readable(&mut self, value : bool);
fn FilePermissions::set_writable(&mut self, value : bool);
fn FilePermissions::set_deletable(&mut self, value : bool);
fn FilePermissions::set_executable(&mut self, value : bool);
fn FilePermissions::is_readable(&self) -> bool;
fn FilePermissions::is_writable(&self) -> bool;
fn FilePermissions::is_deletable(&self) -> bool;
fn FilePermissions::is_executable(&self) -> bool;

fn FileTableEntry::new(
    inf_index: BlockIndex, 
    permissions : FilePermissions, 
    owner : u8
) -> Self;
fn FileTableEntry::clear(&mut self);
fn FileTableEntry::file_info(&self) -> &FileInfo;

fn FileInfo::new(name : String, permissions : FilePermissions)
fn FileInfo::name(&self) -> String;
fn FileInfo::permissions(&self) -> FilePermissions;
fn FileInfo::owner_id(&self) -> u8;
fn FileInfo::filetype(&self) -> FileType;
fn FileInfo::indexes(&self) -> &Vec<IndexNode>;

fn Block::new(index : BlockIndex) -> self;
fn Block::index(&self, index : usize) -> &u8;
fn Block::index_mut(&mut self, index : usize) -> &mut u8;
fn Block::write_ptr<T>(&mut self, ptr : ConstPointer<T>);
fn Block::read_ptr<T>(&mut self) -> ConstPointer<T>;

fn ConstPointer<T>::new(item : &T) -> self;
fn ConstPointer<T>::from_raw_ptr(ptr : *const T) -> self;
fn ConstPointer<T>::as_slice(&self) -> &[u8];
fn ConstPointer<T>::cast_to<C>(&self) -> ConstPointer<C>;

fn MutPointer<T>::new(item : &T) -> self;
fn MutPointer<T>::from_raw_ptr(ptr : *mut T) -> self;
fn MutPointer<T>::as_slice(&self) -> &[u8];
fn MutPointer<T>::cast_to<C>(&self) -> MutPointer<C>;

fn 

```

