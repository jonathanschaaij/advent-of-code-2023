use std::{ops::Range, time::Instant};

#[derive(Debug)]
enum AlmanacType {
    Seed,
    Soil,
    Fertilizer,
    Water,
    Light,
    Temperature,
    Humidity,
    Location,
}

impl From<&str> for AlmanacType {
    fn from(s: &str) -> Self {
        match s {
            "seed" => AlmanacType::Seed,
            "soil" => AlmanacType::Soil,
            "fertilizer" => AlmanacType::Fertilizer,
            "water" => AlmanacType::Water,
            "light" => AlmanacType::Light,
            "temperature" => AlmanacType::Temperature,
            "humidity" => AlmanacType::Humidity,
            "location" => AlmanacType::Location,
            _ => panic!("Unknown almanac type: {}", s),
        }
    }
}

#[derive(Debug)]
struct ConversionValue {
    from_range: Range<u64>,
    to_range: Range<u64>,
}

impl From<&str> for ConversionValue {
    fn from(row: &str) -> Self {
        let parts = row.split_whitespace();
        let nums = parts.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        ConversionValue {
            from_range: (nums[1]..(nums[1] + nums[2])),
            to_range: (nums[0]..(nums[0] + nums[2])),
        }
    }
}

#[derive(Debug)]
struct ConversionMap {
    from: AlmanacType,
    to: AlmanacType,
    map: Vec<ConversionValue>,
}

impl ConversionMap {
    fn convert(&self, starting_range: Vec<Range<u64>>) -> Vec<Range<u64>> {
        let mut output_ranges = Vec::new();
        let mut leftover_range = starting_range.clone();

        // println!("Converting {:?} to {:?}", self.from, self.to);
        for conv in self.map.iter() {
            let mut new_leftover_range = Vec::new();

            for before in leftover_range.iter() {
                if before.is_empty() {
                    continue;
                }

                // println!("Checking {:?} against {:?}", conv.from_range, before);
                // Check for no overlap
                if conv.from_range.start > before.end || conv.from_range.end < before.start {
                    new_leftover_range.push(before.clone());
                    continue;
                }

                // Check complete overlap
                if before.start <= conv.from_range.start && before.end >= conv.from_range.end {
                    // println!("Complete overlap conversion range inside");
                    output_ranges.push(conv.to_range.clone()); // Overlapping range is converted

                    // Non overlapping ranges are added to the leftover range
                    new_leftover_range.push(before.start..conv.from_range.start);
                    new_leftover_range.push(conv.from_range.end..before.end);
                    continue;
                }

                if before.start >= conv.from_range.start && before.end <= conv.from_range.end {
                    // println!("Complete overlap conversion range outside");
                    let difference = before.start - conv.from_range.start;
                    let new_range = (conv.to_range.start + difference)
                        ..(conv.to_range.start + difference + before.end - before.start);

                    if new_range.start == 0 || new_range.start == 1 {
                        println!("Difference: {}", difference);
                        println!("Found 0 or 1");
                        println!("Before: {:?}", before);
                        println!("Conversion: {:?}", conv);
                        println!("Almanac type: {:?}", self.to);
                    }
                    output_ranges.push(new_range); // Overlapping range is converted
                    continue;
                }

                // Check partial overlap
                // Overlap on beginning of before
                if before.start > conv.from_range.start && before.start < conv.from_range.end {
                    let overlap_length = conv.from_range.end - before.start;
                    let new_range = (conv.to_range.end - overlap_length)..conv.to_range.end;
                    if new_range.start == 0 || new_range.start == 1 {
                        println!("Found 0 or 1");
                        println!("Before: {:?}", before);
                        println!("Conversion: {:?}", conv);
                        println!("Almanac type: {:?}", self.to);
                    }
                    output_ranges.push(new_range); // Overlapping range is converted

                    new_leftover_range.push(conv.from_range.end..before.end);
                    continue;
                }
                // Overlap near end of before
                if before.end > conv.from_range.start && before.end < conv.from_range.end {
                    let overlap_length = before.end - conv.from_range.start;
                    let new_range = conv.to_range.start..(conv.to_range.start + overlap_length);
                    if new_range.start == 0 || new_range.start == 1 {
                        println!("Found 0 or 1");
                        println!("Before: {:?}", before);
                        println!("Conversion: {:?}", conv);
                        println!("Almanac type: {:?}", self.to);
                    }
                    output_ranges.push(new_range); // Overlapping range is converted

                    new_leftover_range.push(before.start..conv.from_range.start);
                    continue;
                }
            }
            leftover_range = new_leftover_range;
        }

        for leftover in leftover_range.iter().filter(|r| !r.is_empty()) {
            output_ranges.push(leftover.clone());
        }
        output_ranges
    }
}

