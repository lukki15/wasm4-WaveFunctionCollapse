#[cfg(test)]
mod tests;

#[cfg(feature = "buddy-alloc")]
mod alloc;
mod bitmap;
mod constants;
mod palette;
mod wasm4;

use constants::*;

#[no_mangle]
fn start() {
    palette::set_palette([0xbfe8f2, 0xb97a57, 0xfff200, 0x00aa00]);
    palette::set_draw_colour(0x4203);
}

#[no_mangle]
fn update() {
    wasm4::blit(&BITMAP, 76, 76, BITMAP_WIDTH, BITMAP_HEIGHT, BITMAP_FLAGS);

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let array = bitmap::get_pattern(x, y);
            wasm4::blit(
                &array,
                (x * 2) * PATTERN_N,
                (y * 2) * PATTERN_N,
                PATTERN_N,
                PATTERN_N,
                BITMAP_FLAGS,
            );
        }
    }
}
