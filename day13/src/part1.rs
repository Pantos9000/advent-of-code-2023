pub fn run(input: &str) -> usize {
    0 // TODO
}

struct Map {
    row_hashes: Vec<u32>,
    col_hashes: Vec<u32>,
}

impl Map {
    pub fn from_block(block: &str) -> Self {
        fn parse_line(line: &str) -> u32 {
            line.chars()
                .enumerate()
                .map(|(i, c)| parse_char(c) << i)
                .sum()
        }

        fn parse_char(c: char) -> u32 {
            match c {
                '.' => 0,
                '#' => 1,
                _ => panic!("unknown map char"),
            }
        }

        fn transpose_hashes(hashes: &[u32], len: usize) -> Vec<u32> {
            let mut ret = vec![0; len];
            for (i, hash) in hashes.iter().enumerate() {
                for (j, val) in ret.iter_mut().enumerate() {
                    *val += ((hash >> j) & 0b_1) << i;
                }
            }
            ret
        }

        let row_hashes: Vec<_> = block.lines().map(parse_line).collect();
        let col_len = block.lines().nth(0).unwrap().chars().count();
        let col_hashes = transpose_hashes(&row_hashes, col_len);

        Self {
            row_hashes,
            col_hashes,
        }
    }

    fn is_mirroring(hashes: &[u32], index: usize) -> bool {
        let first_side = hashes.iter().take(index + 1).rev();
        let second_side = hashes.iter().skip(index + 1);
        first_side
            .zip(second_side)
            .all(|(first, second)| first == second)
    }

    fn summarize_hashes(hashes: &[u32]) -> usize {
        hashes
            .iter()
            .take(hashes.len() - 1)
            .enumerate()
            .filter(|(i, _)| Self::is_mirroring(hashes, *i))
            .map(|(i, _)| i + 1)
            .sum()
    }

    pub fn summarize(&self) -> usize {
        let sum_rows = Self::summarize_hashes(&self.row_hashes);
        let sum_cols = Self::summarize_hashes(&self.col_hashes);
        100 * sum_rows + sum_cols
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_map_parse() {
        let block = "\
            ..\n\
            .#\n\
            #.";
        let map = Map::from_block(block);
        assert_eq!(map.row_hashes, vec![0, 2, 1]);
        assert_eq!(map.col_hashes, vec![4, 2]);
    }

    #[test]
    fn test_map_is_mirroring() {
        let hashes = [3, 2, 2, 3];
        assert!(!Map::is_mirroring(&hashes, 0));
        assert!(Map::is_mirroring(&hashes, 1));
    }

    #[test]
    fn test_example1() {
        let block = "\
            #.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.";
        let map = Map::from_block(block);
        assert_eq!(map.summarize(), 5);
    }

    #[test]
    fn test_example2() {
        let block = "\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";
        let map = Map::from_block(block);
        assert_eq!(map.summarize(), 400);
    }

    #[test]
    fn test_block_example() {
        let input = "\
            #.##..##.\n\
            ..#.##.#.\n\
            ##......#\n\
            ##......#\n\
            ..#.##.#.\n\
            ..##..##.\n\
            #.#.##.#.\n\
            \n\
            #...##..#\n\
            #....#..#\n\
            ..##..###\n\
            #####.##.\n\
            #####.##.\n\
            ..##..###\n\
            #....#..#";
        assert_eq!(run(input), 405);
    }
}
