use crate::bitmap::*;

#[test]
fn first_pixel() {
    const BITMAP_MASK: u8 = 0b1100_0000;
    let first_section = get_pattern(0, 0);
    let first_section_pixel = first_section[0] & BITMAP_MASK;

    let first_bitmap_pixel = BITMAP[0] & BITMAP_MASK;

    assert_eq!(first_section_pixel, first_bitmap_pixel);
}

#[test]
fn first_pixels() {
    const BITMAP_MASK: u8 = if PATTERN_N >= 4 {
        0xFF
    } else {
        0xFF << (2 * (4 - PATTERN_N))
    };

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let first_section = get_pattern(x, y);
            let first_section_byte_pixel = first_section[0] & BITMAP_MASK;

            let bitmap_pixel = (y * PATTERN_N) * BITMAP_WIDTH + x * PATTERN_N;
            let bitmap_index = bitmap_pixel / 4;
            let bitmap_offset = bitmap_pixel % 4;
            let first_bitmap_byte_pixel =
                (BITMAP[bitmap_index] << (2 * bitmap_offset)) & BITMAP_MASK;

            assert_eq!(first_section_byte_pixel, first_bitmap_byte_pixel);
        }
    }
}

#[test]
fn first_pixels() {
    const BITMAP_MASK: u8 = if PATTERN_N >= 4 {
        0xFF
    } else {
        0xFF << (2 * (4 - PATTERN_N))
    };

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let first_section = get_pattern(x, y);
            let first_section_byte_pixel = first_section[0] & BITMAP_MASK;

            let bitmap_pixel = (y * PATTERN_N) * BITMAP_WIDTH + x * PATTERN_N;
            let bitmap_index = bitmap_pixel / 4;
            let bitmap_offset = bitmap_pixel % 4;
            let first_bitmap_byte_pixel =
                (BITMAP[bitmap_index] << (2 * bitmap_offset)) & BITMAP_MASK;

            assert_eq!(first_section_byte_pixel, first_bitmap_byte_pixel);
        }
    }
}
