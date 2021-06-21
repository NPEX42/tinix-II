    /// IN-TESTING, MAY HAVE UNDEFINED BEHAVIOUR
    pub fn vsync_wait() {
        unsafe {
            let mut vga = VGA.lock();
            crate::input::serial_println!("[{}]: MSR:{:08b}",file!(),vga.general_registers.read_msr());
            while (vga.general_registers.read_msr() & 1 << 3) == 0 {
                crate::input::serial_println!("VSync Bit: {:01b}", (vga.general_registers.read_msr() & 8) >> 3);
            }

            while (vga.general_registers.read_msr() & 1 << 3) == 1 {
                crate::input::serial_println!("DD Bit: {:}", (vga.general_registers.read_msr() & 1) == 1);
            }
        }
    }

    use lazy_static::lazy_static;
    use spin::*;

    pub use vga::colors::Color16;
    pub use vga::writers::{Graphics640x480x16, GraphicsWriter};
    pub use vga::fonts;
    use vga::vga::VGA;
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref GFX_MODE : Mutex<Graphics640x480x16> = Mutex::new(
            Graphics640x480x16::new()
        );
    }

    pub fn init() {
        GFX_MODE.lock().set_mode();
    }

    pub fn clear_screen(color : Color16) {
        GFX_MODE.lock().clear_screen(color);
    }

    pub fn draw(x : usize, y : usize, color : Color16) {
        GFX_MODE.lock().set_pixel(x,y,color);
    }

    pub fn draw_str(x : usize, y : usize, text : &str, color : Color16) {
        let mode = GFX_MODE.lock();
        for (offset, chr) in text.chars().enumerate() {
                mode.draw_character(x + offset * 8, y, chr, Color16::White);
        }
    }

    pub fn draw_chr(x : usize, y : usize, chr : char, color : Color16) {
        GFX_MODE.lock().draw_character(x, y, chr, Color16::White);
    }

    pub fn draw_line(start : (isize, isize), end : (isize, isize), color : Color16) {
        GFX_MODE.lock().draw_line(start, end, color);
    }


