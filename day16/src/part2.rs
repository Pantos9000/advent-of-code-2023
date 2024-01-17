use crate::direction::Direction;
use crate::hamster::QuantumHamster;
use crate::map::{Coords, Map};
use crate::part1::fire_hamster;

pub fn run(input: &str) -> usize {
    let map = Map::parse(input);

    hamsters_from_above(&map)
        .chain(hamsters_from_below(&map))
        .chain(hamsters_from_left(&map))
        .chain(hamsters_from_right(&map))
        .map(|hamster| fire_hamster(map.clone(), hamster))
        .max()
        .unwrap()
}

fn hamsters_from_above(map: &Map) -> impl Iterator<Item = QuantumHamster> {
    let range = 0..map.width();
    let y = 0; // first row
    let direction = Direction::Down;
    range
        .map(move |x| Coords::new(x, y))
        .map(move |position| QuantumHamster::new(position, direction))
}

fn hamsters_from_below(map: &Map) -> impl Iterator<Item = QuantumHamster> {
    let range = 0..map.width();
    let y = map.height() - 1; // last row
    let direction = Direction::Up;
    range
        .map(move |x| Coords::new(x, y))
        .map(move |position| QuantumHamster::new(position, direction))
}

fn hamsters_from_left(map: &Map) -> impl Iterator<Item = QuantumHamster> {
    let range = 0..map.height();
    let x = 0; // first col
    let direction = Direction::Right;
    range
        .map(move |y| Coords::new(x, y))
        .map(move |position| QuantumHamster::new(position, direction))
}

fn hamsters_from_right(map: &Map) -> impl Iterator<Item = QuantumHamster> {
    let range = 0..map.height();
    let x = map.width() - 1; // last col
    let direction = Direction::Left;
    range
        .map(move |y| Coords::new(x, y))
        .map(move |position| QuantumHamster::new(position, direction))
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
    fn hamsters_from_the_side_count_is_correct() {
        let map = Map::parse(EXAMPLE);
        let num_hamsters = hamsters_from_above(&map)
            .chain(hamsters_from_below(&map))
            .chain(hamsters_from_left(&map))
            .chain(hamsters_from_right(&map))
            .count();
        let expected = 2 * (map.width() + map.height());
        assert_eq!(num_hamsters, expected);
    }

    #[test]
    fn test_example() {
        assert_eq!(run(EXAMPLE), 51);
    }

    #[test]
    fn test_example_right_hamster() {
        let map = Map::parse(EXAMPLE);
        let hamster = QuantumHamster::new(Coords::new(3, 0), Direction::Down);
        assert_eq!(fire_hamster(map, hamster), 51);
    }
}
