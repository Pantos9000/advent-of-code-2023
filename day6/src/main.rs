pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

type Time = usize;
type Distance = usize;

#[derive(Debug, Clone, Copy)]
struct Race {
    time: Time,
    record: Distance,
}

impl Race {
    fn new(time: Time, record: Distance) -> Self {
        Self { time, record }
    }

    fn calc_distance(&self, press_time: Time) -> Distance {
        let run_time = self.time - press_time;
        let velocity = press_time;
        run_time * velocity
    }

    fn calc_num_win_options(&self) -> usize {
        let press_times = 0..=self.time;
        press_times
            .map(|press_time| self.calc_distance(press_time))
            .filter(|distance| distance > &self.record)
            .count()
    }
}

// TODO is this lifetime correct?
// see https://www.youtube.com/watch?v=CWiz_RtA1Hw
fn parse_races(input: &str) -> impl Iterator<Item = Race> + '_ {
    let (times, distances) = input.split_once('\n').unwrap();

    let (_, times) = times.split_once(':').unwrap();
    let times = times.split(' ').filter_map(|x| x.parse::<usize>().ok());

    let (_, distances) = distances.split_once(':').unwrap();
    let distances = distances
        .trim()
        .split(' ')
        .filter_map(|x| x.parse::<usize>().ok());

    times
        .zip(distances)
        .map(|(time, distance)| Race::new(time, distance))
}

fn part1(input: &str) -> usize {
    parse_races(input)
        .map(|race| race.calc_num_win_options())
        .product()
}

fn parse_badly_kerned_race(input: &str) -> Race {
    let (time_input, record_input) = input.split_once('\n').unwrap();

    let (_, time_input) = time_input.split_once(':').unwrap();
    let mut time = String::new();
    time_input.trim().split(' ').for_each(|s| time.push_str(s));
    let time = time.parse().unwrap();

    let (_, record_input) = record_input.split_once(':').unwrap();
    let mut record = String::new();
    record_input
        .trim()
        .split(' ')
        .for_each(|s| record.push_str(s));
    let record = record.parse().unwrap();

    Race::new(time, record)
}
fn part2(input: &str) -> usize {
    parse_badly_kerned_race(input).calc_num_win_options()
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
    fn test_race() {
        let race = Race::new(7, 9);
        assert_eq!(race.calc_num_win_options(), 4);
        let race = Race::new(15, 40);
        assert_eq!(race.calc_num_win_options(), 8);
        let race = Race::new(30, 200);
        assert_eq!(race.calc_num_win_options(), 9);
    }

    #[test]
    fn test_parse_race() {
        let input = "\
            Time:        53     91     67     68\n\
            Distance:   250   1330   1081   1025";
        let races: Vec<_> = parse_races(input).collect();
        assert_eq!(races.len(), 4);
    }
}
