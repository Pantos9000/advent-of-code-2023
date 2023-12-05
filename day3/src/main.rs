use std::iter;

pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Entry {
    Number(u32),
    Symbol,
    #[default]
    Empty,
}

impl Entry {
    fn parse(c: char) -> Self {
        if let Some(number) = c.to_digit(10) {
            return Self::Number(number);
        }
        match c {
            '.' => Self::Empty,
            '\n' => Self::Empty,
            _ => Entry::Symbol,
        }
    }
}

type EntryNeighborhood = [Entry; 8];

pub struct Schematic {
    entries: Vec<Vec<Entry>>,
}

impl Schematic {
    pub fn parse(input: &str) -> Self {
        fn parse_line(line: &str) -> Vec<Entry> {
            let prepend = iter::once(Entry::default());
            let append = prepend.clone();
            let parse_iter = line.chars().map(Entry::parse);
            prepend.chain(parse_iter).chain(append).collect()
        }

        let line_length = input.find('\n').unwrap_or(input.len()) + 2;
        let prepend = iter::once(vec![Entry::default(); line_length]);
        let append = prepend.clone();
        let entries_iter = input
            .split('\n')
            .filter(|line| !line.is_empty())
            .map(parse_line);

        let entries: Vec<_> = prepend.chain(entries_iter).chain(append).collect();
        Self { entries }
    }

    pub fn entry(&self, x: usize, y: usize) -> Entry {
        self.entries[y + 1][x + 1]
    }

    /// get the schematic's x/y dimensions
    pub fn dimensions(&self) -> (usize, usize) {
        let y = self.entries.len() - 2;
        let x = self.entries[0].len() - 2;
        (x, y)
    }

    pub fn entry_neighbors(&self, x: usize, y: usize) -> EntryNeighborhood {
        [
            self.entries[y + 0][x + 0],
            self.entries[y + 0][x + 1],
            self.entries[y + 0][x + 2],
            self.entries[y + 1][x + 0],
            self.entries[y + 1][x + 2],
            self.entries[y + 2][x + 0],
            self.entries[y + 2][x + 1],
            self.entries[y + 2][x + 2],
        ]
    }
}

#[derive(Debug, Default)]
struct NumCollector {
    sum: u32,
    buffer: Option<u32>,
    buffer_valid: bool,
}

impl NumCollector {
    fn shift_into_buffer(&mut self, num: u32) {
        self.buffer = match self.buffer {
            None => Some(num),
            Some(buffer) => Some(buffer * 10 + num),
        }
    }

    fn flush_buffer(&mut self) {
        if let Some(buffer) = self.buffer {
            if self.buffer_valid {
                self.sum += buffer;
            }
        };

        self.buffer = None;
        self.buffer_valid = false;
    }

    fn set_buffer_valid(&mut self) {
        self.buffer_valid = true;
    }

    fn sum(&self) -> u32 {
        self.sum
    }
}

fn part1(input: &str) -> u32 {
    let schematic = Schematic::parse(input);
    let mut collector = NumCollector::default();

    let (len_x, len_y) = schematic.dimensions();
    for y in 0..len_y {
        for x in 0..len_x {
            if let Entry::Number(num) = schematic.entry(x, y) {
                collector.shift_into_buffer(num);

                for neighbor in schematic.entry_neighbors(x, y) {
                    if neighbor == Entry::Symbol {
                        collector.set_buffer_valid();
                    }
                }
            } else {
                collector.flush_buffer();
            }
        }
        // at the end of a line, also flush buffer
        collector.flush_buffer()
    }

    collector.sum()
}

fn main() {
    let input = read_input();
    let result1 = part1(&input);
    println!("Result1 is {result1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1_solution() {
        let input = read_input();
        let result = part1(&input);
        assert_eq!(result, 539590);
    }

    #[test]
    fn test_small_solution() {
        let input = "\
        .11..42+\n\
        1..*....\n\
        ......$2\n\
        12345...";
        let sum = part1(input);
        assert_eq!(sum, 55);
    }

    #[test]
    fn test_schematic_parse() {
        let input = ".*.\n23.";
        let schematic = Schematic::parse(input);
        assert_eq!(schematic.dimensions(), (3, 2));
        assert_eq!(schematic.entry(0, 0), Entry::Empty);
        assert_eq!(schematic.entry(1, 0), Entry::Symbol);
        assert_eq!(schematic.entry(2, 0), Entry::Empty);
        assert_eq!(schematic.entry(0, 1), Entry::Number(2));
        assert_eq!(schematic.entry(1, 1), Entry::Number(3));
        assert_eq!(schematic.entry(2, 1), Entry::Empty);
    }

    #[test]
    fn test_schematic_neighborhood() {
        let all_empty = "..\n..";
        let schematic = Schematic::parse(all_empty);
        let neighbors = schematic.entry_neighbors(1, 1);
        for neighbor in neighbors {
            assert!(matches!(neighbor, Entry::Empty));
        }

        let all_symbols = "*+#\n!§$\n%&/";
        let schematic = Schematic::parse(all_symbols);
        let neighbors = schematic.entry_neighbors(1, 1);
        for neighbor in neighbors {
            assert!(matches!(neighbor, Entry::Symbol));
        }
    }

    #[test]
    fn test_collector() {
        let mut collector = NumCollector::default();
        assert_eq!(collector.sum(), 0);
        collector.flush_buffer();
        assert_eq!(collector.sum(), 0);
        collector.shift_into_buffer(4);
        collector.shift_into_buffer(2);
        collector.flush_buffer();
        assert_eq!(collector.sum(), 0);
        collector.shift_into_buffer(1);
        collector.shift_into_buffer(3);
        collector.set_buffer_valid();
        collector.shift_into_buffer(3);
        collector.shift_into_buffer(7);
        collector.flush_buffer();
        assert_eq!(collector.sum(), 1337);
        collector.shift_into_buffer(4);
        collector.set_buffer_valid();
        collector.shift_into_buffer(2);
        collector.flush_buffer();
        assert_eq!(collector.sum(), 1337 + 42);
        collector.set_buffer_valid();
        collector.flush_buffer();
        collector.shift_into_buffer(1);
        collector.flush_buffer();
        assert_eq!(collector.sum(), 1337 + 42);
    }
}
