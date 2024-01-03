use std::cmp;
use std::str::FromStr;

pub fn run(input: &str) -> usize {
    input.lines().map(count_possible_arrangements).sum()
}

pub fn count_possible_arrangements(line: &str) -> usize {
    let springs = BitSprings::from_str(line).unwrap();
    let group_springs = GroupSprings::from_str(line).unwrap();

    let mut num = 0;
    let mut arrangements = vec![springs];

    while let Some(springs) = arrangements.pop() {
        let Some((a, b)) = springs.collapse_next() else {
            num += 1;
            continue;
        };
        if group_springs.validate(&a).is_ok() {
            arrangements.push(a);
        }
        if group_springs.validate(&b).is_ok() {
            arrangements.push(b);
        }
    }

    num
}

#[derive(Clone)]
pub struct GroupSprings {
    groups: Vec<usize>,
}

impl std::fmt::Debug for GroupSprings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .groups
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");
        write!(f, "{s}")
    }
}

impl FromStr for GroupSprings {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let groups = line
            .split_once(' ')
            .ok_or(())?
            .1
            .split(',')
            .map(|x| x.parse().unwrap())
            .collect();
        Ok(Self { groups })
    }
}

impl GroupSprings {
    pub fn unfold(&mut self, factor: usize) {
        self.groups = std::iter::repeat(self.groups.iter())
            .take(factor)
            .flatten()
            .map(|&x| x)
            .collect();
    }

    pub fn validate(&self, springs: &BitSprings) -> Result<(), ()> {
        struct Checker {
            buf: u128,
            num_remaining: usize,
            num_ignored: usize,
        }
        impl Checker {
            fn new(springs: &BitSprings) -> Self {
                let mut num_remaining = cmp::min(
                    springs.num_springs,
                    springs.unknown_mask.trailing_zeros() as usize,
                );
                let num_missing = springs.num_springs - num_remaining;

                // mask away unknown springs
                let known_mask = (1 << num_remaining) - 1;
                let mut buf = springs.broken_mask & known_mask;

                // add 1 working spring at the beginning, to not trigger the good spring check
                buf <<= 1;
                num_remaining += 1;

                Self {
                    buf,
                    num_remaining,
                    num_ignored: num_missing,
                }
            }

            fn shift_out(&mut self, num: usize) {
                assert!(self.num_remaining >= num);
                self.num_remaining -= num;
                self.buf >>= num;
            }

            fn check_good_before_group(&mut self) -> Result<(), ()> {
                let num_good_springs =
                    cmp::min(self.num_remaining, self.buf.trailing_zeros() as usize);

                // before each group, there has to be at least one good spring. because of
                // the constructor, this is also the case for the first group.
                if num_good_springs == 0 {
                    return Err(());
                }

                // now shift out the good springs
                self.shift_out(num_good_springs);

                return Ok(());
            }

            fn check_only_good_remaining(&self) -> Result<(), ()> {
                if self.buf != 0 {
                    Err(())
                } else {
                    Ok(())
                }
            }

            fn check_group(&mut self, group: usize) -> Result<(), ()> {
                // min with remaining not needed, as rest is 0 and won't be counted
                let num_bad_springs = self.buf.trailing_ones() as usize;

                if num_bad_springs == group {
                    self.shift_out(num_bad_springs);
                    return Ok(());
                }

                // if group fits into remaining, then the bits are wrong
                if self.num_remaining >= group {
                    return Err(());
                }

                // if group does not fit into the rest (including ignored), the bits are wrong
                if group > self.num_remaining + self.num_ignored {
                    return Err(());
                }

                // if group just does not fit into the remaining ones, but there would still be
                // enough that are ignored, we are just not finished yet
                self.num_remaining = 0;
                Ok(())
            }

            fn none_remaining(&self) -> bool {
                self.num_remaining == 0
            }

            fn none_ignored(&self) -> bool {
                self.num_ignored == 0
            }
        }

        let mut checker = Checker::new(springs);
        let mut groups = self.groups.iter();

        if checker.none_remaining() {
            // there is nothing to check yet
            return Ok(());
        }

        'checkloop: loop {
            // check next group
            let Some(&group) = groups.next() else {
                // if last group, then no bad springs should be left
                return checker.check_only_good_remaining();
            };

            // check good springs
            checker.check_good_before_group()?;

            // next, check if group fits in
            checker.check_group(group)?;

            // if no bits to check are remaining, break the loop
            if checker.none_remaining() {
                break 'checkloop;
            }

            // continue with next group
        }

        // if all available bits were checked, but not all groups could be assigned, return Err
        if checker.none_ignored() {
            if groups.next().is_some() {
                return Err(());
            }
            // checker.check_only_good_remaining()?;
        }

        Ok(())
    }
}

#[derive(Clone)]
pub struct BitSprings {
    broken_mask: u128,
    unknown_mask: u128,
    num_springs: usize,
}

