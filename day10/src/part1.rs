use std::iter;

pub fn run(input: &str) -> usize {
    let maze = Maze::new(input, '-');
    let mut hamster_0 = Hamster::new(maze.start(), Direction::Right);
    let mut hamster_1 = Hamster::new(maze.start(), Direction::Left);
    let mut num_steps = 0;

    'hamsters_walking: loop {
        hamster_0.walk_maze(&maze);
        hamster_1.walk_maze(&maze);
        num_steps += 1;

        if hamster_0.position == hamster_1.position() {
            break 'hamsters_walking;
        }
    }

    num_steps
}

/// Follows pipes
pub struct Hamster {
    position: Coords,
    direction: Direction,
}

impl Hamster {
    /// `entering_direction` is the direction the hamster won't walk (it won't walk backwards)
    pub fn new(position: Coords, entering_direction: Direction) -> Self {
        Self {
            position,
            direction: entering_direction,
        }
    }

    /// walk one field further
    pub fn walk_maze(&mut self, maze: &Maze) {
        let direction_to_walk = self.exiting_direction(maze);
        self.position = self
            .position
            .move_into_direction(direction_to_walk)
            .unwrap();
        self.direction = direction_to_walk;
    }

    pub fn position(&self) -> Coords {
        self.position
    }

    /// Get the direction the hamster is currently facing, which is equal to the
    /// side of the current pipe where it entered.
    pub fn direction(&self) -> Direction {
        self.direction
    }

    pub fn exiting_direction(&self, maze: &Maze) -> Direction {
        maze.get_field(self.position)
            .unwrap()
            .pipe()
            .unwrap()
            .follow(self.direction)
    }

    pub fn peek<'maze>(&self, maze: &'maze Maze, direction: Direction) -> &'maze Field {
        let coords = self.position().move_into_direction(direction).unwrap();
        maze.get_field(coords).unwrap()
    }

    pub fn take_dump(&self, maze: &mut Maze) {
        let field = maze.get_field_mut(self.position()).unwrap();
        field.pipe_mut().unwrap().fill();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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

/// Coords are `[y][x]`
pub struct Maze {
    fields: Vec<Vec<Field>>,
    start: Coords,
}

impl Maze {
    /// starting position will be overwritten after parsing with `start_char`
    pub fn new(input: &str, start_char: char) -> Self {
        fn parse_line(line: &str) -> Vec<Field> {
            let prepend = iter::once(Field::default());
            let append = prepend.clone();
            let parse_iter = line.chars().map(Field::from);
            prepend.chain(parse_iter).chain(append).collect()
        }

        let line_len = input.find('\n').unwrap_or(input.len()) + 2;
        let prepend = iter::once(vec![Field::default(); line_len]);
        let append = prepend.clone();

        let fields_iter = input.lines().map(parse_line);
        let mut fields: Vec<Vec<_>> = prepend.chain(fields_iter).chain(append).collect();

        let start = Self::find_start_coords(&fields).unwrap();

        // overwrite starting field
        fields[start.y][start.x] = Field::from(start_char);

        Self { fields, start }
    }

    pub fn find_start_coords(fields: &[Vec<Field>]) -> Option<Coords> {
        let (x, y) = fields.iter().enumerate().find_map(|(y, line)| {
            line.iter()
                .enumerate()
                .find(|(_x, field)| field.is_start)
                .map(|(x, _field)| (x, y))
        })?;
        Some(Coords::new(x, y))
    }

    pub fn start(&self) -> Coords {
        self.start
    }

    pub fn get_field(&self, coords: Coords) -> Option<&Field> {
        self.fields.get(coords.y)?.get(coords.x)
    }

    pub fn get_field_mut(&mut self, coords: Coords) -> Option<&mut Field> {
        self.fields.get_mut(coords.y)?.get_mut(coords.x)
    }

    pub fn size(&self) -> usize {
        self.fields.len() * self.fields[0].len()
    }

    pub fn into_iter(self) -> impl Iterator<Item = Field> {
        self.fields.into_iter().flat_map(|v| v.into_iter())
    }
}

#[derive(Debug, Default, Clone, PartialEq)]
pub struct Field {
    pipe: Option<Pipe>,
    is_start: bool,
    is_outer: bool,
    is_inner: bool,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        let pipe = match value {
            '.' => None,
            'S' => None,
            _ => Some(Pipe::from(value)),
        };
        let is_start = value == 'S';

        Self {
            pipe,
            is_start,
            is_outer: false,
            is_inner: false,
        }
    }
}

impl Field {
    pub fn pipe(&self) -> Option<&Pipe> {
        self.pipe.as_ref()
    }

    pub fn pipe_mut(&mut self) -> Option<&mut Pipe> {
        self.pipe.as_mut()
    }

    pub fn mark_outer(&mut self) {
        self.is_outer = true;
    }

    pub fn is_outer(&self) -> bool {
        self.is_outer
    }

    pub fn mark_inner(&mut self) {
        self.is_inner = true;
    }

    pub fn is_inner(&self) -> bool {
        self.is_inner
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Pipe {
    openings: [Direction; 2],
    full: bool,
}

impl From<char> for Pipe {
    fn from(value: char) -> Self {
        let openings = match value {
            '|' => [Direction::Up, Direction::Down],
            '-' => [Direction::Left, Direction::Right],
            'L' => [Direction::Up, Direction::Right],
            'J' => [Direction::Up, Direction::Left],
            '7' => [Direction::Left, Direction::Down],
            'F' => [Direction::Right, Direction::Down],
            _ => panic!("unknown pipe form '{value}"),
        };
        Self {
            openings,
            full: false,
        }
    }
}

impl Pipe {
    fn openings(&self) -> &[Direction; 2] {
        &self.openings
    }

    pub fn fill(&mut self) {
        self.full = true;
    }

    pub fn is_full(&self) -> bool {
        self.full
    }

    pub fn follow(&self, entering_direction: Direction) -> Direction {
        let exit_direction = self
            .openings()
            .iter()
            .find(|direction| !direction.is_opposite(entering_direction))
            .unwrap();
        *exit_direction
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn is_opposite(&self, other: Direction) -> bool {
        self.opposite() == other
    }

    pub fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
        }
    }

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_start() {
        let input = "\
        ....\n\
        ..S.\n\
        ....";
        let maze = Maze::new(input, '.');
        let coords = maze.start();
        assert_eq!(coords.x, 3);
        assert_eq!(coords.y, 2);
    }

    #[test]
    fn test_maze() {
        let input = "\
            .S-7.\n\
            .|.|.\n\
            .L-J.";
        let maze = Maze::new(input, 'F');
        assert_eq!(
            maze.fields
                .iter()
                .flatten()
                .filter(|field| field.pipe.is_some())
                .count(),
            8
        );
    }

    #[test]
    fn test_hamster() {
        let input = "\
        -L|F7\n\
        7S-7|\n\
        L|7||\n\
        -L-J|\n\
        L|-JF";
        let maze = Maze::new(input, 'F');
        assert_eq!(maze.start().x, 2);
        assert_eq!(maze.start().y, 2);

        let mut hamster = Hamster::new(maze.start(), Direction::Left);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position().x, 2);
        assert_eq!(hamster.position().y, 3);

        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position().x, 4);
        assert_eq!(hamster.position().y, 4);

        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position(), maze.start());
    }
}
