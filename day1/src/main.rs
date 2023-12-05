fn part1_get_first_digit(line: &str, from_right: bool) -> Option<u32> {
    let mut num_iter = line.chars().filter(|c| c.is_digit(10));
    let num = if from_right {
        num_iter.rev().next()?
    } else {
        num_iter.next()?
    };
    num.to_digit(10)
}

fn part1_get_num_from_line(line: &str) -> Option<u32> {
    let num_left = part1_get_first_digit(line, false)?;
    let num_right = part1_get_first_digit(line, true)?;

    let num = 10 * num_left + num_right;
    Some(num)
}

fn substring_to_num(substr: &str) -> Option<u32> {
    let values = [
        (0, "0", "zero"),
        (1, "1", "one"),
        (2, "2", "two"),
        (3, "3", "three"),
        (4, "4", "four"),
        (5, "5", "five"),
        (6, "6", "six"),
        (7, "7", "seven"),
        (8, "8", "eight"),
        (9, "9", "nine"),
    ];

    for (result, num_repr, str_repr) in values {
        if substr.starts_with(num_repr) || substr.starts_with(str_repr) {
            return Some(result);
        }
    }

    None
}

fn get_first_digit(line: &str, from_right: bool) -> Option<u32> {
    let mut num_iter = line.char_indices().filter_map(|(index, _)| {
        let substr = &line[index..];
        substring_to_num(substr)
    });

    if from_right {
        num_iter.rev().next()
    } else {
        num_iter.next()
    }
}

fn get_num_from_line(line: &str) -> Option<u32> {
    let num_left = get_first_digit(line, false)?;
    let num_right = get_first_digit(line, true)?;

    let num = 10 * num_left + num_right;
    Some(num)
}

fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn part1(input: &str) -> u32 {
    input.lines().filter_map(part1_get_num_from_line).sum()
}

fn part2(input: &str) -> u32 {
    input.lines().filter_map(get_num_from_line).sum()
}

fn main() {
    let input = read_input();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Part1 result is {result1}");
    println!("Part2 result is {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_first_digit_with_none() {
        let x = "asd";
        assert!(get_first_digit(x, false).is_none());
        assert!(get_first_digit(x, true).is_none());
    }

    #[test]
    fn test_get_first_digit_with_num_repr() {
        let x = "asd1zero2asd";
        assert_eq!(get_first_digit(x, false), Some(1));
        assert_eq!(get_first_digit(x, true), Some(2));
    }

    #[test]
    fn test_get_first_digit_with_str_repr() {
        let x = "asdone0twoasd";
        assert_eq!(get_first_digit(x, false), Some(1));
        assert_eq!(get_first_digit(x, true), Some(2));
    }
}