fn solve(file: &str) -> u64 {
    let lines = file.lines().collect::<Vec<_>>();

    let seed_ranges: Vec<Range<u64>> = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>()
        .chunks(2)
        .map(|w| w[0]..w[0] + w[1])
        .collect::<Vec<_>>();

    let mut conversion_maps = Vec::new();
    let mut reading_map = false;
    for line in lines[1..].iter() {
        if line.is_empty() {
            reading_map = false;
            continue;
        }

        if !reading_map {
            // Starting a new map
            let parts = line
                .split_whitespace()
                .nth(0)
                .unwrap()
                .split("-to-")
                .collect::<Vec<_>>();
            assert_eq!(parts.len(), 2);
            let from = AlmanacType::from(parts[0]);
            let to = AlmanacType::from(parts[1]);
            conversion_maps.push(ConversionMap {
                from,
                to,
                map: Vec::new(),
            });
            reading_map = true;
            continue;
        }

        if reading_map {
            // Reading conversion values
            conversion_maps
                .last_mut()
                .unwrap()
                .map
                .push(ConversionValue::from(*line));
        }
    }

    seed_ranges.iter().for_each(|r| println!("{:?}", r));

    // println!("Seed ranges: {:?}", seed_ranges);

    // println!("Conversion maps: {:?}", conversion_maps);
    // Assume the conversion maps are in the correct order
    conversion_maps
        .iter()
        .fold(seed_ranges, |ranges, map| map.convert(ranges))
        .iter()
        // .inspect(|r| println!("{:?}", r))
        .map(|r| r.start)
        .min()
        .unwrap()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution for part 1");
    let t0 = Instant::now();
    let result = solve(input);
    let t1 = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", (t1 - t0).as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "seeds: 79 14 55 13

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
56 93 4";
        assert_eq!(solve(input), 46);
    }

    #[test]
    fn test_simple_rangle() {
        let some_num = 5_u64;
        assert!((1..=10).contains(&some_num));
        assert!((some_num..=some_num + 5).contains(&some_num));
        assert_eq!(1, (1..10).start)
    }
    #[test]
    fn test_range_conv_end_1() {
        let input = "seeds: 10 10 \n\nseed-to-soil map:\n 35 15 10";
        assert_eq!(solve(input), 10);
    }
    #[test]
    fn test_range_conv_end_2() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 55 10";
        assert_eq!(solve(input), 5);
    }
    #[test]
    fn test_range_conv_inclusive_1() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 55 2";
        assert_eq!(solve(input), 5);
    }
    #[test]
    fn test_range_conv_inclusive_2() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 95 55 2";
        assert_eq!(solve(input), 50);
    }
    #[test]
    fn test_range_inclusive_1() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 85 45 20";
        assert_eq!(solve(input), 90);
    }
    #[test]
    fn test_range_inclusive_2() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 45 20";
        assert_eq!(solve(input), 10);
    }
    #[test]
    fn test_range_conv_start_1() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 85 45 10";
        assert_eq!(solve(input), 55);
    }
    #[test]
    fn test_range_conv_start_2() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 45 10";
        assert_eq!(solve(input), 10);
    }

    #[test]
    fn test_range_outside_1() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 20 10";
        assert_eq!(solve(input), 50);
    }
    #[test]
    fn test_range_outside_2() {
        let input = "seeds: 50 10 \n\nseed-to-soil map:\n 5 80 10";
        assert_eq!(solve(input), 50);
    }
}
