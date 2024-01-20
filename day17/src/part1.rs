use crate::hamster::QuantumHamster;
use crate::map::{Coords, Direction, Map};

use std::collections::BinaryHeap;

pub fn run(input: &str) -> usize {
    let mut map = Map::parse(input);
    let starting_position = Coords::new(0, 0);

    let hamster = QuantumHamster::new(starting_position, Direction::Down);

    let mut hamsters = BinaryHeap::new();
    hamsters.push(hamster);

    while let Some(hamster) = hamsters.pop() {
        let (alive_hamster, dead_hamster, zombie_hamster) = hamster.reorient();

        if let Some(alive_hamster) = alive_hamster.walk(&mut map) {
            hamsters.push(alive_hamster);
        }
        if let Some(dead_hamster) = dead_hamster.walk(&mut map) {
            hamsters.push(dead_hamster);
        }
        if let Some(zombie_hamster) = zombie_hamster {
            if let Some(zombie_hamster) = zombie_hamster.walk(&mut map) {
                hamsters.push(zombie_hamster);
            }
        }
    }

    let target_position = Coords::new(map.width() - 1, map.height() - 1);
    let overall_heat_loss = map
        .get_field(target_position)
        .unwrap()
        .smallest_trace()
        .unwrap();

    overall_heat_loss.try_into().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533";

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 102);
    }

    #[test]
    fn test_2x2() {
        let input = "\
            29\n\
            13";
        assert_eq!(run(input), 6);
    }

    #[test]
    fn test_horizontal_snake() {
        let input = "\
            1111\n\
            9991\n\
            9991\n\
            1111\n\
            1999\n\
            1999\n\
            1111";
        assert_eq!(run(input), 16);
    }

    #[test]
    fn test_vertical_snake() {
        let input = "\
            19111\n\
            19191\n\
            19191\n\
            11191";
        assert_eq!(run(input), 14);
    }

    #[test]
    fn test_max3_horizonal() {
        let input = "\
            1111111111111\n\
            1111111111111";
        assert_eq!(run(input), 16);
    }

    #[test]
    fn test_max3_vertical() {
        let input = "\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11\n\
            11";
        assert_eq!(run(input), 16);
    }
}
