use std::iter;

pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub enum Entry {
    Number(u32),
    Gear,
    OtherSymbol,
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
            '*' => Self::Gear,
            _ => Entry::OtherSymbol,
        }
    }

    pub fn is_symbol(&self) -> bool {
        match self {
            Entry::Number(_) => false,
            Entry::Gear => true,
            Entry::OtherSymbol => true,
            Entry::Empty => false,
        }
    }

    pub fn is_number(&self) -> bool {
        matches!(self, Entry::Number(_))
    }

    pub fn is_gear(&self) -> bool {
        matches!(self, Entry::Gear)
    }
}

#[derive(Debug, Default)]
pub struct EntryNeighborhood {
    upper_left: Entry,
    upper_middle: Entry,
    upper_right: Entry,
    middle_left: Entry,
    middle_right: Entry,
    lower_left: Entry,
    lower_middle: Entry,
    lower_right: Entry,
}

impl EntryNeighborhood {
    pub fn contains_symbols(&self) -> bool {
        if self.upper_left.is_symbol() {
            return true;
        }
        if self.upper_middle.is_symbol() {
            return true;
        }
        if self.upper_right.is_symbol() {
            return true;
        }
        if self.middle_left.is_symbol() {
            return true;
        }
        if self.middle_right.is_symbol() {
            return true;
        }
        if self.lower_left.is_symbol() {
            return true;
        }
        if self.lower_middle.is_symbol() {
            return true;
        }
        if self.lower_right.is_symbol() {
            return true;
        }
        false
    }

    fn count_numbers_in_line(left: Entry, middle: Entry, right: Entry) -> u32 {
        if middle.is_number() {
            // can max be one number
            return 1;
        }

        let mut count = 0;
        if left.is_number() {
            count += 1;
        }
        if right.is_number() {
            count += 1;
        }
        count
    }

    pub fn count_numbers(&self) -> u32 {
        let mut count = 0;

        count += Self::count_numbers_in_line(self.upper_left, self.upper_middle, self.upper_right);
        if self.middle_left.is_number() {
            count += 1;
        }
        if self.middle_right.is_number() {
            count += 1;
        }
        count += Self::count_numbers_in_line(self.lower_left, self.lower_middle, self.lower_right);

        count
    }
}

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
        EntryNeighborhood {
            upper_left: self.entries[y][x],
            upper_middle: self.entries[y][x + 1],
            upper_right: self.entries[y][x + 2],
            middle_left: self.entries[y + 1][x],
            middle_right: self.entries[y + 1][x + 2],
            lower_left: self.entries[y + 2][x],
            lower_middle: self.entries[y + 2][x + 1],
            lower_right: self.entries[y + 2][x + 2],
        }
    }

    pub fn neighboring_gears(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let (len_x, len_y) = self.dimensions();
        let (max_x, max_y) = (len_x - 1, len_y - 1);
        let hood = self.entry_neighbors(x, y);
        let mut gears = Vec::new();

        if x > 0 && y > 0 && hood.upper_left.is_gear() {
            gears.push((x - 1, y - 1));
        }
        if y > 0 && hood.upper_middle.is_gear() {
            gears.push((x, y - 1));
        }
        if x < max_x && y > 0 && hood.upper_right.is_gear() {
            gears.push((x + 1, y - 1));
        }
        if x > 0 && hood.middle_left.is_gear() {
            gears.push((x - 1, y));
        }
        if x < max_x && hood.middle_right.is_gear() {
            gears.push((x + 1, y));
        }
        if x > 0 && y < max_y && hood.lower_left.is_gear() {
            gears.push((x - 1, y + 1));
        }
        if y < max_y && hood.lower_middle.is_gear() {
            gears.push((x, y + 1));
        }
        if x < max_x && y < max_y && hood.lower_right.is_gear() {
            gears.push((x + 1, y + 1));
        }

        gears
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

                if schematic.entry_neighbors(x, y).contains_symbols() {
                    collector.set_buffer_valid();
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
    fn test_neighborhood_nums() {
        let mut hood = EntryNeighborhood::default();
        hood.upper_middle = Entry::Number(2);
        hood.upper_right = Entry::Number(3);
        hood.lower_left = Entry::Number(4);
        hood.middle_left = Entry::Number(5);
        hood.middle_right = Entry::Number(6);
        assert_eq!(hood.count_numbers(), 4);
    }

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
    fn test_schematic_neighborhood_gears() {
        let input = "\
            ..*\n\
            *..\n\
            .**";
        let schematic = Schematic::parse(input);
        let mut gears = schematic.neighboring_gears(1, 1);
        assert_eq!(gears.pop().unwrap(), (2, 2));
        assert_eq!(gears.pop().unwrap(), (1, 2));
        assert_eq!(gears.pop().unwrap(), (0, 1));
        assert_eq!(gears.pop().unwrap(), (2, 0));
        assert!(gears.pop().is_none());

        let input = "**";
        let schematic = Schematic::parse(input);
        let mut gears = schematic.neighboring_gears(0, 0);
        assert_eq!(gears.pop().unwrap(), (1, 0));
        assert!(gears.pop().is_none());
    }

    #[test]
    fn test_schematic_parse() {
        let input = ".*.\n2+.";
        let schematic = Schematic::parse(input);
        assert_eq!(schematic.dimensions(), (3, 2));
        assert_eq!(schematic.entry(0, 0), Entry::Empty);
        assert_eq!(schematic.entry(1, 0), Entry::Gear);
        assert_eq!(schematic.entry(2, 0), Entry::Empty);
        assert_eq!(schematic.entry(0, 1), Entry::Number(2));
        assert_eq!(schematic.entry(1, 1), Entry::OtherSymbol);
        assert_eq!(schematic.entry(2, 1), Entry::Empty);
    }

    #[test]
    fn test_schematic_neighborhood() {
        let all_empty = "..\n..";
        let schematic = Schematic::parse(all_empty);
        let neighbors = schematic.entry_neighbors(1, 1);
        assert!(!neighbors.contains_symbols());

        let all_symbols = "=+#\n!§$\n%&/";
        let schematic = Schematic::parse(all_symbols);
        let neighbors = schematic.entry_neighbors(1, 1);
        assert!(neighbors.contains_symbols());
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
