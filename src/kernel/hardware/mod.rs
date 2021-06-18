pub mod pic;
pub mod pit;
pub mod vga;


pub use x86_64::instructions::port::*;

pub type PortRW<T> = Port<T>;
pub type PortRO<T> = PortReadOnly<T>;
pub type PortWO<T> = PortWriteOnly<T>;


pub struct InitError(&'static str);