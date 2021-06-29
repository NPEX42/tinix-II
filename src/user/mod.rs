pub mod graphics;
pub mod math;
pub mod time;
pub mod input;
// pub mod fs;


#[cfg(feature = "liballoc")]
pub use alloc as data;

pub struct Arguments {
    parent : &'static str,
    args   : Option<&'static [&'static str]> 
}

impl Arguments {
    pub fn empty() -> Self {
        Self {
            parent : "none",
            args : None
        }
    }

    pub fn new(parent : &'static str, args : &'static str) -> Self {
        Self {
            parent,
            args : None
        }
    }

    pub fn parent(&self) -> &'static str {
        self.parent
    }

    pub fn args(&self) -> Option<&'static [&'static str]> {
        self.args
    }
}