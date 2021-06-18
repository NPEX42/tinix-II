

pub extern "C" fn print(msg : &str) {
    crate::print!("{}", msg);
}