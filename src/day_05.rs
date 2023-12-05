#[derive(Debug)]
struct MapRange {
    dst_start: u64,
    src_start: u64,
    len: u64,
}

impl MapRange {
    fn src_end(&self) -> u64 {
        self.src_start + self.len
    }

    fn shift_dist(&self) -> i64 {
        self.dst_start as i64 - self.src_start as i64
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct SeedRange {
    start: u64,
    len: u64,
}

impl SeedRange {
    fn end(&self) -> u64 {
        self.start + self.len
    }

    fn shift(&self, dist: i64) -> Self {
        Self {
            start: (self.start as i64 + dist) as u64,
            len: self.len,
        }
    }
}

pub fn solve_1(input: &str) -> String {
    let mut lines = input.lines();

    let seeds = parse_seeds(&mut lines);
    assert!(
        matches!(lines.next(), Some(""))
    );

    let maps = parse_maps(&mut lines);

    let mut locs = vec!();
    for seed in seeds {
        locs.push(map_seed_to_loc(seed, &maps));
    }

    locs.iter().min().unwrap().to_string()
}

pub fn solve_2(input: &str) -> String {
    let mut lines = input.lines();

    let seeds = parse_seeds(&mut lines);
    let mut seed_ranges = seeds
        .chunks(2)
        .map(|v| SeedRange { start: v[0], len: v[1] })
        .collect::<Vec<_>>();
    assert!(
        matches!(lines.next(), Some(""))
    );

    let maps = parse_maps(&mut lines);

    for map in maps.iter() {
        let mut mapped_seed_ranges = vec!();
        let mut not_mapped_seed_ranges = seed_ranges.clone();
        for map_range in map.iter() {
            let (_mapped_seed_ranges, _not_mapped_seed_ranges) = map_seed_ranges(&not_mapped_seed_ranges, map_range);
            mapped_seed_ranges.extend(_mapped_seed_ranges);
            not_mapped_seed_ranges = _not_mapped_seed_ranges;
        }
        seed_ranges.clear();
        seed_ranges.extend(mapped_seed_ranges);
        seed_ranges.extend(not_mapped_seed_ranges);
    }

    let mut min_loc = u64::MAX;
    for loc_range in seed_ranges {
        if loc_range.start < min_loc {
            min_loc = loc_range.start;
        }
    }
    min_loc.to_string()
}

fn map_seed_ranges(seed_ranges: &Vec<SeedRange>, map_range: &MapRange) -> (Vec<SeedRange>, Vec<SeedRange>) {
    let mut mapped_seed_ranges = vec!();
    let mut not_mapped_seed_ranges = vec!();
    for seed_range in seed_ranges {
        let (mapped_seed_range, outside_seed_ranges) = map_seed_range(seed_range, map_range);
        if let Some(mapped_seed_range) = mapped_seed_range {
            mapped_seed_ranges.push(mapped_seed_range);
        }
        not_mapped_seed_ranges.extend(outside_seed_ranges);
    }
    (mapped_seed_ranges, not_mapped_seed_ranges)
}

fn map_seed_range(seed_range: &SeedRange, map_range: &MapRange) -> (Option<SeedRange>, Vec<SeedRange>) {
    // Seed range is fully outside of the map range
    if seed_range.end() <= map_range.src_start ||
        seed_range.start >= map_range.src_end()
    {
        return (None, vec!(*seed_range));
    }

    // Seed range is fully inside of the map range
    if seed_range.start >= map_range.src_start &&
        seed_range.end() <= map_range.src_end()
    {
        let new_seed_range_start = seed_range.start + map_range.dst_start - map_range.src_start;
        return (
            Some(SeedRange { start: new_seed_range_start, len: seed_range.len }),
            vec!()
        )
    }

    let mut outside_seed_ranges = vec!();
    let mut intermediate_seed_range = *seed_range;
    // Cut left side of the seed range
    if seed_range.start < map_range.src_start {
        let outside_len = map_range.src_start - seed_range.start;
        outside_seed_ranges.push(
            SeedRange { start: seed_range.start, len: outside_len }
        );
        intermediate_seed_range.start = map_range.src_start;
        intermediate_seed_range.len -= outside_len;
    }
    // Cut right side of the seed range
    if seed_range.end() > map_range.src_end() {
        let outside_len = seed_range.end() - map_range.src_end();
        outside_seed_ranges.push(
            SeedRange { start: map_range.src_end(), len: outside_len }
        );
        intermediate_seed_range.len -= outside_len;
    }

    (
        Some(intermediate_seed_range.shift(map_range.shift_dist())),
        outside_seed_ranges
    )
}

fn parse_seeds<'a>(mut lines: impl Iterator<Item = &'a str>) -> Vec<u64> {
    let seeds_line = lines.next().unwrap();
    let (_name, seeds_str) =  seeds_line.split_once(':').unwrap();
    seeds_str.trim().split(' ')
        .map(str::parse::<u64>)
        .filter_map(Result::ok)
        .collect()
}

fn parse_maps<'a>(mut lines: impl Iterator<Item = &'a str>) -> Vec<Vec<MapRange>> {
    let mut maps = vec!();
    while let Some(_map_name_line) = lines.next() {
        let mut map = vec!();
        loop {
            if let Some(map_str) = lines.next() {
                let map_str = map_str.trim();
                if map_str.is_empty() {
                    break;
                }

                let mut map_range_parts = map_str.splitn(3, ' ')
                    .map(str::parse)
                    .filter_map(Result::ok);
                let dst_start = map_range_parts.next().unwrap();
                let src_start = map_range_parts.next().unwrap();
                let len = map_range_parts.next().unwrap();
                let map_range = MapRange { dst_start, src_start, len };
                map.push(map_range);
            } else {
                break;
            }
        }
        maps.push(map);
    }
    maps
}

