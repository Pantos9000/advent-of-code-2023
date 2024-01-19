use crate::hamster::QuantumHamster;
use crate::map::{Coords, Direction, Map};

use std::collections::BinaryHeap;

pub fn run(input: &str) -> usize {
    let mut map = Map::parse(input);
    let starting_position = Coords::new(0, 0);

    let mut hamster = QuantumHamster::new(starting_position, Direction::Down);
    hamster.take_heat(&map);

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
