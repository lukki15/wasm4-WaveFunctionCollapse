use crate::constants::PatternArray;
use std::collections::BTreeSet;

struct Neighbor {
    list: BTreeSet<usize>,
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
    pub fn new(array: PatternArray) -> Self {
        Pattern {
            array: array,
            top: Neighbor {
                list: BTreeSet::new(),
                border: false,
            },
            right: Neighbor {
                list: BTreeSet::new(),
                border: false,
            },
            bottom: Neighbor {
                list: BTreeSet::new(),
                border: false,
            },
            left: Neighbor {
                list: BTreeSet::new(),
                border: false,
            },
        }
    }

    pub fn set_top_border(&mut self) {
        self.top.border = true;
    }
    pub fn set_right_border(&mut self) {
        self.right.border = true;
    }
    pub fn set_bottom_border(&mut self) {
        self.bottom.border = true;
    }
    pub fn set_left_border(&mut self) {
        self.left.border = true;
    }

    pub fn get_top_border(&self) -> bool {
        self.top.border
    }
    pub fn get_right_border(&self) -> bool {
        self.right.border
    }
    pub fn get_bottom_border(&self) -> bool {
        self.bottom.border
    }
    pub fn get_left_border(&self) -> bool {
        self.left.border
    }

    pub fn add_top_neighbor(&mut self, index: usize) {
        self.top.list.insert(index);
    }
    pub fn add_right_neighbor(&mut self, index: usize) {
        self.right.list.insert(index);
    }
    pub fn add_bottom_neighbor(&mut self, index: usize) {
        self.bottom.list.insert(index);
    }
    pub fn add_left_neighbor(&mut self, index: usize) {
        self.left.list.insert(index);
    }

    pub fn get_top_neighbors(&self) -> &BTreeSet<usize> {
        &self.top.list
    }
    pub fn get_right_neighbors(&self) -> &BTreeSet<usize> {
        &self.right.list
    }
    pub fn get_bottom_neighbors(&self) -> &BTreeSet<usize> {
        &self.bottom.list
    }
    pub fn get_left_neighbors(&self) -> &BTreeSet<usize> {
        &self.left.list
    }

    pub fn find(patterns: &Vec<Self>, pattern_array: PatternArray) -> Option<usize> {
        patterns
            .iter()
            .position(|pattern| pattern.array == pattern_array)
    }

    pub fn get_array(&self) -> &PatternArray {
        return &self.array;
    }
}
