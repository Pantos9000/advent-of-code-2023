pub fn run(input: &str) -> isize {
    input
        .lines()
        .map(|line| line.split(" ").map(|s| s.parse().unwrap()))
        .map(calc_next)
        .sum()
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
/// * We want to calculate all entries on the right diagonale.
/// * Values we already used can be overwritten, so entries can be reused for calculated diffs
///
/// ## Steps
/// Note: History content depicted in reverse order for better overview,
/// i.e. last element has Position `0`.
///
/// **Step 0**
/// * First add `45` to the `accumulator`: `accumulator == 45`
/// * Then calculate the `line 1` diff: `45 - 30 = 15`
/// * Store it at position `0`: `10 13 16 21 30 15`
/// * The diff is not `0`, so last line was not yet reached => continue
///
/// **Step 1**
/// * Add `15` (stored in first element) to `accumulator`: `accumulator == 60`
/// * Calculate `line 2` diff by subtracting the diffs from `line 1`
/// * Only `15` is available in `history[0]`, so iterate over the history until the last element
///   that can affect the diff, and store the diffs in the Vec
/// * Do iteration in reverse, so newly calculated diffs are immmediately propagated:
///   `30 - 21 = 9`: `10 13 16 21 9 15`
/// * Do iteration until the beginning of the array, so needed diff of this line is calculated:
///   `15 - 9 = 6`: `10 13 16 21 9 6`
///* Diff at position `0` is not `0`, so continue
///
/// **Step 2**
/// * Add `6` to `accumulator`: `accumulator == 66`
/// * Calculate `line 3` diff by reverse-iterating again over the past history affecting it,
///   i.e. one element more than last iteration:
///   * `21 - 16 = 5`: `10 13 16 5 9 6`
///   * `9 - 5 = 4`: `10 13 16 5 4 6`
///   * `6 - 4 = 2`: `10 13 16 5 4 2`
/// * The diff is`2`, not `0`, so continue
///
/// **Step 3**
/// * Add `2` to `accumulator`: `accumulator == 68`
/// * Calculate `line 4` diff:
///   * `16 - 13 = 3`: `10 13 3 5 4 2`
///   * `5 - 3 = 2`: `10 13 3 2 4 2`
///   * `4 - 2 = 2`: `10 13 3 2 2 2`
///   * `2 - 2 = 0`: `10 13 3 2 2 0`
/// * Diff is `0`, stop and return `accumulator`
///
/// ## Asymptotic order
/// Should be `O(n²)` if I'm not mistaken.
fn calc_next(history: impl Iterator<Item = isize>) -> isize {
    let mut history: Vec<_> = history.collect();
    history.reverse();
    let mut accu = 0;
    let mut affecting_range_end = 0;
    while history[0] != 0 {
        affecting_range_end += 1;
        accu += history[0];
        for i in (0..affecting_range_end).rev() {
            history[i] -= history[i + 1];
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
