pub struct Map<const MAX_STRAIGHT_WALKS: u8> {
    fields: Vec<Vec<Field<MAX_STRAIGHT_WALKS>>>,
}
impl<const MAX_STRAIGHT_WALKS: u8> Map<MAX_STRAIGHT_WALKS> {
    pub fn parse(s: &str) -> Self {
        let fields = s
            .lines()
            .map(|line| line.chars().map(Field::parse).collect())
            .collect();
        Self { fields }
    }

    pub fn get_field(&self, coords: Coords) -> Option<&Field<MAX_STRAIGHT_WALKS>> {
        self.fields.get(coords.y)?.get(coords.x)
    }

    pub fn get_field_mut(&mut self, coords: Coords) -> Option<&mut Field<MAX_STRAIGHT_WALKS>> {
        self.fields.get_mut(coords.y)?.get_mut(coords.x)
    }

    pub fn width(&self) -> usize {
        self.fields[0].len()
    }

    pub fn height(&self) -> usize {
        self.fields.len()
    }
}

pub struct Field<const MAX_STRAIGHT_WALKS: u8> {
    heat_loss: u32,
    trace_cache: TraceCache<MAX_STRAIGHT_WALKS>,
}

impl<const MAX_STRAIGHT_WALKS: u8> Field<MAX_STRAIGHT_WALKS> {
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

struct TraceCache<const MAX_STRAIGHT_WALKS: u8> {
    traces: Vec<Option<u32>>,
}

impl<const MAX_STRAIGHT_WALKS: u8> Default for TraceCache<MAX_STRAIGHT_WALKS> {
    fn default() -> Self {
        let size = usize::from(Self::TRACE_CACHE_SIZE);
        let traces = vec![None; size];
        Self { traces }
    }
}

impl<const MAX_STRAIGHT_WALKS: u8> TraceCache<MAX_STRAIGHT_WALKS> {
    const POSSIBLE_DIRECTIONS: u8 = 4;
    const TRACE_CACHE_SIZE: u8 = Self::POSSIBLE_DIRECTIONS * (MAX_STRAIGHT_WALKS + 1);

    fn calc_trace_index(direction: Direction, num_straight_walks: u8) -> usize {
        let directions_id = match direction {
            Direction::Up => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Right => 3,
        };
        let max_straight_walks = usize::from(MAX_STRAIGHT_WALKS);
        let walks_id = usize::from(num_straight_walks);
        directions_id * max_straight_walks + walks_id
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
