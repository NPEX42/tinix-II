pub const HEAP_START: usize = 0x4444_4444_0000;
#[cfg(feature = "HEAP128")]
pub const HEAP_SIZE: usize =  128 * MB; // 1MB

#[cfg(feature = "HEAP16")]
pub const HEAP_SIZE: usize =  16 * MB; // 1MB

#[cfg(feature = "HEAP32")]
pub const HEAP_SIZE: usize =  32 * MB; // 1MB

pub const KB : usize = 1024;
pub const MB : usize = KB * 1024;
pub const GB : usize = MB * 1024;

