pub unsafe fn memcpy(source : *mut u8,dest : *mut u8, len : usize) {
    for idx in 0..len {
        *dest.offset(idx as isize) = *source.offset(idx as isize); 
    }
}

pub unsafe fn memset(dest : *mut u8, len : usize, value : u8) {
    for idx in 0..len {
        *dest.offset(idx as isize) = value; 
    }
}