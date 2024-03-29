use crate::hamster::QuantumHamster;
use crate::map::{Coords, Direction, Map};

use std::collections::BinaryHeap;

pub fn run(input: &str) -> usize {
    const MAX_STRAIGHT_WALKS: u8 = 3;
    const MIN_STRAIGHT_WALKS: u8 = 0;

    hamster_wheel::<MIN_STRAIGHT_WALKS, MAX_STRAIGHT_WALKS>(input)
}

pub fn hamster_wheel<const MIN_STRAIGHT_WALKS: u8, const MAX_STRAIGHT_WALKS: u8>(
    input: &str,
) -> usize {
    let mut map = Map::parse(input);
    let starting_position = Coords::new(0, 0);

    let hamster_down = QuantumHamster::<MIN_STRAIGHT_WALKS, MAX_STRAIGHT_WALKS>::new(
        starting_position,
        Direction::Down,
    );

    let hamster_right = QuantumHamster::<MIN_STRAIGHT_WALKS, MAX_STRAIGHT_WALKS>::new(
        starting_position,
        Direction::Right,
    );

    let mut hamsters = BinaryHeap::new();
    hamsters.push(hamster_down);
    hamsters.push(hamster_right);

    while let Some(hamster) = hamsters.pop() {
        let (alive_hamster, dead_hamster, zombie_hamster) = hamster.reorient();

        if let Some(alive_hamster) = alive_hamster {
            if let Some(alive_hamster) = alive_hamster.walk(&mut map) {
                hamsters.push(alive_hamster);
            }
        }
        if let Some(dead_hamster) = dead_hamster {
            if let Some(dead_hamster) = dead_hamster.walk(&mut map) {
                hamsters.push(dead_hamster);
            }
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

    #[test]
    fn test_2x2() {
        let input = "\
            29\n\
            13";
        assert_eq!(run(input), 4);
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
        assert_eq!(run(input), 15);
    }

    #[test]
    fn test_vertical_snake() {
        let input = "\
            19111\n\
            19191\n\
            19191\n\
            11191";
        assert_eq!(run(input), 13);
    }

    #[test]
    fn test_max3_horizonal() {
        let input = "\
            1111111111111\n\
            1111111111111";
        assert_eq!(run(input), 15);
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
        assert_eq!(run(input), 15);
    }
}
