use crate::map::{Coords, Direction, Map};

pub struct QuantumHamster {
    position: Coords,
    direction: Direction,
    num_straight_walks: u8,
    heat_trace: u32,
}

impl PartialEq for QuantumHamster {
    fn eq(&self, other: &Self) -> bool {
        self.heat_trace.eq(&other.heat_trace)
    }
}

impl Eq for QuantumHamster {}

// Hamsters with less heat loss are better, so reverse the ordering
impl std::cmp::PartialOrd for QuantumHamster {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Hamsters with less heat loss are better, so reverse the ordering
impl std::cmp::Ord for QuantumHamster {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_trace.cmp(&self.heat_trace)
    }
}

impl QuantumHamster {
    const MAX_STRAIGHT_WALKS: u8 = 3;

    pub fn new(position: Coords, direction: Direction) -> Self {
        Self {
            position,
            direction,
            num_straight_walks: 0,
            heat_trace: 0,
        }
    }

    fn superposition(&self, new_direction: Direction) -> Self {
        Self {
            position: self.position,
            direction: new_direction,
            num_straight_walks: 0,
            heat_trace: self.heat_trace,
        }
    }

    fn take_heat_and_leave_trace(mut self, map: &mut Map) -> Option<Self> {
        let field = map.get_field_mut(self.position)?;
        self.heat_trace += field.heat_loss();
        field
            .leave_trace(self.direction, self.num_straight_walks, self.heat_trace)
            .ok()?;
        Some(self)
    }

    /// Quantum Hamster will try to reorient and follow all possible ways. As it is quantum,
    /// it will split up and go into superposition, returning two evil parallel-universe-versions
    /// of itself.
    ///
    /// If it is possible to walk straight, it will even go into hyper-position (3 hamsters).
    pub fn reorient(self) -> (Self, Self, Option<Self>) {
        let hamster_a = self.superposition(self.direction.left());
        let hamster_b = self.superposition(self.direction.right());
        let hamster_c = if self.num_straight_walks < Self::MAX_STRAIGHT_WALKS {
            Some(self)
        } else {
            None
        };

        (hamster_a, hamster_b, hamster_c)
    }

    /// Quantum hamster will try to walk into the direction it is facing, but its wave function
    /// will cease to exist on the edges of the map and when encountering a trail of another
    /// quantum hamster that was more efficient.
    ///
    /// Luckily each [`QuantumHamster`] is evil by definition, so it's okay if it ceases to exist.
    /// Nobody will miss it.
    ///
    /// Furthermore, this prevents the evil hamsters from clinging together, forming a fascist
    /// government and conquering all the RAM.
    pub fn walk(mut self, map: &mut Map) -> Option<Self> {
        self = self.take_heat_and_leave_trace(map)?;

        let next_position = self.position.move_into_direction(self.direction)?;
        self.position = next_position;
        self.num_straight_walks += 1;
        Some(self)
    }
}
