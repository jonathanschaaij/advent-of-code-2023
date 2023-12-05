use std::time::Instant;

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
    from_val: u64,
    to_val: u64,
    range: u64,
}

impl From<&str> for ConversionValue {
    fn from(row: &str) -> Self {
        // println!("Parsing row to conversion: {}", row);
        let parts = row.split_whitespace();
        let nums = parts.map(|s| s.parse::<u64>().unwrap()).collect::<Vec<_>>();
        ConversionValue {
            from_val: nums[1],
            to_val: nums[0],
            range: nums[2],
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
    fn convert(&self, value: Option<u64>) -> Option<u64> {
        if let Some(value) = value {
            println!("Converting {} from {:?} to {:?}", value, self.from, self.to);
            for conversion in self.map.iter() {
                if (conversion.from_val..=(conversion.from_val + conversion.range)).contains(&value)
                {
                    return Some(conversion.to_val + (value - conversion.from_val));
                }
            }
            return Some(value);
        }
        None
    }
}

fn solve(file: &str) -> u64 {
    let lines = file.lines().collect::<Vec<_>>();

    let seeds = lines[0]
        .split(":")
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse::<u64>().unwrap())
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

    // Assume the conversion maps are in the correct order

    seeds
        .iter()
        .map(|seed| {
            conversion_maps
                .iter()
                .fold(Some(*seed), |acc, map| map.convert(acc))
        })
        .flatten()
        .min()
        .unwrap_or(11)
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
        assert_eq!(solve(input), 35);
    }

    #[test]
    fn test_simple_rangle() {
        let some_num = 5_u64;
        assert!((1..=10).contains(&some_num));
        assert!((some_num..=some_num + 5).contains(&some_num));
    }
}
