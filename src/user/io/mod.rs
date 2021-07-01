use core::fmt::Write;

pub mod devices;


pub trait IoReader<T> {
    fn read(&self) -> Option<T>;
}
pub trait IoWriter<T> : Write {
    fn write(&mut self, item : T);
}
