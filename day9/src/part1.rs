pub fn run(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse().unwrap()))
        .map(calc_next)
        .sum()
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum CalcWhat {
    Next,
    Prev,
}

fn calc_next(history: impl Iterator<Item = isize>) -> isize {
    calc(history, CalcWhat::Next)
}

/// Calculate the next element for a given history.
///
/// ## Example
/// Assume the history input `10 13 16 21 30 45`:
/// ```text
/// 10  13  16  21  30  45    <-- Line 0 (original history)
///       3   5   9  15       <-- Line 1 (all differences of line 0)
///         2   4   6         <-- Line 2 (all differences of line 1)
///           2   2           <-- Line 3 (all differences of line 2)
///             0             <-- Line 4 (all differences of line 3)
/// ```
///
///## Solution
/// The solution to this example is `68`. To calculate it,
/// the next difference needs to be determined:
/// * Because of `line 4`, all diffs in `line 3` have to be `2`.
/// * In `line 2`, the new diff in `line 4` would be calculated with `X - 6 = 2`,
///   so the new diff here is `8`.
/// * In `line 1` the new diff would be `23` accordingly.
/// * When adding this diff to the last number `45` in `line 0`, we get the solution `68`.
///
/// ## Algorithm
/// * Notice that we can just accumulate the last diagonal to the right: `2 + 6 + 15 + 45 = 68`
/// * Furthermore, the first element is not even needed to calculate all needed diffs
/// * We reverse the history, so we effectively iterate it from the back, ignoring
///   unneeded entries at the back
/// * Thus we want to calculate all entries on the right diagonale.
/// * Values we already used can be overwritten, so entries can be reused for calculated diffs
///
/// ## Steps
/// **Notes**
/// * History content depicted in reverse order for better overview,
///   i.e. last element has Position `0`.
/// * Entries marked with `x` are not taken into account anymore for the rest of the algorithm.
///
/// **Step 0**
/// * First add `45` to the `accumulator`: `accumulator == 45`
/// * Then calculate the `line 1` diffs: `45 - 30 = 15` and so on...
/// * Store them in the history list`: `x 3 5 9 15`
/// * Range that is taken into account contains still 4 elements => continue
///
/// **Step 1**
/// * Add `15` (stored in first element) to `accumulator`: `accumulator == 60`
/// * Calculate `line 2` diffs: `15 - 9 = 6` and so on...
/// * Store them in the history list`: `x x 2 4 6`
/// * Range that is taken into account contains still 3 elements => continue
///
/// **Step 2**
/// * Add `6` to `accumulator`: `accumulator == 66`
/// * Calculate `line 3` diffs: `6 - 4 = 2` and so on...
/// * Store them in the history list`: `x x x 2 2`
/// * Range that is taken into account contains still 2 elements => continue
///
/// **Step 3**
/// * Add `2` to `accumulator`: `accumulator == 68`
/// * Calculate `line 4` diffs: `2 - 2 = 0`
/// * Store them in the history list`: `x x x x 0`
/// * Range that is taken into account contains still 1 element => continue
///
/// **Step 4**
/// * No more diffs to calculate, stop loop
///
/// ## Asymptotic order
/// Should be `O(n²)` if I'm not mistaken.
pub fn calc(history: impl Iterator<Item = isize>, calc_what: CalcWhat) -> isize {
    let mut list: Vec<_> = history.collect();
    assert!(list.len() > 1);
    if calc_what == CalcWhat::Next {
        list.reverse();
    }
    let mut accu = 0;
    let mut affecting_range_end = list.len() - 1;
    while affecting_range_end > 0 {
        accu += list[0];
        affecting_range_end -= 1;
        for i in 0..affecting_range_end {
            list[i] -= list[i + 1];
        }
    }

    accu
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example() {
        let input = "\
            0 3 6 9 12 15\n\
            1 3 6 10 15 21\n\
            10 13 16 21 30 45";
        assert_eq!(run(input), 114);
    }

    #[test]
    fn test_calc_diff_over_0() {
        let history = [4, 2, 1, 1];
        assert_eq!(calc_next(history.into_iter()), 2);
    }

    #[test]
    fn test_calc_negative_0() {
        let history = [-3, -2, -1];
        assert_eq!(calc_next(history.into_iter()), 0);
    }

    #[test]
    fn test_calc_negative_1() {
        let history = [0, -1, -3, -6];
        assert_eq!(calc_next(history.into_iter()), -10);
    }

    #[test]
    fn test_calc_next_0() {
        let history = [3, 3, 3];
        assert_eq!(calc_next(history.into_iter()), 3);
    }

    #[test]
    fn test_calc_next_1() {
        let history = [0, 3, 6, 9, 12, 15];
        assert_eq!(calc_next(history.into_iter()), 18);
    }

    #[test]
    fn test_calc_next_2() {
        let history = [1, 3, 6, 10, 15, 21];
        assert_eq!(calc_next(history.into_iter()), 28);
    }

    #[test]
    fn test_calc_next_3() {
        let history = [10, 13, 16, 21, 30, 45];
        assert_eq!(calc_next(history.into_iter()), 68);
    }
}
