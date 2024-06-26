use crate::wasm4;

pub fn set_palette(palette: [u32; 4]) {
    unsafe {
        *wasm4::PALETTE = palette;
    }
}

pub fn set_draw_colour(idx: u16) {
    unsafe { *wasm4::DRAW_COLORS = idx.into() }
}
