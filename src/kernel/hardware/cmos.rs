use core::fmt::Display;

use x86_64::instructions::port::*;

use crate::time;


const UPDATE_IN_PROGRESS_BIT : usize = 1 << 7;
const BCD_MODE : usize = 1 << 1;
const HOUR_24  : usize = 1 << 2;
pub struct Cmos {
    index_reg   : Port<u8>, // 0x70
    data_reg    : Port<u8>, // 0x71
}

#[allow(unused)]
pub struct Rtc {
    second  : u8, // 0..59
    minute  : u8, // 0..59
    hour    : u8, // 24Hr: 0..23, 12Hr: 1..12 (msb set if pm)
    weekday : u8, // Note: OSDev Wiki states that this shouldn't be used. Should be set to zero.
    day     : u8, // 1..31
    month   : u8, // 1..12
    year    : u8, // 0..99
    // Note: Maybe add support for the Century register?
}

#[repr(C)]
pub enum RtcIndexes {
    Seconds     = 0x00,
    Minutes     = 0x02,
    Hours       = 0x04,
    Weekday     = 0x06,
    DayOfMonth  = 0x07,
    Month       = 0x08,
    Year        = 0x09,
    
    StatusA     = 0x0A,
    StatusB     = 0x0B,
}

impl Cmos {
    pub fn get() -> Cmos {
        Cmos {
            index_reg : Port::new(0x70),
            data_reg  : Port::new(0x71),
        }
    }

    pub fn read_rtc(&mut self, index : RtcIndexes) -> u8 {
        unsafe { 
            self.index_reg.write(index as u8);
            self.data_reg.read()
        }
    }

    fn update_in_progress(&mut self) -> bool {
        (self.read_rtc(RtcIndexes::StatusA) & UPDATE_IN_PROGRESS_BIT as u8) > 0 
    }

    pub fn bcd_mode(&mut self) -> bool {
        (self.read_rtc(RtcIndexes::StatusB) & BCD_MODE as u8) < 1
    }

    pub fn hour_mode(&mut self) -> bool {
        (self.read_rtc(RtcIndexes::StatusB) & HOUR_24 as u8) > 0
    } 

    pub fn rtc(&mut self) -> Rtc {



        self.wait_for_update_completion();
        let second = self.read_rtc(RtcIndexes::Seconds);
        let minute = self.read_rtc(RtcIndexes::Minutes);
        let hour   = self.read_rtc(RtcIndexes::Hours);
        let day    = self.read_rtc(RtcIndexes::DayOfMonth);
        let year   = self.read_rtc(RtcIndexes::Year);
        let month  = self.read_rtc(RtcIndexes::Month);

        Rtc {
            day,
            hour,
            minute,
            month,
            second,
            weekday : 0,
            year
        }
    }

    pub fn wait_for_update_completion(&mut self) {
        while self.update_in_progress() {
            time::sleep_ticks(1); //Sleep for 1ms
        }
    }

}

impl Rtc {
    pub fn bcd_mode(&self) -> bool {
        (Cmos::get().read_rtc(RtcIndexes::StatusB) & BCD_MODE as u8) > 1
    }

    pub fn hour_mode(&self) -> bool {
        (Cmos::get().read_rtc(RtcIndexes::StatusB) & HOUR_24 as u8) > 1
    } 

    pub fn status_a(&self) -> u8 {
        Cmos::get().read_rtc(RtcIndexes::StatusA)
    }

    pub fn status_b(&self) -> u8 {
        Cmos::get().read_rtc(RtcIndexes::StatusA)
    }
}

impl Display for Rtc {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}/{}/20{} {}:{}:{:x}, 24Hr: {}, BCD: {}, StatusA: {:08b}, StatusB: {:08b}", 
        self.day,
        self.month, 
        self.year, 
        self.hour, 
        self.minute, 
        self.second, self.bcd_mode(), self.hour_mode(),
        self.status_a(), self.status_b())

    }
}