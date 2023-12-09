use std::{collections::HashMap, time::Instant};

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
enum Direction {
    Left,
    Right,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c {
            'L' => Direction::Left,
            'R' => Direction::Right,
            _ => panic!("Invalid direction"),
        }
    }
}

#[derive(Debug, Eq, PartialEq, Hash, Clone, Copy)]
struct Location([char; 3]);

impl Location {
    fn is_start(&self) -> bool {
        self.0[2] == 'A'
    }
    fn is_end(&self) -> bool {
        self.0[2] == 'Z'
    }
}

impl From<&str> for Location {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let third = chars.next().unwrap();
        Location([first, second, third])
    }
}

fn greatest_common_denominator(a: i64, b: i64) -> i64 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn least_common_multiple(a: i64, b: i64) -> i64 {
    a * b / greatest_common_denominator(a, b)
}

fn solve(file: &str) -> i64 {
    let lines = file.lines().collect::<Vec<_>>();
    //parse the instructions
    let instructions = lines[0].chars().map(Direction::from).collect::<Vec<_>>();

    //parse the map
    let mut map = HashMap::<Location, [Location; 2]>::new();
    for line in lines[2..].iter() {
        let mut parts = line.split(" = ");
        let key = parts.next().unwrap();
        let values = parts.next().unwrap();
        let values: Vec<Location> = values[1..values.len() - 1]
            .split(", ")
            .map(|loc| loc.into())
            .collect();
        map.insert(key.into(), [values[0], values[1]]);
    }

    // Find valid starting points
    let starting_points = map
        .iter()
        .filter(|(k, _)| k.is_start())
        .map(|(k, _)| *k)
        .collect::<Vec<_>>();

    let first_end_counts = starting_points
        .iter()
        .map(|start| {
            let mut current = *start;
            let mut steps = 0;
            loop {
                let instruction = instructions[steps % instructions.len()];
                let next = match instruction {
                    Direction::Left => map[&current][0],
                    Direction::Right => map[&current][1],
                };
                current = next;
                steps += 1;
                if current.is_end() {
                    break;
                }
            }
            steps
        })
        .collect::<Vec<_>>();

    // Find the least common multiple of the end counts (THIS SHOULD NOT WORK BUT SEEMS TO BE THE
    // CORRECT ANSWER)

    first_end_counts
        .iter()
        .fold(1, |acc, &x| least_common_multiple(acc, x as i64)) as i64
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution");
    let t0 = Instant::now();
    let result = solve(input);
    let duration = t0.elapsed();
    println!("Result: {}", result);
    println!("Time: {:?}", duration);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 2);
    }

    #[test]
    fn test_whole_part_2() {
        let input = "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)";
        assert_eq!(solve(input), 6);
    }
}
