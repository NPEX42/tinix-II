use core::fmt::Arguments;
use x86_64::instructions::interrupts::without_interrupts;
use pc_keyboard::*;
use x86_64::instructions::port::*;
use lazy_static::lazy_static;
use spin::Mutex;

pub fn init() -> crate::kernel::InitResult<()> {
    crate::kernel::arch::set_interrupt(1, on_key_pressed)
    .expect("Unable To Setup Keyboard Interrupt");
    Ok(())
}

fn on_key_pressed(irq : u8) {
    without_interrupts(|| {
        KEYBOARD.lock().process_scancode();
    });
}

pub fn serial_read() -> Option<u8> {
    Some(crate::kernel::hardware::uart::read_u8())
}

pub fn serial_write(byte : u8){
    crate::kernel::hardware::uart::write_u8(byte);
}

pub macro serial_print {
    ($($arg:tt)*) => { $crate::user::input::_print(format_args!($($arg)*)) }
}

pub macro serial_println {
    ($($arg:tt)*) => { $crate::user::input::serial_print!("{}\r\n",format_args!($($arg)*)) }
}

pub fn _print(args : Arguments) {
    without_interrupts(|| {
        crate::kernel::hardware::uart::write_str(args);
    });
}
    

lazy_static! {
    static ref KEYBOARD : Mutex<KeyBoard> = Mutex::new(KeyBoard::new());
}


struct KeyBoard {
    last_key    : Option<char>,
    //IO Ports
    data    : Port<u8>, //Port 0x60
    //pc-keyboard
    kb      : Keyboard<layouts::Uk105Key, ScancodeSet1>
}

impl KeyBoard {
    pub fn new() -> Self {
        KeyBoard {
            last_key : None,
            data : Port::new(0x60),
            kb : Keyboard::new(layouts::Uk105Key, ScancodeSet1, HandleControl::Ignore)
        }
    }

    pub fn process_scancode(&mut self) {
        let mut result : Option<char> = None;
            result = {
                if let Ok(Some(event)) = self.kb.add_byte(unsafe {self.data.read()}) {
                    if let Some(key) = self.kb.process_keyevent(event) {
                        match key {
                            DecodedKey::Unicode(c) => Some(c),
                            _ => None
                        }
                    } else {None}
                } else {None}
            };
        self.last_key = result;
    }

    pub fn last_key(&self) -> Option<char> {
        self.last_key
    }
}

pub fn key() -> Option<char> {
    let mut lk : Option<char> = None;
    without_interrupts(|| {
        lk = KEYBOARD.lock().last_key();
    });
    lk
} 