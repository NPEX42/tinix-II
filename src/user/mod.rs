pub mod graphics;
pub mod math;

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