use crate::bitmap;
use crate::constants::*;
use crate::matcher::*;
use crate::pattern;

#[test]
fn all_are_some() {
    let patterns = generate_patterns();

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let array = bitmap::get_pattern(x, y);
            let index_option = pattern::Pattern::find(&patterns, array);

            assert!(index_option.is_some());
        }
    }
}

fn get_pattern_index(patterns: &Vec<pattern::Pattern>, x: usize, y: usize) -> usize {
    let array = bitmap::get_pattern(x, y);
    pattern::Pattern::find(&patterns, array).unwrap()
}

fn get_pattern(patterns: &Vec<pattern::Pattern>, x: usize, y: usize) -> &pattern::Pattern {
    let index_option = get_pattern_index(patterns, x, y);
    &patterns[index_option]
}

#[test]
fn borders() {
    let patterns = generate_patterns();

    for x in 0..PATTERN_X {
        {
            let y = 0;
            let pattern = get_pattern(&patterns, x, y);
            assert!(pattern.get_top_border());
        }
        {
            let y = PATTERN_Y - 1;
            let pattern = get_pattern(&patterns, x, y);
            assert!(pattern.get_bottom_border());
        }
    }

    for y in 0..PATTERN_Y {
        {
            let x = 0;
            let pattern = get_pattern(&patterns, x, y);
            assert!(pattern.get_left_border());
        }
        {
            let x = PATTERN_X - 1;
            let pattern = get_pattern(&patterns, x, y);
            assert!(pattern.get_right_border());
        }
    }
}

#[test]
fn neighbors() {
    let patterns = generate_patterns();

    for x in 0..PATTERN_X {
        for y in 0..PATTERN_Y {
            let pattern = get_pattern(&patterns, x, y);

            if y > 0 {
                let pattern_top_index = get_pattern_index(&patterns, x, y - 1);
                assert!(pattern.get_top_neighbors().contains(&pattern_top_index));
            }

            if x < PATTERN_X - 1 {
                let pattern_right_index = get_pattern_index(&patterns, x + 1, y);
                assert!(pattern.get_right_neighbors().contains(&pattern_right_index));
            }

            if y < PATTERN_Y - 1 {
                let pattern_bottom_index = get_pattern_index(&patterns, x, y + 1);
                assert!(pattern
                    .get_bottom_neighbors()
                    .contains(&pattern_bottom_index));
            }

            if x > 0 {
                let pattern_left_index = get_pattern_index(&patterns, x - 1, y);
                assert!(pattern.get_left_neighbors().contains(&pattern_left_index));
            }
        }
    }
}
