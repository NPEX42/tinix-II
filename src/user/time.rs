use spin::Mutex;
use lazy_static::lazy_static;
use x86_64::instructions::interrupts::{without_interrupts, enable_and_hlt};

pub struct Counter(u128);

pub const TICKS_PER_SECOND : usize = 1000;

impl Counter {
    pub fn ticks(&self) -> u128 { self.0 }
    pub fn inc(&mut self) { self.0 += 1; }
}

lazy_static! {
    static ref COUNTER : Mutex<Counter> = Mutex::new(Counter(0));
}

pub fn ticks() -> u128 {
    let mut t : u128 = 0;
    without_interrupts(|| {
        t = COUNTER.lock().ticks()
    });
    t
}

pub fn update() {
    without_interrupts(|| {
        COUNTER.lock().inc();
    });
}

pub fn sleep_ticks(ticks : usize) {
    for _ in 0..ticks {
        enable_and_hlt();
    }
}

pub fn sleep(seconds : f64) {
    sleep_ticks((seconds * TICKS_PER_SECOND as f64) as usize)
}


