pub const BITMAP_WIDTH: usize = 15;
pub const BITMAP_HEIGHT: usize = 24;
pub const BITMAP_FLAGS: u32 = 1; // BLIT_2BPP
pub const BITMAP: [u8; BITMAP_WIDTH * BITMAP_HEIGHT / 4] = [
    0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x54, 0x55, 0x54, 0x55, 0x4c, 0x55, 0x4c, 0x55,
    0x45, 0x55, 0x45, 0x55, 0xd5, 0x55, 0xd5, 0x57, 0xd5, 0x5f, 0x51, 0x57, 0xd5, 0xf5, 0x31, 0x57,
    0x57, 0x55, 0x15, 0x7d, 0x5f, 0x57, 0x55, 0xd4, 0x5f, 0x5f, 0x5f, 0x4c, 0x5d, 0x5d, 0xf5, 0x45,
    0x75, 0x7f, 0x55, 0xd7, 0xd5, 0x75, 0x57, 0x7d, 0x55, 0xf5, 0x5f, 0xd5, 0x55, 0xf5, 0x5d, 0x55,
    0x55, 0xf5, 0xf5, 0x55, 0x55, 0xdf, 0x55, 0x55, 0x57, 0xf5, 0x55, 0x55, 0x57, 0x55, 0x55, 0x55,
    0x5d, 0x55, 0x5a, 0xaa, 0xba, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
];

pub const BITMAP_BIT_PER_PIXEL: usize = if BITMAP_FLAGS == 0 { 1 } else { 2 };
pub const BITMAP_PIXELS_PER_BYTE: usize = 8 / BITMAP_BIT_PER_PIXEL;

pub const PATTERN_N: usize = 3;
pub const PATTERN_X: usize = BITMAP_WIDTH / PATTERN_N;
pub const PATTERN_Y: usize = BITMAP_HEIGHT / PATTERN_N;

fn get_pixel(x: usize, y: usize) -> u8 {
    std::assert!(x < BITMAP_WIDTH, "x = {}", x);
    std::assert!(y < BITMAP_HEIGHT, "y = {}", y);

    let pixel = y * BITMAP_WIDTH + x;
    let byte_index = pixel / BITMAP_PIXELS_PER_BYTE;
    let byte_offset = pixel % BITMAP_PIXELS_PER_BYTE;

    const MASK: u8 = if BITMAP_BIT_PER_PIXEL == 1 {0b10000000} else {0b11000000};

    let shifted_pixel = BITMAP[byte_index] << (BITMAP_BIT_PER_PIXEL * byte_offset);
    shifted_pixel & MASK
}

const PATTERN_ARRAY_SIZE: usize =
    (PATTERN_N * PATTERN_N + (BITMAP_PIXELS_PER_BYTE - 1)) / BITMAP_PIXELS_PER_BYTE;
type PatternArray = [u8; PATTERN_ARRAY_SIZE];

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

            let pixel = get_pixel(bitmap_x+array_x, bitmap_y+array_y);
            array[byte_index] |= pixel >> (BITMAP_BIT_PER_PIXEL*byte_offset);
        }
    }

    array
}
