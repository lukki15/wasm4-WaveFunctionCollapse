use crate::constants::*;

fn get_pixel(x: usize, y: usize) -> u8 {
    std::assert!(x < BITMAP_WIDTH, "x = {}", x);
    std::assert!(y < BITMAP_HEIGHT, "y = {}", y);

    let pixel = y * BITMAP_WIDTH + x;
    let byte_index = pixel / BITMAP_PIXELS_PER_BYTE;
    let byte_offset = pixel % BITMAP_PIXELS_PER_BYTE;

    const MASK: u8 = if BITMAP_BIT_PER_PIXEL == 1 {
        0b10000000
    } else {
        0b11000000
    };

    let shifted_pixel = BITMAP[byte_index] << (BITMAP_BIT_PER_PIXEL * byte_offset);
    shifted_pixel & MASK
}

pub fn get_pattern(x: usize, y: usize) -> PatternArray {
    std::assert!(x < PATTERN_X);
    std::assert!(y < PATTERN_Y);
    let mut array: PatternArray = [0; PATTERN_ARRAY_SIZE];

    let bitmap_x = x * PATTERN_N;
    let bitmap_y = y * PATTERN_N;

    for array_y in 0..PATTERN_N {
        for array_x in 0..PATTERN_N {
            let index = array_y * PATTERN_N + array_x;
            let byte_index = index / BITMAP_PIXELS_PER_BYTE;
            let byte_offset = index % BITMAP_PIXELS_PER_BYTE;

            let pixel = get_pixel(bitmap_x + array_x, bitmap_y + array_y);
            array[byte_index] |= pixel >> (BITMAP_BIT_PER_PIXEL * byte_offset);
        }
    }

    array
}
