use core::fmt::Write;

pub mod devices;
pub mod fs;


pub trait IoReader<T> {
    fn read(&mut self) -> Option<T>;
}
pub trait IoWriter<T> : Write {
    fn write(&mut self, item : T);
}
