pub mod graphics;
pub mod math;
pub mod time;
pub mod input;

pub struct Arguments {
    parent : &'static str
}

impl Arguments {
    pub fn empty() -> Self {
        Self {
            parent : "none"
        }
    }
}