fn map_seed_to_loc(seed: u64, maps: &Vec<Vec<MapRange>>) -> u64 {
    let mut seed_loc = seed;

    'maps: for map in maps.iter() {
        for map_range in map.iter() {
            if seed_loc >= map_range.src_start && seed_loc < map_range.src_start + map_range.len {
                seed_loc = map_range.dst_start + (seed_loc - map_range.src_start);
                continue 'maps;
            }
        }
    }
    seed_loc
}

#[cfg(test)]
mod tests {
    use indoc::indoc;
    use test_log::test;
    use super::*;

    const EXAMPLE_INPUT: &'static str = indoc!{"
        seeds: 79 14 55 13

        seed-to-soil map:
        50 98 2
        52 50 48

        soil-to-fertilizer map:
        0 15 37
        37 52 2
        39 0 15

        fertilizer-to-water map:
        49 53 8
        0 11 42
        42 0 7
        57 7 4

        water-to-light map:
        88 18 7
        18 25 70

        light-to-temperature map:
        45 77 23
        81 45 19
        68 64 13

        temperature-to-humidity map:
        0 69 1
        1 0 69

        humidity-to-location map:
        60 56 37
        56 93 4
    "};

    #[test]
    fn test_solve_1() {
        assert_eq!(
            solve_1(EXAMPLE_INPUT),
            "35".to_string()
        );
    }

    #[test]
    fn test_solve_2() {
        assert_eq!(
            solve_2(EXAMPLE_INPUT),
            "46".to_string()
        );
    }

    #[test]
    fn test_map_seed_range() {
        // 7..17 -> 20..30 (10)  - map range from 7 to 20 with length 10
        let map_range = MapRange { dst_start: 20, src_start: 7, len: 10 };

        assert_eq!(
            map_seed_range(&SeedRange { start: 5, len: 2 }, &map_range),
            (None, vec!(SeedRange { start: 5, len: 2 }))
        );
        assert_eq!(
            map_seed_range(&SeedRange { start: 30, len: 93 }, &map_range),
            (None, vec!(SeedRange { start: 30, len: 93 }))
        );
        assert_eq!(
            map_seed_range(&SeedRange { start: 7, len: 10 }, &map_range),
            (Some(SeedRange { start: 20, len: 10 }), vec!())
        );
        assert_eq!(
            map_seed_range(&SeedRange { start: 11, len: 3 }, &map_range),
            (Some(SeedRange { start: 24, len: 3 }), vec!())
        );
        assert_eq!(
            map_seed_range(&SeedRange { start: 6, len: 10 }, &map_range),
            (Some(SeedRange { start: 20, len: 9 }), vec!(SeedRange { start: 6, len: 1 }))
        );
        assert_eq!(
            map_seed_range(&SeedRange { start: 16, len: 10 }, &map_range),
            (Some(SeedRange { start: 29, len: 1 }), vec!(SeedRange { start: 17, len: 9 }))
        );
    }
}
