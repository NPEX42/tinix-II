

    use lazy_static::lazy_static;
    use spin::*;

    pub use vga::colors::Color16;
    pub use vga::writers::{Graphics640x480x16, GraphicsWriter};
    pub use vga::fonts;


    use crate::kernel::InitResult;

    lazy_static! {
        static ref GFX_MODE : Mutex<Graphics640x480x16> = Mutex::new(
            Graphics640x480x16::new()
        );
    }

    pub fn init() -> InitResult<()> {
        GFX_MODE.lock().set_mode();
        Ok(())
    }

    pub fn clear_screen(color : Color16) {
        GFX_MODE.lock().clear_screen(color);
    }

    pub fn draw(x : usize, y : usize, color : Color16) {
        GFX_MODE.lock().set_pixel(x,y,color);
    }

    pub fn draw_str(x : usize, mut y : usize, text : &str, color : Color16) {
        let mode = GFX_MODE.lock();
        for (offset, chr) in text.chars().enumerate() {
                if chr == '\n' || x * 8 + offset > 80 {y += 1};
                mode.draw_character(x + offset * 8, y, chr, color);
        }
    }

    pub fn draw_chr(x : usize, y : usize, chr : char, color : Color16) {
        GFX_MODE.lock().draw_character(x, y, chr, color);
    }

    pub fn draw_line(start : (isize, isize), end : (isize, isize), color : Color16) {
        GFX_MODE.lock().draw_line(start, end, color);
    }


