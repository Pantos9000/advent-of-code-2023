use crate::part1;

pub fn run(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse().unwrap()))
        .map(calc_prev)
        .sum()
}

fn calc_prev(history: impl Iterator<Item = isize>) -> isize {
    part1::calc(history, part1::CalcWhat::Prev)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45";
        assert_eq!(run(input), 2);
    }
}
