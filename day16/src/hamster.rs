use crate::direction::Direction;
use crate::map::{Coords, Map};

pub struct QuantumHamster {
    position: Coords,
    direction: Direction,
}

impl QuantumHamster {
    pub fn new(position: Coords, direction: Direction) -> Self {
        Self {
            position,
            direction,
        }
    }

    /// Quantum Hamster will try to reorient and follow the shape of the tile. As it is quantum,
    /// it may go into superposition and return an evil parallel-universe-version of itself.
    pub fn reorient(&mut self, map: &Map) -> Option<Self> {
        let current_field = map.get_field(self.position).unwrap();
        let (next_direction_a, next_direction_b) = self.direction.follow(current_field.shape());
        self.direction = next_direction_a;
        next_direction_b.map(|direction| Self::new(self.position, direction))
    }

    /// Quantum hamster will try to walk into the direction it is facing, but it will interfere
    /// negatively with itself at the end of the map and with tiles already visited by other hamsters.
    ///
    /// Luckily each [`QuantumHamster`] is evil by definition, so it's okay if it ceases to exist.
    /// Nobody will miss it.
    ///
    /// Furthermore, this prevents the evil hamsters from clinging together, forming a fascist
    /// government and conquering all the RAM.
    pub fn walk(mut self, map: &mut Map) -> Option<Self> {
        let current_field = map.get_field_mut(self.position).unwrap();
        current_field.traverse(self.direction).ok()?;

        let next_position = self.position.move_into_direction(self.direction)?;
        let next_field = map.get_field_mut(next_position)?;
        next_field.traverse(self.direction.opposite()).ok()?;

        self.position = next_position;
        Some(self)
    }
}
