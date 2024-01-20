use crate::part1;

pub fn run(input: &str) -> usize {
    const MAX_STRAIGHT_WALKS: u8 = 10;
    const MIN_STRAIGHT_WALKS: u8 = 4;

    part1::hamster_wheel::<MIN_STRAIGHT_WALKS, MAX_STRAIGHT_WALKS>(input)
}
