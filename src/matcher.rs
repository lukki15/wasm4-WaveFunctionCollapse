use crate::bitmap;
use crate::constants::*;
use crate::pattern;

pub fn generate_patterns() -> Vec<pattern::Pattern> {
    let mut patterns: Vec<pattern::Pattern> = Vec::new();
    let mut pattern_indexes = [[0; PATTERN_Y]; PATTERN_X];

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let array = bitmap::get_pattern(x, y);
            let index_option = pattern::Pattern::find(&patterns, array);

            match index_option {
                Some(index) => {
                    pattern_indexes[x][y] = index;
                }
                None => {
                    patterns.push(pattern::Pattern::new(array));
                    pattern_indexes[x][y] = patterns.len() - 1;
                }
            }
        }
    }

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let current_index = pattern_indexes[x][y];
            let current_pattern = &mut patterns[current_index];

            if y == 0 {
                current_pattern.set_top_border();
            } else {
                current_pattern.add_top_neighbor(pattern_indexes[x][y - 1]);
            }

            if x == PATTERN_X - 1 {
                current_pattern.set_right_border();
            } else {
                current_pattern.add_right_neighbor(pattern_indexes[x + 1][y]);
            }

            if y == PATTERN_Y - 1 {
                current_pattern.set_bottom_border();
            } else {
                current_pattern.add_bottom_neighbor(pattern_indexes[x][y + 1]);
            }

            if x == 0 {
                current_pattern.set_left_border();
            } else {
                current_pattern.add_left_neighbor(pattern_indexes[x - 1][y]);
            }
        }
    }

    patterns
}
