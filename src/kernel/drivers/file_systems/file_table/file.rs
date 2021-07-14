use alloc::vec::Vec;

use crate::kernel::drivers::file_systems::*;
use crate::size_of;
#[repr(C, align(512))]
#[derive(Debug, Clone, Copy)]
pub struct FileInfo {
    index_node_lead : BlockIndex
}

impl FileInfo {
    pub fn new(_sectors : Vec<Sector>) -> Self {
        let index_node_lead = 0;
        Self {
            index_node_lead,
        }
    }

    pub fn index_head(&self) -> IndexNode {
        let head = IndexNode::from_index(self.index_node_lead);
        head
    }

    pub fn get_data(&self) -> Vec<u8> {
        self.index_head().get_data()
    }

    pub fn set_data(&mut self, data : Vec<u8>) {
        let mut list = SectorList::new();
        for (index, byte) in data.iter().enumerate() {
            list.write_u8(index, *byte);
        }
    }

    pub fn get_mut_sectorlist(&mut self) -> SectorList {
        let mut sectors = SectorList::new();
        sectors
    }

    pub fn set_sectors(&mut self, sectors : &SectorList) {
        let mut head = self.index_head();
        head.clear_list();

        for sector in sectors.sectors() {
            head.add_data_sector(sector);
        }
    }
}
#[repr(C, align(512))]
#[derive(Debug, Clone, Copy)]
pub struct IndexNode {
    current : BlockIndex,
    next : BlockIndex,

    next_free : BlockIndex,
    next_entry : BlockIndex,

    data_blocks : [BlockIndex; 32],
}


impl IndexNode {
    pub fn create(index : BlockIndex) -> Self {
        Self {
            current : index,
            next : 0,
            data_blocks : [0; 32],
            next_free : (index + 1),
            next_entry : 0,
        }
    }


    /// Returns A Vector Of [Sector]s in the node (and all the following nodes) paired with the parent
    /// [IndexNode].
    pub fn sectors(&self) -> Vec<(IndexNode, Sector)> {
        let mut list : Vec<(IndexNode, Sector)> = Vec::new();
        for index in self.data_blocks {
            list.push((*self, Sector::new(index)));
        }
        let next = self.get_next();
        if next.is_some() {
            let node = next.unwrap();
            list.append(&mut node.sectors());
        }

        list
    }

    pub fn get_data(&self) -> Vec<u8> {
        let mut v : Vec<u8> = Vec::new();
        for (_, sector) in self.sectors() {
            for byte in sector.data() {
                v.push(byte);
            }
        }

        if let Some(next) = self.get_next() {
            v.append(&mut next.get_data());
        }

        v
    }

    pub fn add_data_sector(&mut self, sector : &Sector) {
        if self.next_free > 33 {
            self.add_index_node().add_data_sector(sector);
        } else {
            self.data_blocks[self.next_free as usize] = sector.index;
        }
    }

    pub fn add_block(&mut self, sector : &Block, index : BlockIndex) {
        if self.next_free > 33 {
            self.add_index_node().add_block(sector, index);
        } else {
            self.data_blocks[self.next_free as usize] = index;
        }
    }

    fn add_index_node(&mut self) -> IndexNode {
        let disk = &get_disk(0, 1).expect("Couldn't Find Drive B...");
        for sec_num in 0..get_disk_sector_count(0, 1) {
            if is_range_empty(disk, (sec_num as u32)..(sec_num as u32 + 33 as u32)) {
                self.next = sec_num as u32;
                return self.get_next().unwrap();
            }
        }
        panic!("Unable To Allocate New Index Node, Disk Is Full.",)
    }

    pub fn get_next(&self) -> Option<IndexNode> {
        if self.next == 0 {return None} else {

            let ptr = ConstPointer::from(&get_sector(&get_disk(0, 1).expect("Couldn't Find Drive B..."),self.next, ), size_of!(Sector));
            Some(*ptr.cast::<IndexNode>())
        }
    }

    pub fn as_const_ptr(&self) -> ConstPointer<IndexNode> {
        ConstPointer::from(self, size_of!(IndexNode))
    }

    pub fn from_block(data : &Block) -> IndexNode {
        let ptr = ConstPointer::from(data, size_of!(Block));
        *ptr.cast::<IndexNode>()
    }

    pub fn from_index(index : BlockIndex) -> IndexNode {
        let ptr = ConstPointer::from(&Block::load(0, 1, index), size_of!(Block));
        *ptr.cast::<IndexNode>()
    }

    pub fn clear_list(&mut self) {
        self.data_blocks = [0; 32];
        self.next_free = self.current + 1;
        self.next_entry = 0;
        if let Some(mut next) = self.get_next() {
            next.clear_list();
        }
    }

    
}


