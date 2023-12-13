pub fn part1(input: &str) -> usize {
    let mut hands: Vec<Hand> = input.lines().map(Hand::parse).collect();
    hands.sort_unstable();
    hands
        .iter()
        .enumerate()
        .map(|(index, hand)| hand.winning(index + 1))
        .sum()
}

#[derive(Debug, Clone, Copy)]
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
    fn from_cards(cards: &[Card; 5]) -> Self {
        let mut counts = [0_usize; NUM_DIFFERENT_CARDS];
        for card in cards {
            let index = card.0;
            counts[index] += 1;
        }

        let mut largest_count = 0;
        let mut second_largest_count = 0;
        for count in counts {
            if count > largest_count {
                second_largest_count = largest_count;
                largest_count = count;
            } else if count > second_largest_count {
                second_largest_count = count;
            }
        }

        match (largest_count, second_largest_count) {
            (5, 0) => Self::FiveOfAKind,
            (4, 1) => Self::FourOfAKind,
            (3, 2) => Self::FullHouse,
            (3, 1) => Self::ThreeOfAKind,
            (2, 2) => Self::TwoPair,
            (2, 1) => Self::OnePair,
            (1, 1) => Self::HighCard,
            _ => panic!("unexpected largest counts: {largest_count}, {second_largest_count}"),
        }
    }
}

#[derive(Debug, Clone, Copy)]
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
const NUM_DIFFERENT_CARDS: usize = 13;

struct Cards([Card; 5]);
impl Cards {
    fn parse(cards_str: &str) -> Self {
        let mut chars = cards_str.chars();
        let cards = [(); 5].map(|_| Card::from(chars.next().unwrap()));
        Self(cards)
    }
    fn hand_type(&self) -> HandType {
        HandType::from_cards(&self.0)
    }
    fn highcard_ranking(&self) -> usize {
        self.0
            .iter()
            .enumerate()
            .map(|(index, card)| card.0 * NUM_DIFFERENT_CARDS.pow(4 - index as u32))
            .sum()
    }
    fn type_ranking(&self) -> usize {
        let immediate_rank = match self.hand_type() {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        };
        immediate_rank * 1000000 // bigger than highest highcard ranking
    }
    fn ranking(&self) -> usize {
        self.type_ranking() + self.highcard_ranking()
    }
}

#[derive(Debug)]
struct Hand {
    ranking: usize,
    bid: usize,
}
impl Hand {
    fn parse(line: &str) -> Self {
        let (cards, bid) = line.split_once(' ').unwrap();
        let cards = Cards::parse(cards);
        let ranking = cards.ranking();
        let bid = bid.parse().unwrap();

        Self { ranking, bid }
    }
    fn winning(&self, rank: usize) -> usize {
        self.bid * rank
    }
}
impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.ranking.eq(&other.ranking)
    }
}
impl Eq for Hand {}
impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.ranking.partial_cmp(&other.ranking)
    }
}
impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.ranking.cmp(&other.ranking)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type() {
        let cards = Cards::parse("QTQTQ 42");
        assert!(matches!(cards.hand_type(), HandType::FullHouse));
    }

    #[test]
    fn test_cards_highcard_ranking() {
        let cards = Cards::parse("QTQTQ 42");
        let other_cards = Cards::parse("TQQTQ 42");
        assert!(cards.highcard_ranking() > other_cards.highcard_ranking())
    }

    #[test]
    fn test_example_part1() {
        let input = "\
            32T3K 765\n\
            T55J5 684\n\
            KK677 28\n\
            KTJJT 220\n\
            QQQJA 483";
        let result = part1(input);
        assert_eq!(result, 6440);
    }
}
