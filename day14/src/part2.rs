use std::{collections::VecDeque, str::FromStr};

pub fn run(input: &str) -> usize {
    let mut panel = Panel::from_str(input).unwrap();
    panel.spin(1000000000);
    panel.calc_load()
}

#[derive(Clone, PartialEq)]
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

impl std::fmt::Debug for Panel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "Panel {}x{}",
            self.positions[0].len(),
            self.positions.len()
        )?;
        for line in &self.positions {
            for position in line {
                write!(f, "{position:?}")?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Panel {
    pub fn calc_load(&self) -> usize {
        let row_len = self.positions[0].len();
        let col_len = self.positions.len();
        let mut weights = vec![0; row_len];

        for (row, row_data) in self.positions.iter().enumerate() {
            for (col, pos) in row_data.iter().enumerate() {
                if pos == &Position::Round {
                    let pos_weight = col_len - row;
                    weights[col] += pos_weight;
                }
            }
        }

        weights.into_iter().sum()
    }

    pub fn spin(&mut self, cycles: usize) {
        if cycles == 0 {
            return;
        }

        let mut old_hashes = Vec::new();

        let cycle = 'find_cycle_length: {
            for _ in 0..cycles {
                self.spin_once();

                let new_hash = self.hash();
                if let Some(hash_index) = old_hashes.iter().position(|&x| x == new_hash) {
                    let cycle_start = hash_index;
                    let cycle_length = old_hashes.len() - hash_index;
                    break 'find_cycle_length Some((cycle_start, cycle_length));
                }
                old_hashes.push(new_hash);
            }
            None
        };

        let Some((cycle_start, cycle_length)) = cycle else {
            return;
        };

        println!("cycle of length {cycle_length} starting at iteration {cycle_start} detected, taking shortcut");

        // cycle was detected, maybe some spins are still missing
        assert_ne!(cycle_length, 0);
        let missing_cycles = (cycles - cycle_start - 1) % cycle_length;

        for _ in 0..missing_cycles {
            self.spin_once();
        }
    }

    fn hash(&self) -> usize {
        let row_len = self.positions[0].len();
        let mut hash = 0;
        for (row, row_data) in self.positions.iter().enumerate() {
            for (col, pos) in row_data.iter().enumerate() {
                match pos {
                    Position::Empty => (), // empty will be counted as 0
                    Position::Round => hash += row * row_len + col,
                    Position::Square => (), // squares never change, so don't add them to the hash
                }
            }
        }

        hash
    }

    fn spin_once(&mut self) {
        self.tilt_north();
        self.tilt_west();
        self.tilt_south();
        self.tilt_east();
    }

    fn tilt_north(&mut self) {
        let row_len = self.positions[0].len();
        let mut sliders = vec![Slider::default(); row_len];

        for row_data in self.positions.iter_mut() {
            for (col, pos) in row_data.iter_mut().enumerate() {
                sliders[col].process_next_pos(pos);
            }
        }
    }

    fn tilt_west(&mut self) {
        let mut slider = Slider::default();

        for row_data in self.positions.iter_mut() {
            for pos in row_data.iter_mut() {
                slider.process_next_pos(pos);
            }
            slider.reset();
        }
    }

    fn tilt_south(&mut self) {
        let row_len = self.positions[0].len();
        let mut sliders = vec![Slider::default(); row_len];

        for row_data in self.positions.iter_mut().rev() {
            for (col, pos) in row_data.iter_mut().enumerate() {
                sliders[col].process_next_pos(pos);
            }
        }
    }

    fn tilt_east(&mut self) {
        let mut slider = Slider::default();
        for row_data in self.positions.iter_mut() {
            for pos in row_data.iter_mut().rev() {
                slider.process_next_pos(pos);
            }
            slider.reset();
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
pub enum Position {
    Empty,
    Round,
    Square,
}

impl std::fmt::Debug for Position {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let char = match self {
            Position::Empty => '.',
            Position::Round => 'O',
            Position::Square => '#',
        };
        write!(f, "{char}")
    }
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

#[derive(Default)]
struct Slider<'a> {
    slide_to: VecDeque<&'a mut Position>,
}

impl<'a> Slider<'a> {
    fn process_next_pos(&mut self, pos: &'a mut Position) {
        match pos {
            Position::Empty => self.slide_sink(pos),
            Position::Round => self.slide(pos),
            Position::Square => self.reset(),
        }
    }

    /// set a new pos to slide to, if not already set
    fn slide_sink(&mut self, to: &'a mut Position) {
        self.slide_to.push_front(to);
    }

    /// slide a round rock to the saved pos, if set
    fn slide(&mut self, slide_from: &'a mut Position) {
        let Some(slide_to) = self.slide_to.pop_back() else {
            return;
        };

        *slide_to = Position::Round;
        *slide_from = Position::Empty;
        self.slide_sink(slide_from);
    }

    /// unset the saved pos to slide to
    fn reset(&mut self) {
        self.slide_to.clear();
    }
}

impl Clone for Slider<'_> {
    fn clone(&self) -> Self {
        if !self.slide_to.is_empty() {
            panic!("can't clone slider that already holds state!");
        }
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calc_load() {
        let input = "\
            OOOO.#.O..\n\
            OO..#....#\n\
            OO..O##..O\n\
            O..#.OO...\n\
            ........#.\n\
            ..#....#.#\n\
            ..O..#.O.O\n\
            ..O.......\n\
            #....###..\n\
            #....#....";
        let panel = Panel::from_str(input).unwrap();
        assert_eq!(panel.calc_load(), 136);
    }

    #[test]
    fn test_tilt_west() {
        let input = ".#.O.O.";
        let mut panel = Panel::from_str(input).unwrap();
        panel.tilt_west();

        let expected = Panel::from_str(".#OO...").unwrap();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_tilt_east() {
        let input = ".O.O.#.";
        let mut panel = Panel::from_str(input).unwrap();
        panel.tilt_east();

        let expected = Panel::from_str("...OO#.").unwrap();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_tilt_north() {
        let input = "\
            .\n\
            #\n\
            .\n\
            O\n\
            .\n\
            O\n\
            .";
        let mut panel = Panel::from_str(input).unwrap();
        panel.tilt_north();

        let expected = "\
            .\n\
            #\n\
            O\n\
            O\n\
            .\n\
            .\n\
            .";
        let expected = Panel::from_str(expected).unwrap();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_tilt_south() {
        let input = "\
            .\n\
            #\n\
            .\n\
            O\n\
            .\n\
            O\n\
            .";
        let mut panel = Panel::from_str(input).unwrap();
        panel.tilt_south();

        let expected = "\
            .\n\
            #\n\
            .\n\
            .\n\
            .\n\
            O\n\
            O";
        let expected = Panel::from_str(expected).unwrap();
        assert_eq!(panel, expected);
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
        assert_eq!(run(input), 3 + 2 + 1);
    }

    #[test]
    fn test_example_one_detailed_spins() {
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
        let mut panel = Panel::from_str(input).unwrap();

        let after_tilt_north = "\
            OOOO.#.O..\n\
            OO..#....#\n\
            OO..O##..O\n\
            O..#.OO...\n\
            ........#.\n\
            ..#....#.#\n\
            ..O..#.O.O\n\
            ..O.......\n\
            #....###..\n\
            #....#....";
        let after_tilt_north = Panel::from_str(after_tilt_north).unwrap();

        panel.tilt_north();
        assert_eq!(panel, after_tilt_north);

        let after_tilt_west = "\
            OOOO.#O...\n\
            OO..#....#\n\
            OOO..##O..\n\
            O..#OO....\n\
            ........#.\n\
            ..#....#.#\n\
            O....#OO..\n\
            O.........\n\
            #....###..\n\
            #....#....";
        let after_tilt_west = Panel::from_str(after_tilt_west).unwrap();

        panel.tilt_west();
        assert_eq!(panel, after_tilt_west);

        let after_tilt_south = "\
            .....#....\n\
            ....#.O..#\n\
            O..O.##...\n\
            O.O#......\n\
            O.O....O#.\n\
            O.#..O.#.#\n\
            O....#....\n\
            OO....OO..\n\
            #O...###..\n\
            #O..O#....";
        let after_tilt_south = Panel::from_str(after_tilt_south).unwrap();

        panel.tilt_south();
        assert_eq!(panel, after_tilt_south);

        let after_tilt_east = "\
            .....#....\n\
            ....#...O#\n\
            ...OO##...\n\
            .OO#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#....\n\
            ......OOOO\n\
            #...O###..\n\
            #..OO#....";
        let after_tilt_east = Panel::from_str(after_tilt_east).unwrap();

        panel.tilt_east();
        assert_eq!(panel, after_tilt_east);
    }

    #[test]
    fn test_example_after_two_spins() {
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
        let mut panel = Panel::from_str(input).unwrap();

        let expected = "\
            .....#....\n\
            ....#...O#\n\
            .....##...\n\
            ..O#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#...O\n\
            .......OOO\n\
            #..OO###..\n\
            #.OOO#...O";
        let expected = Panel::from_str(expected).unwrap();

        panel.spin_once();
        panel.spin_once();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_example_after_three_spins() {
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
        let mut panel = Panel::from_str(input).unwrap();

        let expected = "\
            .....#....\n\
            ....#...O#\n\
            .....##...\n\
            ..O#......\n\
            .....OOO#.\n\
            .O#...O#.#\n\
            ....O#...O\n\
            .......OOO\n\
            #...O###.O\n\
            #.OOO#...O";
        let expected = Panel::from_str(expected).unwrap();

        panel.spin_once();
        panel.spin_once();
        panel.spin_once();
        assert_eq!(panel, expected);
    }

    #[test]
    fn test_example_full() {
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
        assert_eq!(run(input), 64);
    }
}
