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

impl From<&str> for Location {
    fn from(s: &str) -> Self {
        let mut chars = s.chars();
        let first = chars.next().unwrap();
        let second = chars.next().unwrap();
        let third = chars.next().unwrap();
        Location([first, second, third])
    }
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

    //Walk the map until it reaches target
    let mut current = Location(['A', 'A', 'A']);
    let target = Location(['Z', 'Z', 'Z']);
    let mut steps = 0;

    while current != target {
        let direction = instructions[steps % instructions.len()];
        let next = match direction {
            Direction::Left => map[&current][0],
            Direction::Right => map[&current][1],
        };
        current = next;
        steps += 1;
    }

    steps as i64
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
