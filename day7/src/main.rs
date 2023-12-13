pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn part1(input: &str) -> usize {
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}
impl HandType {
    fn new(cards: &[Card; 5]) -> Self {
        todo!()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
struct Card(usize);
impl From<char> for Card {
    fn from(value: char) -> Self {
        let int_repr = match value {
            '2' => 0,
            '3' => 1,
            '4' => 2,
            '5' => 3,
            '6' => 4,
            '7' => 5,
            '8' => 6,
            '9' => 7,
            'T' => 8,
            'J' => 9,
            'Q' => 10,
            'K' => 11,
            'A' => 12,
            _ => panic!("unknown card '{value}'"),
        };
        Self(int_repr)
    }
}

#[derive(Debug)]
struct Hand {
    cards: [Card; 5],
    highcard_value: usize,
    hand_type: HandType,
    bid: usize,
}
impl Hand {
    fn parse(input: &str) -> Self {
        todo!()
    }
}

fn main() {
    let input = read_input();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Result1 is {result1}");
    println!("Result2 is {result2}");
}
