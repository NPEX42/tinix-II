pub mod mem {
    use x86_64::instructions::interrupts::without_interrupts;
    use crate::kernel::arch::x64::mem::allocator::ALLOCATOR;
    pub fn total() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().size()
        });
        total
    }

    pub fn free() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().free()
        });
        total
    }

    pub fn used() -> usize {
        let mut total = 0;
        without_interrupts(|| {
            total = ALLOCATOR.lock().used()
        });
        total
    }


}