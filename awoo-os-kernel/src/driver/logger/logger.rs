use crate::{
    driver::timer::timer::{get_ms, TIME},
    serial_println,
};
use alloc::{collections::VecDeque, format, string::String};
use lazy_static::lazy_static;
use spin::{Mutex, MutexGuard};

lazy_static! {
    pub static ref LOGGER_LINES: Mutex<VecDeque<String>> = Mutex::new(VecDeque::new());
}

pub fn init_logger() {
    (*LOGGER_LINES.lock()).push_back(String::new());
}

pub fn println(s: &str) {
    serial_println!("{}", s);
    print(&format!("[{:04}:{:03}] {}\n", *TIME.lock(), get_ms(), s,));
}

pub fn print(s: &str) {
    let mut lines = LOGGER_LINES.lock();
    let mut line = lines.pop_back().unwrap();
    for c in s.chars() {
        match c {
            '\n' => {
                push_line(&mut lines, line);
                line = String::new();
            }
            _ => {
                line.push(c);
            }
        }
    }
    push_line(&mut lines, line);
}

fn push_line(lines: &mut MutexGuard<VecDeque<String>>, line: String) {
    lines.push_back(line);
    if lines.len() > 32 {
        lines.pop_front();
    }
}
