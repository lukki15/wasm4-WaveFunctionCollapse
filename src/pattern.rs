use crate::constants::PatternArray;

struct Neighbor {
    list : Vec<usize>,
    border: bool,
}

pub struct Pattern {
    array: PatternArray,
    top: Neighbor,
    right: Neighbor,
    bottom: Neighbor,
    left: Neighbor,
}

impl Pattern {
    pub fn new(
        array: PatternArray
    ) -> Self {
        Pattern {
            array: array,
            top: Neighbor{list:Vec::new(), border: false},
            right: Neighbor{list:Vec::new(), border: false},
            bottom: Neighbor{list:Vec::new(), border: false},
            left: Neighbor{list:Vec::new(), border: false},
        }
    }

    pub fn find(patterns: &Vec<Self>, pattern_array: PatternArray) -> Option<usize> {
        patterns.iter().position(|pattern| pattern.array == pattern_array)
    }

}
