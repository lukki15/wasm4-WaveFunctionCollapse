use crate::constants::*;
use crate::matcher;
use crate::pattern;
use crate::wasm4;
use std::collections::BTreeSet;

pub const PATTERN_SIZE: usize = wasm4::SCREEN_SIZE / PATTERN_N;

pub struct Generator {
    pub patterns: Vec<pattern::Pattern>,
    pub grid: [[BTreeSet<usize>; PATTERN_SIZE]; PATTERN_SIZE],
}

impl Generator {
    pub fn new() -> Self {
        let patterns = matcher::generate_patterns();

        const ARRAY_REPEAT_VALUE: BTreeSet<usize> = BTreeSet::new();
        const ROW_REPEAT_VALUE: [BTreeSet<usize>; PATTERN_SIZE] =
            [ARRAY_REPEAT_VALUE; PATTERN_SIZE];
        let mut grid = [ROW_REPEAT_VALUE; PATTERN_SIZE];

        for x in 0..PATTERN_SIZE {
            for y in 0..PATTERN_SIZE {
                grid[x][y] = (0..patterns.len()).collect();
            }
        }

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
                        self.grid[x][Y].remove(&pattern_index);
                    }
                }
                {
                    // bottom
                    const Y: usize = PATTERN_SIZE - 1;
                    if !pattern.get_bottom_border() {
                        self.grid[x][Y].remove(&pattern_index);
                    }
                }
            }
            for y in 1..PATTERN_SIZE - 1 {
                {
                    // left
                    const X: usize = 0;
                    if !pattern.get_left_border() {
                        self.grid[X][y].remove(&pattern_index);
                    }
                }
                {
                    // right
                    const X: usize = PATTERN_SIZE - 1;
                    if !pattern.get_right_border() {
                        self.grid[X][y].remove(&pattern_index);
                    }
                }
            }

            if !pattern.get_top_border() || !pattern.get_left_border() {
                self.grid[0][0].remove(&pattern_index);
            }
            if !pattern.get_top_border() || !pattern.get_right_border() {
                self.grid[0][PATTERN_SIZE - 1].remove(&pattern_index);
            }
            if !pattern.get_bottom_border() || !pattern.get_left_border() {
                self.grid[PATTERN_SIZE - 1][0].remove(&pattern_index);
            }
            if !pattern.get_bottom_border() || !pattern.get_right_border() {
                self.grid[PATTERN_SIZE - 1][PATTERN_SIZE - 1].remove(&pattern_index);
            }
        }
    }
}
