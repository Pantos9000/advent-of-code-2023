use std::str::FromStr;

pub fn run(input: &str) -> usize {
    let panel = Panel::from_str(input).unwrap();
    panel.calc_tilted_load()
}

pub struct Panel {
    positions: Vec<Vec<Position>>,
}

impl FromStr for Panel {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        fn parse_line(line: &str) -> Vec<Position> {
            line.chars().map(Position::from).collect()
        }
        let positions = s.lines().map(parse_line).collect();
        Ok(Self { positions })
    }
}

impl Panel {
    pub fn calc_tilted_load(&self) -> usize {
        let row_len = self.positions[0].len();
        let col_len = self.positions.len();
        let mut weights = vec![0; row_len];
        let mut tilt_corrections = vec![0; row_len];

        for (row, row_data) in self.positions.iter().enumerate() {
            for (col, pos) in row_data.iter().enumerate() {
                let pos_weight = col_len - row;
                match pos {
                    Position::Empty => tilt_corrections[col] += 1,
                    Position::Square => tilt_corrections[col] = 0,
                    Position::Round => {
                        weights[col] += pos_weight + tilt_corrections[col];
                    }
                }
            }
        }

        weights.into_iter().sum()
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Position {
    Empty,
    Round,
    Square,
}

impl From<char> for Position {
    fn from(value: char) -> Self {
        match value {
            '.' => Self::Empty,
            'O' => Self::Round,
            '#' => Self::Square,
            _ => panic!("unknown value"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let input = "\
            ...\n\
            ...\n\
            ...\n\
            ...\n\
            ...";
        let panel = Panel::from_str(input).unwrap();
        assert_eq!(panel.positions.len(), 5);
        assert_eq!(panel.positions[0].len(), 3);
    }

    #[test]
    fn test_single_line_without_squares() {
        let input = "\
            O\n\
            .\n\
            O\n\
            .\n\
            O\n\
            .";
        assert_eq!(run(input), 6 + 5 + 4);
    }

    #[test]
    fn test_example() {
        let input = "\
            O....#....\n\
            O.OO#....#\n\
            .....##...\n\
            OO.#O....O\n\
            .O.....O#.\n\
            O.#..O.#.#\n\
            ..O..#O..O\n\
            .......O..\n\
            #....###..\n\
            #OO..#....";
        assert_eq!(run(input), 136);
    }
}
