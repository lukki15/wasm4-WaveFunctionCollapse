#[cfg(test)]
mod tests;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bitmap;
mod constants;
mod generator;
mod matcher;
mod palette;
mod pattern;
mod wasm4;

use constants::*;
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    static ref GENERATOR: Mutex<generator::Generator> = Mutex::new(generator::Generator::new());
}

#[no_mangle]
fn start() {
    palette::set_palette([0xbfe8f2, 0xb97a57, 0xfff200, 0x00aa00]);
    palette::set_draw_colour(0x4203);

    GENERATOR.lock().expect("generator").init_borders();
}

#[no_mangle]
fn update() {
    wasm4::blit(&BITMAP, 76, 76, BITMAP_WIDTH, BITMAP_HEIGHT, BITMAP_FLAGS);
}
