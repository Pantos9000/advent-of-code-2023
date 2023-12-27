use crate::part1::{sum_distances, Universe};

use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut universe = Universe::from_str(input).unwrap();
    universe.expand(1_000_000);
    sum_distances(&universe)
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &'static str = "\
        ...#......\n\
        .......#..\n\
        #.........\n\
        ..........\n\
        ......#...\n\
        .#........\n\
        .........#\n\
        ..........\n\
        .......#..\n\
        #...#.....";

    #[test]
    fn test_example_with_rate_10() {
        let mut universe = Universe::from_str(EXAMPLE).unwrap();
        universe.expand(10);
        assert_eq!(sum_distances(&universe), 1030);
    }

    #[test]
    fn test_example_with_rate_100() {
        let mut universe = Universe::from_str(EXAMPLE).unwrap();
        universe.expand(100);
        assert_eq!(sum_distances(&universe), 8410);
    }
}
