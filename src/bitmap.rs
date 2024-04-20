pub const BITMAP_WIDTH: usize = 16;
pub const BITMAP_HEIGHT: usize = 24;
pub const BITMAP_FLAGS: u32 = 1; // BLIT_2BPP
pub const BITMAP: [u8; BITMAP_WIDTH * BITMAP_HEIGHT / 4] = [
    0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x45, 0x55, 0x45, 0x55, 0x31, 0x55, 0x31,
    0x55, 0x45, 0x55, 0x45, 0x55, 0x75, 0x55, 0x75, 0x55, 0x7d, 0x55, 0xf5, 0x45, 0x5f, 0x57, 0xd5,
    0x31, 0x57, 0x57, 0x55, 0x45, 0x5f, 0x57, 0xd5, 0x75, 0x5d, 0x45, 0xf5, 0x7d, 0x7d, 0x31, 0x75,
    0x5d, 0xf5, 0x45, 0x75, 0x5f, 0xd5, 0x75, 0xf5, 0x57, 0x55, 0x77, 0xd5, 0x57, 0xd5, 0x7f, 0x55,
    0x55, 0xf5, 0x5d, 0x55, 0x55, 0x7d, 0x7d, 0x55, 0x55, 0x5d, 0xf5, 0x55, 0x55, 0x5f, 0xd5, 0x55,
    0x55, 0x57, 0x55, 0x55, 0x55, 0x57, 0x55, 0x55, 0xaa, 0xab, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
];

const BITMAP_BIT_PER_BIXEL: usize = if BITMAP_FLAGS == 0 { 1 } else { 2 };
const BITMAP_PIXELS_PER_BYTE: usize = 8 / BITMAP_BIT_PER_BIXEL;

pub const PATTERN_N: usize = 2;
pub const PATTERN_X: usize = BITMAP_WIDTH / PATTERN_N;
pub const PATTERN_Y: usize = BITMAP_HEIGHT / PATTERN_N;

fn get_pixels(x: usize, y: usize) -> u8 {
    std::assert!(x < BITMAP_WIDTH, "x = {}", x);
    std::assert!(y < BITMAP_HEIGHT, "y = {}", y);

    let pixel = y * BITMAP_WIDTH + x;
    let byte_index = pixel / BITMAP_PIXELS_PER_BYTE;
    let byte_offset = pixel % BITMAP_PIXELS_PER_BYTE;

    BITMAP[byte_index] << (BITMAP_BIT_PER_BIXEL * byte_offset)
}

const PATTERN_ARRAY_SIZE: usize =
    (PATTERN_N * PATTERN_N + (BITMAP_PIXELS_PER_BYTE - 1)) / BITMAP_PIXELS_PER_BYTE;
type PatternArray = [u8; PATTERN_ARRAY_SIZE];

pub fn _get_pattern2(x: usize, y: usize) -> PatternArray {
    std::assert!(x < PATTERN_X);
    std::assert!(y < PATTERN_Y);
    let mut array: PatternArray = [0; PATTERN_ARRAY_SIZE];

    let bitmap_x = x * PATTERN_N;
    let bitmap_y = y * PATTERN_N;

    for array_y in 0..PATTERN_N {
        for array_x in (0..PATTERN_N).step_by(BITMAP_PIXELS_PER_BYTE) {
            let mask_pixel = if PATTERN_N > BITMAP_PIXELS_PER_BYTE {
                0xFF
            } else {
                const SHIFT: usize = BITMAP_BIT_PER_BIXEL * (BITMAP_PIXELS_PER_BYTE - PATTERN_N);
                0xFF << SHIFT
            };

            let pixels_value = get_pixels(bitmap_x + array_x, bitmap_y + array_y);
            let pixels = pixels_value & mask_pixel;

            let array_index = array_y * PATTERN_N + array_x;
            let byte_index = array_index / BITMAP_PIXELS_PER_BYTE;

            if PATTERN_N == 2 && array_y % 2 == 1 {
                array[byte_index] |= pixels >> BITMAP_PIXELS_PER_BYTE;
            } else {
                array[byte_index] = pixels;
            }
        }
    }

    array
}

pub fn get_pattern(x: usize, y: usize) -> PatternArray {
    std::assert!(x < PATTERN_X);
    std::assert!(y < PATTERN_Y);
    let mut array: PatternArray = [0; PATTERN_ARRAY_SIZE];

    let bitmap_x = x * PATTERN_N;
    let bitmap_y = y * PATTERN_N;

    for array_y in 0..PATTERN_N {
        for array_x in (0..PATTERN_N).step_by(4) {
            let mask_pixel = if PATTERN_N > 4 {
                0xFF
            } else {
                0xFF << (2 * (4 - PATTERN_N))
            };

            let pixels_value = get_pixels(bitmap_x + array_x, bitmap_y + array_y);
            let pixels = pixels_value & mask_pixel;

            let array_index = array_y * PATTERN_N + array_x;
            let byte_index = array_index / 4;

            if PATTERN_N == 2 && array_y % 2 == 1 {
                array[byte_index] |= pixels >> 4;
            } else {
                array[byte_index] = pixels;
            }
        }
    }

    array
}
