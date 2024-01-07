use std::iter;
use std::str::FromStr;

pub fn run(input: &str) -> usize {
    input.lines().map(count_per_line_with_unfolding).sum()
}

fn count_per_line_with_unfolding(line: &str) -> usize {
    let mut record = Record::from_str(line).unwrap();
    record.unfold(5);
    record.count_possible_arrangements()
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Spring {
    Ok,
    Damaged,
    Unknown,
}

impl TryFrom<char> for Spring {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Ok),
            '#' => Ok(Self::Damaged),
            '?' => Ok(Self::Unknown),
            _ => Err(()),
        }
    }
}

impl std::fmt::Display for Spring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let c = match self {
            Spring::Ok => '.',
            Spring::Damaged => '#',
            Spring::Unknown => '?',
        };
        write!(f, "{c}")
    }
}

#[derive(Debug)]
struct Record {
    springs: Vec<Spring>,
    counts: Vec<usize>,
}

impl FromStr for Record {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let (springs, counts) = line.split_once(' ').unwrap();
        let springs = springs
            .chars()
            .map(|c| Spring::try_from(c).unwrap())
            .collect();
        let counts = counts.split(',').map(|x| x.parse().unwrap()).collect();
        Ok(Self { springs, counts })
    }
}

impl std::fmt::Display for Record {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use itertools::Itertools;

        for spring in &self.springs {
            write!(f, "{spring}")?;
        }
        write!(f, " ")?;
        write!(f, "{}", self.counts.iter().format(","))?;
        Ok(())
    }
}

impl Record {
    fn count_possible_arrangements(&self) -> usize {
        Self::possible_arrangements_inner(&self.springs, &self.counts)
    }

    fn unfold(&mut self, factor: usize) {
        self.springs = iter::repeat(iter::once(&Spring::Unknown).chain(self.springs.iter()))
            .take(factor)
            .flatten()
            .skip(1)
            .copied()
            .collect();

        self.counts = iter::repeat(self.counts.iter())
            .take(factor)
            .flatten()
            .copied()
            .collect();
    }

    /// invariant: spring position has to be at beginning of damaged block
    fn is_valid_arrangement(springs: &[Spring], counts: &[usize]) -> bool {
        // no more springs left to check
        if springs.is_empty() {
            // valid config: no counts and springs are left, so they match
            // invalid config: counts are still left, but no springs
            return counts.is_empty();
        }

        // get current count unless no more counts
        let Some(current_count) = counts.first().copied() else {
            // valid: Ok springs are valid as they don't add to count; Unknown springs must be Ok
            // invalid: if any spring is damaged, there should have been a count
            return !springs.iter().any(|spring| spring == &Spring::Damaged);
        };

        // invalid if current count is bigger than remaining springs
        if springs.len() < current_count {
            return false;
        }

        // invalid if next group does not contain current count of damaged springs
        if springs[0..current_count]
            .iter()
            .any(|spring| spring == &Spring::Ok)
        {
            return false;
        }

        // invalid if spring after next group is damaged (if it exists)
        if let Some(following_spring) = springs.get(current_count) {
            if following_spring == &Spring::Damaged {
                return false;
            }
        }

        true
    }

    // TODO memo
    fn possible_arrangements_inner(springs: &[Spring], counts: &[usize]) -> usize {
        if springs.is_empty() || counts.is_empty() {
            // we reached the end, so if this is a valid combination, return count of 1
            return if Self::is_valid_arrangement(springs, counts) {
                1
            } else {
                0
            };
        }

        // arrangement is not yet finished, so these have to exist
        let current_spring = springs[0];
        let current_count = counts[0];

        let mut result = 0;

        // assume spring is ok
        if current_spring == Spring::Ok || current_spring == Spring::Unknown {
            let next_springs = &springs[1..];
            result += Self::possible_arrangements_inner(next_springs, counts);
        }

        // assume spring is damaged
        if current_spring == Spring::Damaged || current_spring == Spring::Unknown {
            // invariant is valid, as next spring is damaged
            if Self::is_valid_arrangement(springs, counts) {
                // We checked for valid config, so the damaged springs have to exist. We also skip the
                // mandatory Ok spring after the damaged ones, unless we are at the end of the record.
                let springs_to_skip = current_count + 1;
                let next_springs = springs
                    .get(springs_to_skip..)
                    .unwrap_or(springs.get(current_count..).unwrap());
                let next_counts = &counts[1..];
                result += Self::possible_arrangements_inner(next_springs, next_counts);
            }
        }

        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfolding() {
        let line = "??.## 1,2";

        let mut record = Record::from_str(line).unwrap();

        record.unfold(1);
        assert_eq!(format!("{record}"), line);

        record.unfold(2);
        assert_eq!(format!("{record}"), "??.##???.## 1,2,1,2");
    }

    #[test]
    fn test_inner_count_arrangements_accepts_empty() {
        let springs = Vec::new();
        let counts = Vec::new();
        assert_eq!(Record::possible_arrangements_inner(&springs, &counts), 1);
    }

    #[test]
    fn test_inner_count_arrangements_counts_one() {
        let springs = vec![Spring::Ok, Spring::Damaged, Spring::Ok];
        let counts = vec![1];
        assert_eq!(Record::possible_arrangements_inner(&springs, &counts), 1);
    }

    #[test]
    fn test_example_without_unfolding() {
        let line = "???.### 1,1,3";
        let record = Record::from_str(line).unwrap();
        assert_eq!(record.count_possible_arrangements(), 1);
    }

    #[test]
    fn test_part2_example1() {
        let line = "???.### 1,1,3";
        assert_eq!(count_per_line_with_unfolding(line), 1);
    }

    #[test]
    fn test_part2_example2() {
        let line = ".??..??...?##. 1,1,3";
        assert_eq!(count_per_line_with_unfolding(line), 16384);
    }

    #[test]
    fn test_part2_example3() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(count_per_line_with_unfolding(line), 1);
    }

    #[test]
    fn test_part2_example4() {
        let line = "????.#...#... 4,1,1";
        assert_eq!(count_per_line_with_unfolding(line), 16);
    }

    #[test]
    fn test_part2_example5() {
        let line = "????.######..#####. 1,6,5";
        assert_eq!(count_per_line_with_unfolding(line), 2500);
    }

    #[test]
    fn test_part2_example6() {
        let line = "?###???????? 3,2,1";
        assert_eq!(count_per_line_with_unfolding(line), 506250);
    }
}
