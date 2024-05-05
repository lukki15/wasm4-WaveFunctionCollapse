use crate::matcher;
use crate::pattern;

use bit_set::BitSet;

//pub const PATTERN_SIZE: usize = wasm4::SCREEN_SIZE / PATTERN_N;
pub const PATTERN_SIZE: usize = 6;

pub struct Generator {
    pub patterns: Vec<pattern::Pattern>,
    pub grid: [[BitSet; PATTERN_SIZE]; PATTERN_SIZE],
}

impl Generator {
    pub fn new() -> Self {
        let patterns = matcher::generate_patterns();
        let grid: [[BitSet; PATTERN_SIZE]; PATTERN_SIZE] =
            std::array::from_fn(|_| std::array::from_fn(|_| (0..patterns.len()).collect()));

        Generator { patterns, grid }
    }

    pub fn init_borders(&mut self) {
        for pattern_index in 0..self.patterns.len() {
            let pattern = &self.patterns[pattern_index];
            for x in 1..PATTERN_SIZE - 1 {
                {
                    // top
                    const Y: usize = 0;
                    if !pattern.get_top_border() {
                        self.grid[x][Y].remove(pattern_index);
                    }
                }
                {
                    // bottom
                    const Y: usize = PATTERN_SIZE - 1;
                    if !pattern.get_bottom_border() {
                        self.grid[x][Y].remove(pattern_index);
                    }
                }
            }
            for y in 1..PATTERN_SIZE - 1 {
                {
                    // left
                    const X: usize = 0;
                    if !pattern.get_left_border() {
                        self.grid[X][y].remove(pattern_index);
                    }
                }
                {
                    // right
                    const X: usize = PATTERN_SIZE - 1;
                    if !pattern.get_right_border() {
                        self.grid[X][y].remove(pattern_index);
                    }
                }
            }

            if !pattern.get_top_border() || !pattern.get_left_border() {
                self.grid[0][0].remove(pattern_index);
            }
            if !pattern.get_top_border() || !pattern.get_right_border() {
                self.grid[PATTERN_SIZE - 1][0].remove(pattern_index);
            }
            if !pattern.get_bottom_border() || !pattern.get_left_border() {
                self.grid[0][PATTERN_SIZE - 1].remove(pattern_index);
            }
            if !pattern.get_bottom_border() || !pattern.get_right_border() {
                self.grid[PATTERN_SIZE - 1][PATTERN_SIZE - 1].remove(pattern_index);
            }
        }
    }
}
