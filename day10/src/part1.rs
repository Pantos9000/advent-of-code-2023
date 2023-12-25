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
        let field = maze.get_field(self.position);
        let pipe = field.pipe.as_ref().unwrap();
        let direction_to_walk = pipe
            .openings()
            .iter()
            .find(|direction| !direction.is_opposite(self.direction))
            .unwrap();
        self.position.move_into_direction(*direction_to_walk);
        self.direction = *direction_to_walk;
    }

    pub fn position(&self) -> Coords {
        self.position
    }

    pub fn take_dump(&self, maze: &mut Maze) {
        let position = self.position();
        let field = maze.get_field_mut(position);
        field.pipe.unwrap().fill();
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Coords {
    x: usize,
    y: usize,
}

impl Coords {
    fn new(x: usize, y: usize) -> Self {
        Coords { x, y }
    }

    fn move_into_direction(&mut self, direction: Direction) {
        match direction {
            Direction::Up => self.y -= 1,
            Direction::Down => self.y += 1,
            Direction::Left => self.x -= 1,
            Direction::Right => self.x += 1,
        }
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
        let mut fields: Vec<Vec<_>> = input
            .lines()
            .map(|line| line.chars().map(Field::from).collect())
            .collect();
        let start = Self::find_start_coords(input).unwrap();

        // overwrite starting field
        fields[start.y][start.x] = Field::from(start_char);

        Self { fields, start }
    }
    pub fn find_start_coords(input: &str) -> Option<Coords> {
        let (x, y) = input
            .lines()
            .enumerate()
            .find_map(|(y, line)| line.find('S').map(|x| (x, y)))?;
        Some(Coords::new(x, y))
    }

    pub fn start(&self) -> Coords {
        self.start
    }

    pub fn get_field(&self, coords: Coords) -> Field {
        self.fields[coords.y][coords.x]
    }

    pub fn get_field_mut(&mut self, coords: Coords) -> &mut Field {
        &mut self.fields[coords.y][coords.x]
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Field {
    pipe: Option<Pipe>,
}

impl From<char> for Field {
    fn from(value: char) -> Self {
        let pipe = match value {
            '.' => None,
            'S' => None,
            _ => Some(Pipe::from(value)),
        };

        Self { pipe }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
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
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn is_opposite(&self, other: Direction) -> bool {
        self.opposite() == other
    }

    fn opposite(&self) -> Self {
        match self {
            Self::Up => Self::Down,
            Self::Down => Self::Up,
            Self::Left => Self::Right,
            Self::Right => Self::Left,
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
        let coords = Maze::find_start_coords(input).unwrap();
        assert_eq!(coords.x, 2);
        assert_eq!(coords.y, 1);
    }

    #[test]
    fn test_maze() {
        let input = "\
            .S-7.\n\
            .|.|.\n\
            .L-J.";
        let maze = Maze::new(input, 'F');
        assert_eq!(maze.fields.iter().flatten().count(), 15);
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
        assert_eq!(maze.start().x, 1);
        assert_eq!(maze.start().y, 1);

        let mut hamster = Hamster::new(maze.start(), Direction::Left);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position().x, 1);
        assert_eq!(hamster.position().y, 2);

        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position().x, 3);
        assert_eq!(hamster.position().y, 3);

        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        hamster.walk_maze(&maze);
        assert_eq!(hamster.position(), maze.start());
    }
}
