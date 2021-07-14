pub mod console;
pub mod serial;
pub mod keyboard;

pub use console::Console;
pub use serial::Serial;
pub use keyboard::KeyBoard;
// TODO: Move input Items into here.
#[macro_export]
macro_rules! log {
    () => {
        $crate::io::devices::console::_print(format_args!(""));
        $crate::input::serial_print!();
    };

    ($($args:tt)*) => {
        $crate::io::devices::console::_print(format_args!($($args)*));   
        $crate::input::serial_print!($($args)*);
    };
}

#[macro_export]
macro_rules! clear_console {
    () => {
        $crate::io::devices::console::clear();
    };
}

#[macro_export]
macro_rules! home_console {
    () => {
        $crate::io::devices::console::home();
    };
}

#[macro_export]
macro_rules! reset_console {
    () => {
        $crate::clear_console!();
        $crate::home_console!();
    };
}

#[macro_export]
macro_rules! foreground {
    ($color : expr) => {
        $crate::io::devices::console::foreground($color);
    };
}

#[macro_export]
macro_rules! background {
    ($color : expr) => {
        $crate::io::devices::console::background($color);
    };
}


#[macro_export]
macro_rules! clear_row {
    () => {
        $crate::io::devices::console::clear_current_row();
        $crate::io::devices::console::carriage_return();
    };
}

