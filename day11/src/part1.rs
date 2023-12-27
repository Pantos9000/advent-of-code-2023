use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let mut universe = Universe::from_str(input).unwrap();
    universe.expand(1);
    sum_distances(&universe)
}

pub fn sum_distances(universe: &Universe) -> usize {
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

    /// each empty row/col will be expanded by the given `expansion_rate`.
    pub fn expand(&mut self, expansion_rate: usize) {
        let mut empty_rows = Vec::new();
        let mut empty_cols = Vec::new();

        for x in 0..self.size_x {
            if self.is_column_empty(x) {
                empty_cols.push(x);
            }
        }

        for y in 0..self.size_y {
            if self.is_row_empty(y) {
                empty_rows.push(y);
            }
        }

        for galaxy in &mut self.galaxies {
            let num_empty_cols_before_galaxy = empty_cols.iter().filter(|x| **x < galaxy.x).count();
            let num_empty_rows_before_galaxy = empty_rows.iter().filter(|y| **y < galaxy.y).count();
            galaxy.x += num_empty_cols_before_galaxy * expansion_rate;
            galaxy.y += num_empty_rows_before_galaxy * expansion_rate;
        }

        self.size_x += empty_cols.len();
        self.size_y += empty_rows.len();
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
    fn test_universe_expand_rate_1() {
        let input = "\
            .#.#\n\
            ....\n\
            .#..\n\
            ....\n\
            #...";
        let mut universe = Universe::from_str(input).unwrap();
        universe.expand(1);

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
