pub struct Map {
    fields: Vec<Vec<Field>>,
}
impl Map {
    pub fn parse(s: &str) -> Self {
        let fields = s
            .lines()
            .map(|line| line.chars().map(Field::parse).collect())
            .collect();
        Self { fields }
    }

    pub fn get_field(&self, coords: Coords) -> Option<&Field> {
        self.fields.get(coords.y)?.get(coords.x)
    }

    pub fn get_field_mut(&mut self, coords: Coords) -> Option<&mut Field> {
        self.fields.get_mut(coords.y)?.get_mut(coords.x)
    }

    pub fn width(&self) -> usize {
        self.fields[0].len()
    }

    pub fn height(&self) -> usize {
        self.fields.len()
    }
}

pub struct Field {
    heat_loss: u32,
    trace_cache: TraceCache,
}

impl Field {
    fn parse(c: char) -> Self {
        Self {
            heat_loss: c.to_digit(10).expect("unknown char '{c}'"),
            trace_cache: TraceCache::default(),
        }
    }

    pub fn heat_loss(&self) -> u32 {
        self.heat_loss
    }

    pub fn smallest_trace(&self) -> Option<u32> {
        self.trace_cache.smallest_trace()
    }

    /// set a new smallest hamster trace. Returns an error if the new trace is not smaller.
    pub fn leave_trace(
        &mut self,
        direction: Direction,
        num_straight_walks: u8,
        new_trace: u32,
    ) -> Result<(), ()> {
        self.trace_cache
            .leave_trace(direction, num_straight_walks, new_trace)
    }
}

#[derive(Default)]
struct TraceCache {
    traces: Box<[Option<u32>; Self::TRACE_CACHE_SIZE]>,
}

impl TraceCache {
    const POSSIBLE_DIRECTIONS: usize = 4;
    const POSSIBLE_STRAIGHT_WALKS: usize = 4;
    const TRACE_CACHE_SIZE: usize = Self::POSSIBLE_DIRECTIONS * Self::POSSIBLE_STRAIGHT_WALKS;

    fn calc_trace_index(direction: Direction, num_straight_walks: u8) -> usize {
        let directions_id = match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        };
        let walks_id = usize::from(num_straight_walks);
        directions_id * Self::POSSIBLE_STRAIGHT_WALKS + walks_id
    }

    pub fn leave_trace(
        &mut self,
        direction: Direction,
        num_straight_walks: u8,
        new_trace: u32,
    ) -> Result<(), ()> {
        let index = Self::calc_trace_index(direction, num_straight_walks);
        let maybe_old_trace = self.traces.get_mut(index).unwrap();
        if let Some(old_trace) = maybe_old_trace {
            if new_trace >= *old_trace {
                return Err(());
            }
        }
        *maybe_old_trace = Some(new_trace);
        Ok(())
    }

    pub fn smallest_trace(&self) -> Option<u32> {
        self.traces
            .iter()
            .filter_map(|trace| trace.as_ref().copied())
            .min()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }

    pub fn move_into_direction(&self, direction: Direction) -> Option<Self> {
        let mut ret = *self;
        match direction {
            Direction::Up => {
                if ret.y == 0 {
                    return None;
                } else {
                    ret.y -= 1
                }
            }
            Direction::Down => ret.y += 1,
            Direction::Left => {
                if ret.x == 0 {
                    return None;
                } else {
                    ret.x -= 1
                }
            }
            Direction::Right => ret.x += 1,
        }
        Some(ret)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn left(&self) -> Self {
        match self {
            Self::Up => Self::Left,
            Self::Down => Self::Right,
            Self::Left => Self::Down,
            Self::Right => Self::Up,
        }
    }

    pub fn right(&self) -> Self {
        match self {
            Self::Up => Self::Right,
            Self::Down => Self::Left,
            Self::Left => Self::Up,
            Self::Right => Self::Down,
        }
    }
}
