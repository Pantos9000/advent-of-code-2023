use crate::direction::Direction;

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

    pub fn num_visited_fields(&self) -> usize {
        self.fields
            .iter()
            .flatten()
            .filter(|field| field.was_visited())
            .count()
    }
}

pub struct Field {
    shape: Shape,
    traversed: Traversed,
}
impl Field {
    fn parse(c: char) -> Self {
        let shape = Shape::parse(c);
        let traversed = Traversed::default();
        Self { shape, traversed }
    }

    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// Returns error if traversial was already done
    pub fn traverse(&mut self, direction: Direction) -> Result<(), ()> {
        if self.traversed.was_traversed(direction) {
            return Err(());
        }
        self.traversed.traverse(direction);
        Ok(())
    }

    pub fn was_visited(&self) -> bool {
        self.traversed.was_visited()
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
pub enum Shape {
    Empty,
    MirrorForward,
    MirrorBackward,
    SplitterHorizontal,
    SplitterVertical,
}

impl Shape {
    fn parse(c: char) -> Self {
        match c {
            '.' => Self::Empty,
            '/' => Self::MirrorForward,
            '\\' => Self::MirrorBackward,
            '-' => Self::SplitterHorizontal,
            '|' => Self::SplitterVertical,
            _ => panic!("unknown char {c}"),
        }
    }
}

#[derive(Debug, Default, Clone, PartialEq, Eq)]
struct Traversed {
    up: bool,
    down: bool,
    left: bool,
    right: bool,
}

impl Traversed {
    fn traverse(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.up = true,
            Direction::Down => self.down = true,
            Direction::Left => self.left = true,
            Direction::Right => self.right = true,
        }
    }

    fn was_traversed(&self, direction: Direction) -> bool {
        match direction {
            Direction::Up => self.up,
            Direction::Down => self.down,
            Direction::Left => self.left,
            Direction::Right => self.right,
        }
    }

    fn was_visited(&self) -> bool {
        self.up || self.down || self.left || self.right
    }
}
