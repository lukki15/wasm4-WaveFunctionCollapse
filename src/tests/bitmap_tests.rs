use crate::bitmap::*;
use crate::constants::*;

#[test]
fn first_pixel() {
    const BITMAP_MASK: u8 = 0b1100_0000;
    let first_section = get_pattern(0, 0);
    let first_section_pixel = first_section[0] & BITMAP_MASK;

    let first_bitmap_pixel = BITMAP[0] & BITMAP_MASK;

    assert_eq!(first_section_pixel, first_bitmap_pixel);
}

fn get_bitmap_pixel(x: usize, y: usize) -> u8 {
    let bitmap_index = y * BITMAP_WIDTH + x;
    let byte_index = bitmap_index / BITMAP_PIXELS_PER_BYTE;
    let byte_offset = BITMAP_PIXELS_PER_BYTE - 1 - (bitmap_index % BITMAP_PIXELS_PER_BYTE);

    BITMAP[byte_index] >> (BITMAP_BIT_PER_PIXEL * byte_offset) & 0x03
}

fn get_pattern_pixel(x: usize, y: usize) -> u8 {
    let pattern_y = y / PATTERN_N;
    let pattern_y_offset = y % PATTERN_N;

    let pattern_x = x / PATTERN_N;
    let pattern_x_offset = x % PATTERN_N;

    let pattern_pixels = get_pattern(pattern_x, pattern_y);
    let pattern_index = pattern_y_offset * PATTERN_N + pattern_x_offset;
    let byte_index = pattern_index / BITMAP_PIXELS_PER_BYTE;
    let byte_offset = BITMAP_PIXELS_PER_BYTE - 1 - (pattern_index % BITMAP_PIXELS_PER_BYTE);

    pattern_pixels[byte_index] >> (BITMAP_BIT_PER_PIXEL * byte_offset) & 0x03
}

#[test]
fn pixel_by_piyel() {
    for y in 0..BITMAP_HEIGHT {
        for x in 0..BITMAP_WIDTH {
            let bitmap_pixel = get_bitmap_pixel(x, y);
            let pattern_pixel = get_pattern_pixel(x, y);
            assert_eq!(pattern_pixel, bitmap_pixel);
        }
    }
}
