mod items;

use items::UncannyItem;

use std::marker::PhantomData;
use std::ops::Range;

pub fn read_input() -> String {
    use std::fs;

    let input_path = "./input";
    fs::read_to_string(input_path).unwrap()
}

struct UncannyRange<UncannyFrom: UncannyItem, UncannyTo: UncannyItem> {
    source_range: Range<usize>,
    dest_range: Range<usize>,
    _phantom_from: PhantomData<UncannyFrom>,
    _phantom_to: PhantomData<UncannyTo>,
}

impl<UncannyFrom: UncannyItem, UncannyTo: UncannyItem> UncannyRange<UncannyFrom, UncannyTo> {
    fn from_line(line: &str) -> Self {
        let mut iter = line.split(' ');
        let dest_range_start = iter.next().unwrap().parse().unwrap();
        let source_range_start = iter.next().unwrap().parse().unwrap();
        let range_length: usize = iter.next().unwrap().parse().unwrap();
        let source_range_end = source_range_start + range_length;
        let dest_range_end = dest_range_start + range_length;

        let source_range = source_range_start..source_range_end;
        let dest_range = dest_range_start..dest_range_end;

        Self {
            source_range,
            dest_range,
            _phantom_from: PhantomData,
            _phantom_to: PhantomData,
        }
    }

    fn get(&self, key: UncannyFrom) -> Option<UncannyTo> {
        let key = key.value();

        if !self.source_range.contains(&key) {
            return None;
        }

        let offset = key - self.source_range.start;
        let dest = self.dest_range.start + offset;

        Some(UncannyTo::from_int(dest))
    }
}

struct UncannyMap<UncannyFrom: UncannyItem, UncannyTo: UncannyItem> {
    ranges: Vec<UncannyRange<UncannyFrom, UncannyTo>>,
}

impl<UncannyFrom, UncannyTo> std::fmt::Display for UncannyMap<UncannyFrom, UncannyTo>
where
    UncannyFrom: UncannyItem,
    UncannyTo: UncannyItem,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let uncanny_from = UncannyFrom::identifier();
        let uncanny_to = UncannyTo::identifier();
        writeln!(f, "UncannyMap<{uncanny_from},{uncanny_to}>")?;
        for range in &self.ranges {
            let src_start = range.source_range.start;
            let src_end = range.source_range.end;
            let dst_start = range.dest_range.start;
            let dst_end = range.dest_range.end;
            writeln!(f, "  {src_start}..{src_end} -> {dst_start}..{dst_end}")?;
        }
        Ok(())
    }
}

impl<UncannyFrom, UncannyTo> UncannyMap<UncannyFrom, UncannyTo>
where
    UncannyFrom: UncannyItem,
    UncannyTo: UncannyItem,
{
    fn parse(input: &str) -> Self {
        let from_id = UncannyFrom::identifier();
        let to_id = UncannyTo::identifier();
        let map_id = format!("{from_id}-to-{to_id} map:");

        let map_start = input.find(&map_id).unwrap();
        let ranges = input[map_start..]
            .lines()
            .skip(1)
            .take_while(|line| !line.is_empty())
            .map(|line| UncannyRange::from_line(line))
            .collect();

        Self { ranges }
    }

    fn get(&self, key: UncannyFrom) -> UncannyTo {
        let default = UncannyTo::from_int(key.value());
        self.ranges
            .iter()
            .find_map(|range| range.get(key))
            .unwrap_or(default)
    }
}

fn parse_seeds_part1(input: &str) -> Vec<items::Seed> {
    let (_, seeds) = input.lines().next().unwrap().split_once(':').unwrap();
    seeds
        .trim()
        .split(' ')
        .map(|num| num.parse().unwrap())
        .map(items::Seed::from_int)
        .collect()
}

fn part1(input: &str) -> usize {
    let seed_to_soil: UncannyMap<items::Seed, items::Soil> = UncannyMap::parse(input);
    let soil_to_fert: UncannyMap<items::Soil, items::Fertilizer> = UncannyMap::parse(input);
    let fert_to_water: UncannyMap<items::Fertilizer, items::Water> = UncannyMap::parse(input);
    let water_to_light: UncannyMap<items::Water, items::Light> = UncannyMap::parse(input);
    let light_to_temp: UncannyMap<items::Light, items::Temperature> = UncannyMap::parse(input);
    let temp_to_humidity: UncannyMap<items::Temperature, items::Humidity> =
        UncannyMap::parse(input);
    let humidity_to_location: UncannyMap<items::Humidity, items::Location> =
        UncannyMap::parse(input);

    let seeds = parse_seeds_part1(input);

    seeds
        .iter()
        .map(|seed| seed_to_soil.get(*seed))
        .map(|soil| soil_to_fert.get(soil))
        .map(|fert| fert_to_water.get(fert))
        .map(|water| water_to_light.get(water))
        .map(|light| light_to_temp.get(light))
        .map(|temp| temp_to_humidity.get(temp))
        .map(|humidity| humidity_to_location.get(humidity))
        .map(|location| location.value())
        .min()
        .unwrap()
}

fn part2(_input: &str) -> u32 {
    // TODO
    0
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
    fn test_parse_seeds() {
        let input = "seeds: 4043382508 113348245 177922221\n";

        let mut seeds = parse_seeds_part1(input);
        assert_eq!(seeds.pop().unwrap().value(), 177922221);
        assert_eq!(seeds.pop().unwrap().value(), 113348245);
        assert_eq!(seeds.pop().unwrap().value(), 4043382508);
        assert!(seeds.pop().is_none());
    }

    #[test]
    fn test_uncanny_range() {
        let line = "1 2 2";
        let range: UncannyRange<items::Seed, items::Soil> = UncannyRange::from_line(line);
        assert!(range.get(items::Seed::from_int(0)).is_none());
        assert!(range.get(items::Seed::from_int(1)).is_none());
        assert_eq!(range.get(items::Seed::from_int(2)).unwrap().value(), 1);
        assert_eq!(range.get(items::Seed::from_int(3)).unwrap().value(), 2);
        assert!(range.get(items::Seed::from_int(4)).is_none());
    }

    #[test]
    fn test_uncanny_map() {
        let input = "\
        seeds: 79 14 55 13\n\
\n\
        seed-to-soil map:\n\
        50 98 2\n\
        52 50 48\n\
\n\
        soil-to-fertilizer map:\n\
        0 15 37\n\
        37 52 2\n\
        39 0 15\n\
\n\
        fertilizer-to-water map:\n\
        49 53 8\n\
        0 11 42\n\
        42 0 7\n\
        57 7 4\n\
\n\
        water-to-light map:\n\
        88 18 7\n\
        18 25 70\n\
\n\
        light-to-temperature map:\n\
        45 77 23\n\
        81 45 19\n\
        68 64 13\n\
\n\
        temperature-to-humidity map:\n\
        0 69 1\n\
        1 0 69\n\
\n\
        humidity-to-location map:\n\
        60 56 37\n\
        56 93 4";

        let seed_to_soil: UncannyMap<items::Seed, items::Soil> = UncannyMap::parse(input);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(0)).value(), 0);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(49)).value(), 49);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(50)).value(), 52);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(97)).value(), 99);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(98)).value(), 50);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(99)).value(), 51);
        assert_eq!(seed_to_soil.get(items::Seed::from_int(100)).value(), 100);
    }
}
