#[cfg(test)]
mod tests;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bitmap;
mod constants;
mod palette;
mod pattern;
mod wasm4;

use constants::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref PATTERNS: Mutex<Vec<pattern::Pattern>> = Mutex::new(Vec::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0xbfe8f2, 0xb97a57, 0xfff200, 0x00aa00]);
    palette::set_draw_colour(0x4203);

    let mut pattern_indexes = [[0; PATTERN_Y]; PATTERN_X];

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let array = bitmap::get_pattern(x, y);
            let index_option = pattern::Pattern::find(&PATTERNS.lock().expect("patterns"), array);

            match index_option {
                Some(index) => {pattern_indexes[x][y] = index;}
                None => {
                    PATTERNS.lock().expect("patterns").push(pattern::Pattern::new(array));
                    pattern_indexes[x][y] = PATTERNS.lock().expect("patterns").len() - 1;
                }
            }

            // let is_top = y == 0;
            // let is_right = x == PATTERN_X -1;
            // let is_bottom = y == PATTERN_Y -1;
            // let is_left = x == 0;
            
        }
    }
}

#[no_mangle]
fn update() {
    wasm4::blit(&BITMAP, 76, 76, BITMAP_WIDTH, BITMAP_HEIGHT, BITMAP_FLAGS);
}