impl std::fmt::Debug for BitSprings {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut broken = self.broken_mask;
        let mut unknown = self.unknown_mask;
        for _ in 0..self.num_springs {
            let spring = match (broken & 0b_1, unknown & 0b_1) {
                (0, 0) => '.',
                (1, 0) => '#',
                (_, 1) => '?',
                _ => unreachable!(),
            };
            write!(f, "{spring}")?;
            broken >>= 1;
            unknown >>= 1;
        }
        Ok(())
    }
}

impl FromStr for BitSprings {
    type Err = ();

    fn from_str(line: &str) -> Result<Self, Self::Err> {
        let parse_broken_mask = |spring| match spring {
            '?' => 0,
            '#' => 1,
            '.' => 0,
            _ => panic!("unknown spring symbol"),
        };
        let parse_unknown_mask = |spring| match spring {
            '?' => 1,
            '#' => 0,
            '.' => 0,
            _ => panic!("unknown spring symbol"),
        };
        fn parse(spring_str: &str, parse_func: impl Fn(char) -> u128) -> u128 {
            spring_str
                .chars()
                .map(parse_func)
                .enumerate()
                .map(|(index, spring)| spring << index)
                .fold(0, |buf, spring| buf | spring)
        }

        let spring_str = line.split_once(' ').ok_or(())?.0;
        let num_springs = spring_str.chars().count();
        let broken_mask = parse(spring_str, parse_broken_mask);
        let unknown_mask = parse(spring_str, parse_unknown_mask);

        Ok(Self {
            broken_mask,
            unknown_mask,
            num_springs,
        })
    }
}

impl BitSprings {
    pub fn unfold(&mut self, factor: usize) {
        if (self.num_springs + 1) * factor > 128 {
            panic!("factor is too big for this spring, won't fit in the representation!");
        }

        if factor == 0 {
            self.broken_mask = 0;
            self.unknown_mask = 0;
            self.num_springs = 0;
        }

        let old_broken = self.broken_mask;
        let old_unknown = self.unknown_mask;
        let old_num = self.num_springs;

        for _ in 0..factor - 1 {
            // first shift in 1x '?'
            self.broken_mask <<= 1;
            self.broken_mask |= 0;
            self.unknown_mask <<= 1;
            self.unknown_mask |= 1;
            self.num_springs += 1;

            // then shift in old mask
            self.broken_mask <<= old_num;
            self.broken_mask |= old_broken;
            self.unknown_mask <<= old_num;
            self.unknown_mask |= old_unknown;
            self.num_springs += old_num;
        }
    }

    pub fn collapse_next(self) -> Option<(Self, Self)> {
        if self.unknown_mask == 0 {
            return None;
        }

        let index = self.unknown_mask.trailing_zeros();
        let spring_mask = 1 << index;
        let unknown_mask = self.unknown_mask & !spring_mask;
        let broken_mask_a = self.broken_mask & !spring_mask;
        let broken_mask_b = self.broken_mask | spring_mask;
        let num_springs = self.num_springs;

        let onsen_a = Self {
            broken_mask: broken_mask_a,
            unknown_mask,
            num_springs,
        };
        let onsen_b = Self {
            broken_mask: broken_mask_b,
            unknown_mask,
            num_springs,
        };

        Some((onsen_a, onsen_b))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_onsen_collapse_none() {
        let line = ".# foo";
        let onsen = BitSprings::from_str(line).unwrap();
        assert!(onsen.collapse_next().is_none());
    }

    #[test]
    fn test_onsen_collapse() {
        let line = ".?#?. foo";
        let onsen = BitSprings::from_str(line).unwrap();
        let (onsen_a, onsen_b) = onsen.collapse_next().unwrap();
        assert_eq!(format!("{onsen_a:?}"), "..#?.");
        assert_eq!(format!("{onsen_b:?}"), ".##?.");
    }

    #[test]
    fn test_onsen_parse() {
        let line = ".?#?. foo";
        let onsen = BitSprings::from_str(line).unwrap();
        assert_eq!(format!("{onsen:?} foo"), line);
    }

    #[test]
    fn test_example_1() {
        let line = "???.### 1,1,3";
        assert_eq!(count_possible_arrangements(line), 1);
    }

    #[test]
    fn test_example_2() {
        let line = ".??..??...?##. 1,1,3";
        assert_eq!(count_possible_arrangements(line), 4);
    }

    #[test]
    fn test_example_3() {
        let line = "?#?#?#?#?#?#?#? 1,3,1,6";
        assert_eq!(count_possible_arrangements(line), 1);
    }

    #[test]
    fn test_example_4() {
        let line = "????.#...#... 4,1,1";
        assert_eq!(count_possible_arrangements(line), 1);
    }

    #[test]
    fn test_example_5() {
        let line = "????.######..#####. 1,6,5";
        assert_eq!(count_possible_arrangements(line), 4);
    }

    #[test]
    fn test_example_6() {
        let line = "?###???????? 3,2,1";
        assert_eq!(count_possible_arrangements(line), 10);
    }
}
