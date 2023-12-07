pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

struct Card {
    id: u32,
    is_copy: bool,
    winning_numbers: Vec<u32>,
    chosen_numbers: Vec<u32>,
}

impl Clone for Card {
    fn clone(&self) -> Self {
        Card {
            id: self.id,
            is_copy: true,
            winning_numbers: self.winning_numbers.clone(),
            chosen_numbers: self.chosen_numbers.clone(),
        }
    }
}

impl Card {
    fn parse(line: &str) -> Self {
        let (prefix, numbers) = line.split_once(':').unwrap();
        let id = prefix.split(' ').nth(1).unwrap().parse().unwrap();
        let (winning_numbers, chosen_numbers) = numbers.split_once('|').unwrap();

        Self {
            id,
            is_copy: false,
            winning_numbers: Self::parse_num_array(winning_numbers),
            chosen_numbers: Self::parse_num_array(chosen_numbers),
        }
    }

    fn parse_num_array(s: &str) -> Vec<u32> {
        s.split(' ')
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect()
    }

    fn points(&self) -> u32 {
        let mut points = 0;

        self.chosen_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .for_each(|_| {
                points = if points == 0 { 1 } else { points * 2 };
            });

        points
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Card::parse)
        .map(|card| card.points())
        .sum()
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
    fn test_part1() {
        let card = Card::parse("Card 1: 1 2 3 4 | 2 3");
        assert_eq!(card.points(), 2);
        assert_eq!(card.id, 1);
        let card = Card::parse("Card 2: 1 2 3 4 | 1 2 3 4 5");
        assert_eq!(card.points(), 8);
        assert_eq!(card.id, 2);
    }
}
