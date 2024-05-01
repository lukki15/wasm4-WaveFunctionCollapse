use crate::constants::*;
use crate::pattern::*;

#[test]
fn new() {
    let array: PatternArray = [0; PATTERN_ARRAY_SIZE];
    let _pattern = Pattern::new(array);
}

#[test]
fn find() {
    let array_zero: PatternArray = [0; PATTERN_ARRAY_SIZE];
    let array_one: PatternArray = [1; PATTERN_ARRAY_SIZE];
    let pattern_zero = Pattern::new(array_zero);

    let mut patterns: Vec<Pattern> = Vec::new();
    patterns.push(pattern_zero);

    assert!(Pattern::find(&patterns, array_zero).is_some());
    assert!(Pattern::find(&patterns, array_one).is_none());
}
