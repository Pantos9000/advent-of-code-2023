pub fn run(input: &str) -> usize {
    0 // TODO
}

struct Map {
    row_hashes: Vec<u32>,
    col_hashes: Vec<u32>,
}

impl Map {
    fn from_block(block: &str) -> Self {
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
}
