use super::{mouse::MOUSE_POSITION, queue::MouseInputStream};
use core::cmp::min;
use futures_util::StreamExt;

pub async fn mouse_task() {
    let mut inputs = MouseInputStream::new();
    while let Some(input) = inputs.next().await {
        let x_delta_negative = input.flags & 0b00010000 == 0b00010000;
        let y_delta_negative = input.flags & 0b00100000 == 0b00100000;

        let mut pos = MOUSE_POSITION.lock();
        if x_delta_negative {
            match pos.x.checked_sub(input.x_delta as u16) {
                Some(ncx) => pos.x = ncx,
                None => {}
            }
        } else {
            pos.x = min(pos.x + input.x_delta as u16, 800);
        }
        if y_delta_negative {
            match pos.y.checked_sub(input.y_delta as u16) {
                Some(ncy) => pos.y = ncy,
                None => {}
            }
        } else {
            pos.y = min(pos.y + input.y_delta as u16, 800);
        }
    }
}
