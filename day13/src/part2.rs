pub fn run(input: &str) -> usize {
    input
        .split("\n\n")
        .map(Map::from_block)
        .collect::<Vec<_>>()
        .iter()
        .map(|map| map.desmudge())
        .map(|map| map.summarize())
        .sum()
}

#[derive(Debug, Clone)]
pub struct Map {
    row_hashes: Vec<u32>,
    col_hashes: Vec<u32>,
    ignored_row_mirror: Option<usize>,
    ignored_col_mirror: Option<usize>,
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
            ignored_row_mirror: None,
            ignored_col_mirror: None,
        }
    }

    fn flip(&mut self, row: usize, col: usize) {
        let row_hash = self.row_hashes.get_mut(row).unwrap();
        let col_hash = self.col_hashes.get_mut(col).unwrap();
        let row_mask = 1 << col;
        let col_mask = 1 << row;
        *row_hash ^= row_mask;
        *col_hash ^= col_mask;
    }

    fn flipped_iter(&self) -> impl Iterator<Item = Self> + '_ {
        let rows = 0..self.row_hashes.len();
        let cols = 0..self.col_hashes.len();
        let row_col_combinations =
            rows.flat_map(move |row| cols.clone().map(move |col| (row, col)));
        row_col_combinations.map(|(row, col)| {
            let mut clone = self.clone();
            clone.flip(row, col);
            clone
        })
    }

    fn desmudge(&self) -> Self {
        let orig_row_index = Self::mirroring_iter(&self.row_hashes).next();
        let orig_col_index = Self::mirroring_iter(&self.col_hashes).next();
        let mut map = self
            .flipped_iter()
            .find(|map| {
                let row = Self::find_new_mirror_index(&map.row_hashes, orig_row_index);
                let col = Self::find_new_mirror_index(&map.col_hashes, orig_col_index);
                row.xor(col).is_some()
            })
            .unwrap();
        map.ignored_row_mirror = orig_row_index;
        map.ignored_col_mirror = orig_col_index;
        map
    }

    fn is_mirroring(hashes: &[u32], index: usize) -> bool {
        let first_side = hashes.iter().take(index + 1).rev();
        let second_side = hashes.iter().skip(index + 1);
        first_side
            .zip(second_side)
            .all(|(first, second)| first == second)
    }

    fn mirroring_iter(hashes: &[u32]) -> impl Iterator<Item = usize> + '_ {
        hashes
            .iter()
            .take(hashes.len() - 1)
            .enumerate()
            .map(|(i, _)| i)
            .filter(|i| Self::is_mirroring(hashes, *i))
    }

    fn find_new_mirror_index(hashes: &[u32], orig_index: Option<usize>) -> Option<usize> {
        Self::mirroring_iter(hashes).find(|&index| orig_index != Some(index))
    }

    pub fn summarize(&self) -> usize {
        let row = Self::find_new_mirror_index(&self.row_hashes, self.ignored_row_mirror)
            .map(|x| x + 1)
            .unwrap_or(0);
        let col = Self::find_new_mirror_index(&self.col_hashes, self.ignored_col_mirror)
            .map(|x| x + 1)
            .unwrap_or(0);

        100 * row + col
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
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
        assert_eq!(run(input), 400);
    }
}
