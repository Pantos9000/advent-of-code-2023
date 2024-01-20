use crate::map::{Coords, Direction, Map};

pub struct QuantumHamster<const MIN_STRAIGHT_WALKS: u8, const MAX_STRAIGHT_WALKS: u8> {
    position: Coords,
    direction: Direction,
    num_straight_walks: u8,
    heat_trace: u32,
}

impl<const W: u8, const Q: u8> PartialEq for QuantumHamster<W, Q> {
    fn eq(&self, other: &Self) -> bool {
        self.heat_trace.eq(&other.heat_trace)
    }
}

impl<const W: u8, const Q: u8> Eq for QuantumHamster<W, Q> {}

// Hamsters with less heat loss are better, so reverse the ordering
impl<const W: u8, const Q: u8> std::cmp::PartialOrd for QuantumHamster<W, Q> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// Hamsters with less heat loss are better, so reverse the ordering
impl<const W: u8, const Q: u8> std::cmp::Ord for QuantumHamster<W, Q> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.heat_trace.cmp(&self.heat_trace)
    }
}

impl<const MIN_STRAIGHT_WALKS: u8, const MAX_STRAIGHT_WALKS: u8>
    QuantumHamster<MIN_STRAIGHT_WALKS, MAX_STRAIGHT_WALKS>
{
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

    fn take_heat_and_leave_trace(mut self, map: &mut Map<MAX_STRAIGHT_WALKS>) -> Option<Self> {
        let field = map.get_field_mut(self.position)?;
        self.heat_trace += field.heat_loss();
        field
            .leave_trace(self.direction, self.num_straight_walks, self.heat_trace)
            .ok()?;
        Some(self)
    }

    /// Quantum Hamster will try to reorient and try to follow all possible ways. As it is quantum,
    /// it will split up and go into superposition, returning up to two additional evil
    /// parallel-universe-versions of itself.
    pub fn reorient(self) -> (Option<Self>, Option<Self>, Option<Self>) {
        let hamster_left = if self.num_straight_walks < MIN_STRAIGHT_WALKS {
            None
        } else {
            Some(self.superposition(self.direction.left()))
        };
        let hamster_right = if self.num_straight_walks < MIN_STRAIGHT_WALKS {
            None
        } else {
            Some(self.superposition(self.direction.right()))
        };
        let hamster_straight = if self.num_straight_walks < MAX_STRAIGHT_WALKS {
            Some(self)
        } else {
            None
        };

        (hamster_left, hamster_right, hamster_straight)
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
    pub fn walk(mut self, map: &mut Map<MAX_STRAIGHT_WALKS>) -> Option<Self> {
        let next_position = self.position.move_into_direction(self.direction)?;
        self.position = next_position;
        self.num_straight_walks += 1;

        self = self.take_heat_and_leave_trace(map)?;
        Some(self)
    }
}
