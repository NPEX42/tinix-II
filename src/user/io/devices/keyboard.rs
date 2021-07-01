use crate::io::IoReader;

pub struct KeyBoard;

impl IoReader<char> for KeyBoard {
    fn read(&self) -> Option<char> {
        crate::input::key()
    }
}