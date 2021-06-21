use core::ops::RangeBounds;
use bit_field::BitField;

pub use crate::kernel::hardware::*;

#[derive(Debug, Clone)]
pub struct VgaController {
    pub index_reset     : PortRO<u8>,   //3DA
    pub index_reg       : PortRW<u8>,   //3C0
    pub outputr         : PortRO<u8>,   //3C2
    pub outputw         : PortWO<u8>,   //3CC
    pub index_0         : PortWO<u16>,  //3C4
    pub data_0          : PortWO<u8>,   //3C5
    pub index_1         : PortWO<u16>,  //3CE
    pub data_1          : PortWO<u8>,   //3CF
    pub index_2         : PortWO<u16>,  //3D4
    pub data_2          : PortWO<u8>,   //3D5
    pub dac_mask        : PortWO<u8>,   //3C6
    pub dac_ind_write   : PortWO<u8>,   //3C8,
    pub dac_color       : PortRW<u8>,   //3C9,
    pub dac_ind_read    : PortRO<u8>,   //3C7

    pub crt_controller  : CrtController,
}

impl VgaController {
    pub fn new() -> Self {
        Self {
            index_reset         : PortRO::new(0x3DA),
            index_reg           : PortRW::new(0x3C0),
            outputr             : PortRO::new(0x3C2),
            outputw             : PortWO::new(0x3CC),
            index_0             : PortWO::new(0x3C4),
            data_0              : PortWO::new(0x3C5),
            index_1             : PortWO::new(0x3CE),
            data_1              : PortWO::new(0x3CF),
            index_2             : PortWO::new(0x3D4),
            data_2              : PortWO::new(0x3D5),
            dac_color           : PortRW::new(0x3C9),
            dac_ind_read        : PortRO::new(0x3C7),
            dac_ind_write       : PortWO::new(0x3C8),
            dac_mask            : PortWO::new(0x3C6),
            crt_controller      : CrtController::new()
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrtController {
    pub horizontal_total    : CrtRegister,
    pub end_horizontal_disp : CrtRegister,
    pub start_hblanking     : CrtRegister,
    pub end_hblank          : CrtRegister,
    pub start_hretrace      : CrtRegister,
    pub end_hretrace        : CrtRegister,

    pub vert_total          : CrtRegister,
    pub overflow            : CrtRegister,
    pub max_scanline        : CrtRegister,
    pub vretrace_start      : CrtRegister,
    pub vretrace_end        : CrtRegister,
    pub vdisp_end           : CrtRegister,
    pub vblank_start        : CrtRegister,
    pub vblank_end          : CrtRegister,
    
    pub misc_output         : PortRW<u8>,
    pub clocking_mode       : PortRW<u8>
}


impl CrtController {
    pub fn new() -> Self {
        Self {
            horizontal_total    : CrtRegister::new(0),
            end_horizontal_disp : CrtRegister::new(1),
            start_hblanking     : CrtRegister::new(2),
            end_hblank          : CrtRegister::new(3),
            start_hretrace      : CrtRegister::new(4),
            end_hretrace        : CrtRegister::new(5),

            vert_total          : CrtRegister::new(6),
            overflow            : CrtRegister::new(7),
            max_scanline        : CrtRegister::new(9),
            vretrace_start      : CrtRegister::new(10),
            vretrace_end        : CrtRegister::new(11),
            vdisp_end           : CrtRegister::new(12),
            vblank_start        : CrtRegister::new(15),
            vblank_end          : CrtRegister::new(16),

            misc_output         : PortRW::new(0x3C2),
            clocking_mode       : PortRW::new(0x3C4),
            
        }
    }
}

#[derive(Debug, Clone)]
pub struct CrtRegister {
    port    : PortRW<u8>,
    index   : u8
}

impl CrtRegister {
    pub fn new(index : u8) -> Self {
        Self {
            port : PortRW::new(0x3D4),
            index
        }
    }

    pub fn read(&mut self) -> u8 {
        unsafe {
            self.port.write(self.index);
            self.port.read()
        }
    }

    pub fn write(&mut self, data : u8) {
        unsafe {
            self.port.write(self.index);
            self.port.write(data);
        }
    }

    pub fn get_bits<T: RangeBounds<usize>>(&mut self, range : T) -> u8 {
        self.read().get_bits(range)
    }

    pub fn set_bits<T: RangeBounds<usize> + Copy>(&mut self, range : T, bits : u8) {
        let mut value = self.read().get_bits(range);

        self.write(*(value.set_bits(range, bits)));
    }
}