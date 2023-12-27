use crate::part1::{Coords, Direction, Hamster, Maze};

pub fn run(input: &str) -> usize {
    run_private(input, '-')
}

fn run_private(input: &str, start_char: char) -> usize {
    let mut maze = Maze::new(input, start_char);
    pooping_hamster(&mut maze);
    flood_outer(&mut maze);
    let (flood_start, flood_direction, inner_side) = find_inner_direction_hamster(&mut maze);
    flooding_hamster(&mut maze, flood_start, flood_direction, inner_side);

    maze.into_iter().filter(|field| field.is_inner()).count()
}

enum Side {
    Left,
    Right,
}

/// runs along the pipe and poops everywhere.
fn pooping_hamster(maze: &mut Maze) {
    let mut hamster = Hamster::new(maze.start(), Direction::Left);

    // pooploop
    while !maze
        .get_field(hamster.position())
        .unwrap()
        .pipe()
        .map(|pipe| pipe.is_full())
        .unwrap_or(false)
    {
        hamster.take_dump(maze);
        hamster.walk_maze(maze);
    }
}

/// This hamster finds a position in the loop where either its left side is outer and the right
/// side is inner, or vice versa.
///
/// It then returns the [`Coords`], the entering [`Direction`] of the hamster in that field,
/// and where the inner [`Side`] is.
fn find_inner_direction_hamster(maze: &mut Maze) -> (Coords, Direction, Side) {
    let mut hamster = Hamster::new(maze.start(), Direction::Left);

    // off by one, hrm
    loop {
        let left_field = hamster.peek(maze, hamster.direction().left());
        let right_field = hamster.peek(maze, hamster.direction().right());

        let inner_side = if left_field.is_outer() {
            Some(Side::Right)
        } else if right_field.is_outer() {
            Some(Side::Left)
        } else {
            None
        };

        if let Some(side) = inner_side {
            return (hamster.position(), hamster.direction(), side);
        }

        hamster.walk_maze(maze);
    }
}

fn flooding_hamster(maze: &mut Maze, start: Coords, start_direction: Direction, inner_side: Side) {
    let mut hamster = Hamster::new(start, start_direction);
    hamster.walk_maze(maze);

    fn get_inner_coords(hamster: &Hamster, maze: &Maze, direction: Direction) -> Option<Coords> {
        let inner_field = hamster.peek(maze, direction);
        if inner_field.is_inner() {
            return None;
        }
        let inner_position = hamster.position().move_into_direction(direction).unwrap();
        Some(inner_position)
    }

    while hamster.position() != start {
        let inner_direction_entering = match inner_side {
            Side::Left => hamster.direction().left(),
            Side::Right => hamster.direction().right(),
        };
        let inner_direction_exiting = match inner_side {
            Side::Left => hamster.exiting_direction(maze).left(),
            Side::Right => hamster.exiting_direction(maze).right(),
        };
        if let Some(inner_position) = get_inner_coords(&hamster, maze, inner_direction_entering) {
            flood_inner(maze, inner_position);
        }
        if let Some(inner_position) = get_inner_coords(&hamster, maze, inner_direction_exiting) {
            flood_inner(maze, inner_position);
        }
        hamster.walk_maze(maze);
    }
}

fn flood_outer(maze: &mut Maze) {
    let start = Coords::new(0, 0);
    flood(maze, start, false);
}

fn flood_inner(maze: &mut Maze, start: Coords) {
    flood(maze, start, true);
}

fn flood(maze: &mut Maze, start: Coords, inner: bool) {
    let mut field_stack = Vec::with_capacity(maze.size());

    field_stack.push(start);

    'floodloop: while let Some(pos) = field_stack.pop() {
        let Some(field) = maze.get_field_mut(pos) else {
            continue 'floodloop;
        };

        let already_marked = if inner {
            field.is_inner()
        } else {
            field.is_outer()
        };
        if already_marked {
            continue 'floodloop;
        }

        if let Some(pipe) = field.pipe() {
            if pipe.is_full() {
                continue 'floodloop;
            }
        }

        if inner {
            field.mark_inner();
        } else {
            field.mark_outer();
        }

        if let Some(field) = pos.move_into_direction(Direction::Up) {
            field_stack.push(field);
        }
        if let Some(field) = pos.move_into_direction(Direction::Down) {
            field_stack.push(field);
        }
        if let Some(field) = pos.move_into_direction(Direction::Left) {
            field_stack.push(field);
        }
        if let Some(field) = pos.move_into_direction(Direction::Right) {
            field_stack.push(field);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_flood_outer_all() {
        let input = "\
            S|.\n\
            .|.";
        let mut maze = Maze::new(input, '.');
        flood_outer(&mut maze);
        maze.into_iter().for_each(|field| assert!(field.is_outer()));
    }

    #[test]
    fn test_flood_outer_is_stopped_by_poop() {
        let input = "\
            FS7.\n\
            |.|.\n\
            L-J.";
        let mut maze = Maze::new(input, '-');

        fn fill_field(maze: &mut Maze, x: usize, y: usize) {
            maze.get_field_mut(Coords::new(x, y))
                .unwrap()
                .pipe_mut()
                .unwrap()
                .fill();
        }
        fill_field(&mut maze, 1, 1);
        fill_field(&mut maze, 2, 1);
        fill_field(&mut maze, 3, 1);
        fill_field(&mut maze, 1, 2);
        fill_field(&mut maze, 3, 2);
        fill_field(&mut maze, 1, 3);
        fill_field(&mut maze, 2, 3);
        fill_field(&mut maze, 3, 3);

        flood_outer(&mut maze);
        assert_eq!(
            maze.into_iter().filter(|field| !field.is_outer()).count(),
            9
        );
    }

    #[test]
    fn test_part2_example() {
        let input = "\
            ..FS7.\n\
            .FJ.|.\n\
            .|..|.\n\
            .L--J.";
        assert_eq!(run(input), 3);
    }

    #[test]
    fn test_part2_example_2() {
        let input = "\
            ..........\n\
            .S------7.\n\
            .|F----7|.\n\
            .||..|.||.\n\
            .||....||.\n\
            .|L-7F-J|.\n\
            .|..||..|.\n\
            .L--JL--J.\n\
            ..........";
        assert_eq!(run_private(input, 'F'), 4);
    }

    #[test]
    fn test_part2_example_3() {
        let input = "\
            .F----7F7F7F7F-7....\n\
            .|F--7||||||||FJ....\n\
            .||.FJ||||||||L7....\n\
            FJL7L7LJLJ||LJ.L-7..\n\
            L--J.L7...LJS7F-7L7.\n\
            ....F-J..F7FJ|L7L7L7\n\
            ....L7.F7||L7|.L7L7|\n\
            .....|FJLJ|FJ|F7|.LJ\n\
            ....FJL-7.||.||||...\n\
            ....L---J.LJ.LJLJ...";
        assert_eq!(run_private(input, 'F'), 8);
    }

    #[test]
    fn test_part2_example_4() {
        let input = "\
            FF7FSF7F7F7F7F7F---7\n\
            L|LJ||||||||||||F--J\n\
            FL-7LJLJ||||||LJL-77\n\
            F--JF--7||LJLJ7F7FJ-\n\
            L---JF-JLJ.||-FJLJJ7\n\
            |F|F-JF---7F7-L7L|7|\n\
            |FFJF7L7F-JF7|JL---7\n\
            7-L-JL7||F7|L7F-7F7|\n\
            L.L7LFJ|||||FJL7||LJ\n\
            L7JLJL-JLJLJL--JLJ.L";
        assert_eq!(run_private(input, '7'), 10);
    }
}
