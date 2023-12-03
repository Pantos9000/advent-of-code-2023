#[derive(Default)]
struct Game {
    id: u32,
    rounds: Vec<Round>,
}

impl Game {
    fn parse_id(substr: &str) -> u32 {
        let (prefix, id) = substr.split_once(' ').unwrap();
        assert_eq!(prefix, "Game");
        id.parse().unwrap()
    }

    fn parse_rounds(substr: &str) -> Vec<Round> {
        substr.split(';').map(Round::parse).collect()
    }

    fn parse(line: &str) -> Self {
        let (game, rounds) = line.split_once(':').unwrap();

        Self {
            id: Self::parse_id(game),
            rounds: Self::parse_rounds(rounds),
        }
    }

    fn is_possible_with(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        for round in &self.rounds {
            if !round.is_possible_with(max_red, max_green, max_blue) {
                return false;
            }
        }
        true
    }
}

struct Round {
    red: u32,
    green: u32,
    blue: u32,
}

impl Round {
    fn parse(substr: &str) -> Self {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        substr.split(',').for_each(|x| {
            let (num, color) = x.trim().split_once(' ').unwrap();
            let num = num.parse().unwrap();
            match color {
                "red" => red = num,
                "green" => green = num,
                "blue" => blue = num,
                _ => unreachable!(),
            }
        });
        Self { red, green, blue }
    }

    fn is_possible_with(&self, max_red: u32, max_green: u32, max_blue: u32) -> bool {
        self.red <= max_red && self.green <= max_green && self.blue <= max_blue
    }
}

fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

fn main() {
    let input = read_input();
    let result: u32 = input
        .lines()
        .map(Game::parse)
        .filter_map(|game| {
            if !game.is_possible_with(12, 13, 14) {
                None
            } else {
                Some(game.id)
            }
        })
        .sum();
    println!("Result: {result}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse() {
        let line = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green";
        let Game { id, rounds } = Game::parse(line);
        assert_eq!(id, 1);
        assert_eq!(rounds.len(), 3);
        assert_eq!(rounds[0].red, 4);
        assert_eq!(rounds[0].green, 0);
        assert_eq!(rounds[0].blue, 3);
        assert_eq!(rounds[1].red, 1);
        assert_eq!(rounds[1].green, 2);
        assert_eq!(rounds[1].blue, 6);
        assert_eq!(rounds[2].red, 0);
        assert_eq!(rounds[2].green, 2);
        assert_eq!(rounds[2].blue, 0);
    }
}
