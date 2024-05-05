use crate::generator;

#[test]
fn all_patterns() {
    let generator = generator::Generator::new();

    for x in 0..generator::PATTERN_SIZE {
        for y in 0..generator::PATTERN_SIZE {
            assert_eq!(generator.grid[x][y].len(), generator.patterns.len());
            for i in 0..generator.patterns.len() {
                assert!(generator.grid[x][y].contains(&i));
            }
        }
    }
}

#[test]
fn borders() {
    let mut generator = generator::Generator::new();
    generator.init_borders();

    for x in 1..generator::PATTERN_SIZE - 1 {
        for y in 1..generator::PATTERN_SIZE - 1 {
            assert_eq!(generator.grid[x][y].len(), generator.patterns.len());
            for i in 0..generator.patterns.len() {
                assert!(generator.grid[x][y].contains(&i));
            }
        }
    }

    for i in 0..generator.patterns.len() {
        for x in 1..generator::PATTERN_SIZE - 1 {
            {
                const Y: usize = 0;
                assert_eq!(
                    generator.grid[x][Y].contains(&i),
                    generator.patterns[i].get_top_border()
                );
            }
            {
                const Y: usize = generator::PATTERN_SIZE - 1;
                assert_eq!(
                    generator.grid[x][Y].contains(&i),
                    generator.patterns[i].get_bottom_border()
                );
            }
        }
        for y in 1..generator::PATTERN_SIZE - 1 {
            {
                const X: usize = 0;
                assert_eq!(
                    generator.grid[X][y].contains(&i),
                    generator.patterns[i].get_left_border()
                );
            }
            {
                const X: usize = generator::PATTERN_SIZE - 1;
                assert_eq!(
                    generator.grid[X][y].contains(&i),
                    generator.patterns[i].get_right_border()
                );
            }
        }
        {
            assert_eq!(
                generator.grid[0][0].contains(&i),
                generator.patterns[i].get_top_border() && generator.patterns[i].get_left_border()
            );
            assert_eq!(
                generator.grid[0][generator::PATTERN_SIZE - 1].contains(&i),
                generator.patterns[i].get_top_border() && generator.patterns[i].get_right_border()
            );
            assert_eq!(
                generator.grid[generator::PATTERN_SIZE - 1][0].contains(&i),
                generator.patterns[i].get_bottom_border()
                    && generator.patterns[i].get_left_border()
            );
            assert_eq!(
                generator.grid[generator::PATTERN_SIZE - 1][generator::PATTERN_SIZE - 1]
                    .contains(&i),
                generator.patterns[i].get_bottom_border()
                    && generator.patterns[i].get_right_border()
            );
        }
    }
}
