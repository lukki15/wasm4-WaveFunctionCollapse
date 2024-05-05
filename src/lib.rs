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
    // static ref PATTERNS: Mutex<Vec<pattern::Pattern>> = Mutex::new(matcher::generate_patterns());

}

#[no_mangle]
fn start() {
    palette::set_palette([0xbfe8f2, 0xb97a57, 0xfff200, 0x00aa00]);
    palette::set_draw_colour(0x4203);

    GENERATOR.lock().expect("generator").init_borders();
}

fn draw() {
    let generator = &GENERATOR.lock().unwrap();
    for x in 0..generator::PATTERN_SIZE {
        for y in 0..generator::PATTERN_SIZE {
            if generator.grid[x][y].len() == 1 {
                for pattern_index in 0..generator.patterns.len() {
                    if generator.grid[x][y].contains(pattern_index) {
                        let blit_x = x * PATTERN_N;
                        let blit_y = y * PATTERN_N;
                        wasm4::blit(
                            generator.patterns[pattern_index].get_array(),
                            blit_x,
                            blit_y,
                            PATTERN_N,
                            PATTERN_N,
                            BITMAP_FLAGS,
                        );
                        break;
                    }
                }
            }
        }
    }

    let patterns = &generator.patterns;
    for pattern_index in 0..patterns.len() {
        let x = pattern_index % 2;
        let y = pattern_index / 2;
        let blit_x = x * (PATTERN_N + 1);
        let blit_y = y * (PATTERN_N + 1);
        wasm4::blit(
            patterns[pattern_index].get_array(),
            100 + blit_x,
            100 + blit_y,
            PATTERN_N,
            PATTERN_N,
            BITMAP_FLAGS,
        );
    }
}

#[no_mangle]
fn update() {
    wasm4::blit(&BITMAP, 76, 76, BITMAP_WIDTH, BITMAP_HEIGHT, BITMAP_FLAGS);
    draw();
}
