use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut universe = Universe::from_str(input).unwrap();
    universe.expand();
    sum_distances(&universe)
}

fn sum_distances(universe: &Universe) -> usize {
    let galaxies = universe.galaxies();
    galaxies
        .iter()
        .take(galaxies.len() - 1)
        .enumerate()
        .flat_map(|(index_a, galaxy_a)| {
            galaxies
                .iter()
                .skip(index_a + 1)
                .map(|galaxy_b| galaxy_a.distance(galaxy_b))
        })
        .sum()
}

#[derive(Debug, Clone, PartialEq)]
pub struct Galaxy {
    x: usize,
    y: usize,
}

impl Galaxy {
    fn new(x: usize, y: usize) -> Self {
        Self { x, y }
    }

    fn distance(&self, other: &Galaxy) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

pub struct Universe {
    galaxies: Vec<Galaxy>,
    size_x: usize,
    size_y: usize,
}

impl Universe {
    fn is_column_empty(&self, column: usize) -> bool {
        self.galaxies
            .iter()
            .filter(|galaxy| galaxy.x == column)
            .count()
            == 0
    }

    fn is_row_empty(&self, row: usize) -> bool {
        self.galaxies
            .iter()
            .filter(|galaxy| galaxy.y == row)
            .count()
            == 0
    }

    fn expand(&mut self) {
        let mut rows_to_double = Vec::new();
        let mut columns_to_double = Vec::new();

        for x in 0..self.size_x {
            if self.is_column_empty(x) {
                columns_to_double.push(x);
            }
        }

        for y in 0..self.size_y {
            if self.is_row_empty(y) {
                rows_to_double.push(y);
            }
        }

        for galaxy in &mut self.galaxies {
            let num_columns_doubled_before_galaxy =
                columns_to_double.iter().filter(|x| **x < galaxy.x).count();
            let num_rows_doubled_before_galaxy =
                rows_to_double.iter().filter(|y| **y < galaxy.y).count();
            galaxy.x += num_columns_doubled_before_galaxy;
            galaxy.y += num_rows_doubled_before_galaxy;
        }

        self.size_x += columns_to_double.len();
        self.size_y += rows_to_double.len();
    }

    fn galaxies(&self) -> &Vec<Galaxy> {
        &self.galaxies
    }
}

impl FromStr for Universe {
    type Err = ();

    fn from_str(input: &str) -> Result<Self, Self::Err> {
        fn parse_line_with_num((y, line): (usize, &str)) -> impl Iterator<Item = Galaxy> + '_ {
            line.chars()
                .enumerate()
                .filter(|(_x, c)| *c == '#')
                .map(move |(x, _c)| Galaxy::new(x, y))
        }

        let galaxies: Vec<_> = input
            .lines()
            .enumerate()
            .flat_map(parse_line_with_num)
            .collect();

        let size_x = input.lines().next().unwrap().len();
        let size_y = input.lines().count();

        Ok(Self {
            galaxies,
            size_x,
            size_y,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_universe_expand() {
        let input = "\
            .#.#\n\
            ....\n\
            .#..\n\
            ....\n\
            #...";
        let mut universe = Universe::from_str(input).unwrap();
        universe.expand();

        let galaxy = universe.galaxies.pop().unwrap();
        assert_eq!(galaxy.x, 0);
        assert_eq!(galaxy.y, 6);
        let galaxy = universe.galaxies.pop().unwrap();
        assert_eq!(galaxy.x, 1);
        assert_eq!(galaxy.y, 3);
        let galaxy = universe.galaxies.pop().unwrap();
        assert_eq!(galaxy.x, 4);
        assert_eq!(galaxy.y, 0);
        let galaxy = universe.galaxies.pop().unwrap();
        assert_eq!(galaxy.x, 1);
        assert_eq!(galaxy.y, 0);

        assert_eq!(universe.size_x, 5);
        assert_eq!(universe.size_y, 7);
    }

    #[test]
    fn test_distance_example() {
        let input = "\
            ....#........\n\
            .........#...\n\
            #............\n\
            .............\n\
            .............\n\
            ........#....\n\
            .#...........\n\
            ............#\n\
            .............\n\
            .............\n\
            .........#...\n\
            #....#.......";
        let universe = Universe::from_str(input).unwrap();
        assert_eq!(sum_distances(&universe), 374);
    }
}
