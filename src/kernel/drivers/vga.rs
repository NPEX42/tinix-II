use volatile::Volatile;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct ColorAttribute {
    fg : u8,
    bg : u8,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
#[repr(C)]
pub struct Char {
    codepoint : u8,
    color : ColorAttribute,
}
#[derive(Clone)]
#[repr(transparent)]
pub struct TextBuffer {
    data : [[Volatile<Char> ; 80] ; 25]
}



