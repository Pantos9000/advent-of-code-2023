mod hamster;
mod map;

mod part1;
mod part2;

pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn main() {
    let input = read_input();
    let result1 = part1::run(&input);
    let result2 = part2::run(&input);
    println!("Result1 is {result1}");
    println!("Result2 is {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        2413432311323\n\
        3215453535623\n\
        3255245654254\n\
        3446585845452\n\
        4546657867536\n\
        1438598798454\n\
        4457876987766\n\
        3637877979653\n\
        4654967986887\n\
        4564679986453\n\
        1224686865563\n\
        2546548887735\n\
        4322674655533";

    #[test]
    fn test_part1_solution() {
        let input = read_input();
        assert_eq!(part1::run(&input), 1008);
    }

    #[test]
    fn test_example_part1() {
        assert_eq!(part1::run(EXAMPLE), 102);
    }

    #[test]
    fn test_example_part2() {
        assert_eq!(part2::run(EXAMPLE), 94);
    }
}
