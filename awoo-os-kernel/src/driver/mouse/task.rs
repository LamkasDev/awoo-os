use super::{
    mouse::{MousePosition, MOUSE_POSITION},
    queue::MouseInputStream,
};
use core::cmp::min;
use futures_util::StreamExt;

pub async fn mouse_task() {
    let mut inputs = MouseInputStream::new();
    while let Some(input) = inputs.next().await {
        let x_delta_negative = input.flags & 0b00010000 == 0b00010000;
        let y_delta_negative = input.flags & 0b00100000 == 0b00100000;

        let mut prev_pos = MOUSE_POSITION.lock();
        let mut pos = MousePosition { x: 0, y: 0 };
        if x_delta_negative {
            match prev_pos.x.checked_sub(input.x_delta as u16) {
                Some(ncx) => pos.x = ncx,
                None => {}
            }
        } else {
            pos.x = min(prev_pos.x + input.x_delta as u16, 800);
        }
        if y_delta_negative {
            match prev_pos.y.checked_sub(input.y_delta as u16) {
                Some(ncy) => pos.y = ncy,
                None => {}
            }
        } else {
            pos.y = min(prev_pos.y + input.y_delta as u16, 800);
        }
        *prev_pos = pos;
        drop(prev_pos);
    }
}
