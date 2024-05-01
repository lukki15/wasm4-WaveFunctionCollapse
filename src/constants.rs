pub const BITMAP_WIDTH: usize = 15;
pub const BITMAP_HEIGHT: usize = 24;
pub const BITMAP_FLAGS: u32 = 1; // BLIT_2BPP

pub const BITMAP_BIT_PER_PIXEL: usize = if BITMAP_FLAGS == 0 { 1 } else { 2 };
pub const BITMAP_PIXELS_PER_BYTE: usize = 8 / BITMAP_BIT_PER_PIXEL;

const BITMAP_SIZE: usize =
    (BITMAP_WIDTH * BITMAP_HEIGHT + BITMAP_PIXELS_PER_BYTE - 1) / BITMAP_PIXELS_PER_BYTE;
pub const BITMAP: [u8; BITMAP_SIZE] = [
    0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x55, 0x54, 0x55, 0x54, 0x55, 0x4c, 0x55, 0x4c, 0x55,
    0x45, 0x55, 0x45, 0x55, 0xd5, 0x55, 0xd5, 0x57, 0xd5, 0x5f, 0x51, 0x57, 0xd5, 0xf5, 0x31, 0x57,
    0x57, 0x55, 0x15, 0x7d, 0x5f, 0x57, 0x55, 0xd4, 0x5f, 0x5f, 0x5f, 0x4c, 0x5d, 0x5d, 0xf5, 0x45,
    0x75, 0x7f, 0x55, 0xd7, 0xd5, 0x75, 0x57, 0x7d, 0x55, 0xf5, 0x5f, 0xd5, 0x55, 0xf5, 0x5d, 0x55,
    0x55, 0xf5, 0xf5, 0x55, 0x55, 0xdf, 0x55, 0x55, 0x57, 0xf5, 0x55, 0x55, 0x57, 0x55, 0x55, 0x55,
    0x5d, 0x55, 0x5a, 0xaa, 0xba, 0xaa, 0xaa, 0xaa, 0xaa, 0xaa,
];

pub const PATTERN_N: usize = 3;
pub const PATTERN_X: usize = BITMAP_WIDTH / PATTERN_N;
pub const PATTERN_Y: usize = BITMAP_HEIGHT / PATTERN_N;

pub const PATTERN_ARRAY_SIZE: usize =
    (PATTERN_N * PATTERN_N + (BITMAP_PIXELS_PER_BYTE - 1)) / BITMAP_PIXELS_PER_BYTE;
pub type PatternArray = [u8; PATTERN_ARRAY_SIZE];
