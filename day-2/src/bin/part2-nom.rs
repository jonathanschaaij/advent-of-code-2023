use std::cmp::max;
use std::time::Instant;

fn solve(file: &str) -> u32 {
    file.lines()
        .map(|line| {
            let mut max_val = [0u32; 3];
            for set in line
                .split(":") // Split into game name and sets
                .nth(1) // Take the sets
                .unwrap()
                .split(";") // Split into sets
                .map(|x| x.split(",")) // Split into colors
                .flatten()
            // For each color
            {
                let a = set
                    .split(" ")
                    .filter(|x| !x.is_empty())
                    .collect::<Vec<&str>>();
                let val = a[0].parse::<u32>().unwrap();
                let i = match a[1] {
                    "blue" => 0,
                    "green" => 1,
                    "red" => 2,
                    _ => panic!("Unknown color"),
                };
                max_val[i] = max(max_val[i], val);
            }
            max_val.iter().product::<u32>()
        })
        .sum::<u32>()
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
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), 2286);
    }
}
