use crate::driver::shell::queue::println;

use super::queue::ScancodeStream;
use alloc::{format, string::ToString};
use futures_util::StreamExt;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};

pub async fn scancode_task() {
    let mut scancodes = ScancodeStream::new();
    let mut keyboard = Keyboard::new(
        ScancodeSet1::new(),
        layouts::De105Key,
        HandleControl::Ignore,
    );

    while let Some(scancode) = scancodes.next().await {
        if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
            if let Some(key) = keyboard.process_keyevent(key_event) {
                match key {
                    DecodedKey::Unicode(character) => println(&character.to_string()),
                    DecodedKey::RawKey(key) => println(&format!("{:?}", key)),
                }
            }
        }
    }
}
