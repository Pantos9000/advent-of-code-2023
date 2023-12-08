pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

#[derive(Debug)]
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
        let id = prefix
            .split(' ')
            .filter(|x| !x.is_empty())
            .nth(1)
            .unwrap()
            .parse()
            .unwrap();
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
        let num_matches = self.matches();
        if num_matches == 0 {
            return 0;
        }
        2u32.pow(num_matches - 1)
    }

    fn matches(&self) -> u32 {
        self.chosen_numbers
            .iter()
            .filter(|num| self.winning_numbers.contains(num))
            .map(|_| 1)
            .sum()
    }

    fn id(&self) -> u32 {
        self.id
    }

    fn is_copy(&self) -> bool {
        self.is_copy
    }
}

struct Pile {
    cards: Vec<Card>,
}

impl std::fmt::Display for Pile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Pile:")?;
        for card in self.cards.iter().rev() {
            let id = card.id();
            let copy = if card.is_copy() { "(copy)" } else { "" };
            writeln!(f, "  - Card {id} {copy}")?;
        }
        Ok(())
    }
}

impl Pile {
    fn new(input: &str) -> Self {
        let cards = input.lines().map(Card::parse).rev().collect();
        Self { cards }
    }

    fn pop(&mut self) -> Option<Card> {
        self.cards.pop()
    }

    fn process_card(&mut self, card: &Card) {
        let num_copied_cards = usize::try_from(card.matches()).unwrap();
        let copies: Vec<_> = self
            .cards
            .iter()
            .rev()
            .filter(|contained_card| !contained_card.is_copy())
            .filter(|contained_card| contained_card.id() > card.id())
            .take(num_copied_cards)
            .map(|card| card.clone())
            .collect();
        self.cards.extend(copies.into_iter().rev());
    }
}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .map(Card::parse)
        .map(|card| card.points())
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut pile = Pile::new(&input);
    let mut num_cards = 0;

    while let Some(card) = pile.pop() {
        num_cards += 1;
        pile.process_card(&card);
    }

    num_cards
}

fn main() {
    let input = read_input();
    let result1 = part1(&input);
    let result2 = part2(&input);
    println!("Result1 is {result1}");
    println!("Result2 is {result2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2_example() {
        let input = "\
            Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        let result = part2(input);
        assert_eq!(result, 30);
    }

    #[test]
    fn test_pile() {
        let input = "\
            Card 1: 1 2 | 1 2\n\
            Card 2: 3 4 | 3\n\
            Card 3: 5 6 | 6";
        let mut pile = Pile::new(input);
        let card1 = pile.pop().unwrap();
        assert_eq!(card1.id(), 1);
        assert!(!card1.is_copy());
        pile.process_card(&card1);
        drop(card1);

        // copy from Card 1
        let card2_copy = pile.pop().unwrap();
        assert_eq!(card2_copy.id(), 2);
        assert!(card2_copy.is_copy());
        pile.process_card(&card2_copy);
        drop(card2_copy);

        // Copy from Card 2 copy
        let card3_copy = pile.pop().unwrap();
        assert_eq!(card3_copy.id(), 3);
        assert!(card3_copy.is_copy());
        pile.process_card(&card3_copy);
        drop(card3_copy);

        // copy from card 1
        let card3_copy = pile.pop().unwrap();
        assert_eq!(card3_copy.id(), 3);
        assert!(card3_copy.is_copy());
        pile.process_card(&card3_copy);
        drop(card3_copy);

        // card 2
        let card2 = pile.pop().unwrap();
        assert_eq!(card2.id(), 2);
        assert!(!card2.is_copy());
        pile.process_card(&card2);
        drop(card2);

        // copy from card 2
        let card3_copy = pile.pop().unwrap();
        assert_eq!(card3_copy.id(), 3);
        assert!(card3_copy.is_copy());
        pile.process_card(&card3_copy);
        drop(card3_copy);

        // card 3
        let card3 = pile.pop().unwrap();
        assert_eq!(card3.id(), 3);
        assert!(!card3.is_copy());
        pile.process_card(&card3);
        drop(card3);

        assert!(pile.pop().is_none());
    }

    #[test]
    fn test_part1() {
        let card = Card::parse("Card 1: 1 2 3 4 | 2 3");
        assert_eq!(card.points(), 2);
        assert_eq!(card.id, 1);
        let card = Card::parse("Card 2: 1 2 3 4 | 1 2 3 4 5");
        assert_eq!(card.points(), 8);
        assert_eq!(card.id, 2);
        let card = Card::parse("Card  3: 1 2 3 4 | 1 2 3 4 5");
        assert_eq!(card.points(), 8);
        assert_eq!(card.id, 3);
    }
}
