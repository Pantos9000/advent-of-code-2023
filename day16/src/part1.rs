use crate::direction::Direction;
use crate::hamster::QuantumHamster;
use crate::map::{Coords, Map};

pub fn run(input: &str) -> usize {
    let map = Map::parse(input);
    let starting_position = Coords::new(0, 0);
    let starting_direction = Direction::Right;
    let hamster = QuantumHamster::new(starting_position, starting_direction);

    fire_hamster(map, hamster)
}

pub fn fire_hamster(mut map: Map, hamster: QuantumHamster) -> usize {
    let mut hamsters = vec![hamster];

    while let Some(mut hamster) = hamsters.pop() {
        if let Some(new_hamster) = hamster.reorient(&map) {
            hamsters.push(new_hamster);
        }
        while let Some(mut same_hamster) = hamster.walk(&mut map) {
            if let Some(new_hamster) = same_hamster.reorient(&map) {
                hamsters.push(new_hamster);
            }
            hamster = same_hamster;
        }
    }
    map.num_visited_fields()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        .|...\\....\n\
        |.-.\\.....\n\
        .....|-...\n\
        ........|.\n\
        ..........\n\
        .........\\\n\
        ..../.\\\\..\n\
        .-.-/..|..\n\
        .|....-|.\\\n\
        ..//.|....";

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 46);
    }
}
