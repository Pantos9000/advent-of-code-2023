pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn part1(input: &str) -> usize {
    0 // TODO
}

fn part2(input: &str) -> usize {
    0 // TODO
}

fn main() {
    let input = read_input();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Result1 is {result1}");
    println!("Result2 is {result2}");
}
