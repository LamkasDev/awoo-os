use crate::{driver::logger::logger::println, screen::queue::trigger_screen_frame};
use spin::Mutex;

pub static TIME: Mutex<u64> = Mutex::new(0);
pub static TICKS: Mutex<u64> = Mutex::new(0);

pub const RATE: u8 = 9;
pub const TICKS_PER_SECOND: u64 = 32768 >> (RATE as u64 - 1);
pub const TICKS_PER_SECOND_F: f64 = TICKS_PER_SECOND as f64;
pub const TICKS_PER_FRAME: u64 = TICKS_PER_SECOND / 1;

pub fn common_timer_code() {
    let mut prev_ticks = TICKS.lock();
    match prev_ticks.checked_add(1) {
        Some(ticks) => {
            *prev_ticks = ticks;
            if ticks % TICKS_PER_FRAME == 0 {
                trigger_screen_frame(ticks);
            }
            if ticks % TICKS_PER_SECOND == 0 {
                let mut time = TIME.lock();
                *time += 1;
            }
        }
        None => {
            *prev_ticks = 0;
        }
    }
}

pub fn get_ms() -> u64 {
    return (((*TICKS.lock() % TICKS_PER_SECOND) as f64 / TICKS_PER_SECOND_F) * 1000.0) as u64;
}

pub fn get_total_ms() -> u64 {
    return ((*TICKS.lock() as f64 / TICKS_PER_SECOND_F) * 1000.0) as u64;
}
