use crate::part1::{BitSprings, GroupSprings};

use std::str::FromStr;

pub fn run(_input: &str) -> usize {
    0 // TODO
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_unfold_example_1_springs() {
        let line = ".# 1";

        let mut springs = BitSprings::from_str(line).unwrap();
        springs.unfold(5);
        assert_eq!(format!("{springs:?}"), ".#?.#?.#?.#?.#");
    }

    #[test]
    fn test_unfold_example_1_groups() {
        let line = ".# 1";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(5);
        assert_eq!(format!("{groups:?}"), "1,1,1,1,1");
    }

    #[test]
    fn test_unfold_example_2_springs() {
        let line = "???.### 1,1,3";

        let mut springs = BitSprings::from_str(line).unwrap();
        springs.unfold(5);
        assert_eq!(
            format!("{springs:?}"),
            "???.###????.###????.###????.###????.###"
        );
    }

    #[test]
    fn test_unfold_example2_groups() {
        let line = "???.### 1,1,3";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(5);
        assert_eq!(format!("{groups:?}"), "1,1,3,1,1,3,1,1,3,1,1,3,1,1,3");
    }

    #[test]
    #[should_panic]
    fn test_unfold_springs_panics_if_too_long() {
        let line = "...............";

        let mut groups = GroupSprings::from_str(line).unwrap();
        groups.unfold(8);
    }
}
