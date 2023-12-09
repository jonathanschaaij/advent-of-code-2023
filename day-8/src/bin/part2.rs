use std::{
    collections::{HashMap, HashSet},
    time::{Duration, Instant},
};

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
    fn is_end(&self) -> bool {
        return self.0[2] == 'Z';
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

fn reached_target(locations: &Vec<Location>) -> bool {
    locations.iter().all(|l| l.0[2] == 'Z')
}

#[derive(Debug)]
struct Cycle {
    cycle_length: usize,
    cycle_start_steps: usize,
    end_inds: HashSet<usize>,
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

    let mut steps = 0;
    let mut starting_locations: Vec<Location> = map
        .keys()
        .filter(|k| k.0[2] == 'A')
        .map(|k| k.clone())
        .collect::<Vec<_>>();

    println!("Starting locations: {:?}", starting_locations);

    let t_start = Instant::now();
    // Create arrays of possible solutions within a cycle
    let mut cycles = starting_locations
        .iter()
        .map(|loc| {
            let mut current = loc.clone();
            let mut cycle = HashMap::<(Location, usize), usize>::new();
            let mut steps = 0;
            let mut cycle_starting_index = 0;
            let mut cycle_length = 0;
            loop {
                let index = steps % instructions.len();
                let direction = instructions[index];
                let next = match direction {
                    Direction::Left => map[&current][0],
                    Direction::Right => map[&current][1],
                };
                if cycle.contains_key(&(current, index)) {
                    cycle_starting_index = cycle[&(current, index)];
                    cycle_length = steps - cycle_starting_index;
                    break;
                }
                cycle.insert((current, index), steps);
                println!("In the cycle:{:?}, {}, {}", current, index, steps);
                current = next;
                steps += 1;
            }

            // Filter only valid ending Locations
            let mut cycle_end_points: HashSet<usize> = HashSet::new();
            for (pos, ind) in cycle.keys() {
                if pos.is_end() {
                    let cycle_steps: usize = cycle[&(*pos, *ind)] - cycle_starting_index;
                    cycle_end_points.insert(cycle_steps);
                }
            }
            Cycle {
                cycle_length,
                cycle_start_steps: cycle_starting_index,
                end_inds: cycle_end_points,
            }
        })
        .collect::<Vec<_>>();

    let t_cycles = Instant::now();
    println!(
        "Time for detecting cycles: {:?}",
        t_cycles.duration_since(t_start)
    );

    println!(
        "Cycle_start: {:?} \tCycle_length: {}\t\t Endpoints: {}",
        cycles[0].cycle_start_steps,
        cycles[0].cycle_length,
        cycles[0].end_inds.len()
    );

    let mut cycle_iteration = 0;
    let mut steps = 0;
    loop {
        for loop_steps_first_cycle in &cycles[0].end_inds {
            steps = cycles[0].cycle_length * cycle_iteration
                + cycles[0].cycle_start_steps
                + loop_steps_first_cycle;

            let found_end = cycles[1..].iter().all(|c| {
                let relative_loop_step = (steps - c.cycle_start_steps) % c.cycle_length;
                c.end_inds.contains(&relative_loop_step)
            });
            if found_end {
                return steps as i64;
            }
        }
        cycle_iteration += 1;
        if cycle_iteration % 10 != 0 {
            continue;
        }
        // println!(
        //     "Current steps: {}\t\t Time elapsed: {:?}",
        //     steps,
        //     t_cycles.elapsed()
        // );
    }
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
        let input = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";
        assert_eq!(solve(input), 6);
    }

    #[test]
    fn test_find_cycle() {
        let input = "LLL\n\nAAA = (BBB, AAA)\nBBB = (CCC, AAA)\nCCC = (DDZ, AAA)\nDDZ = (EEZ, AAA)\nEEZ = (BBB, AAA)";
        assert_eq!(solve(input), 3);
    }
    #[test]
    fn test_find_cycle_2() {
        let input = "LLL\n\nAAA = (BBB, AAA)\nBBB = (CCC, AAA)\nCCC = (DDZ, AAA)\nDDZ = (BBB, AAA)\nEEZ = (BBB, AAA)";
        assert_eq!(solve(input), 3);
    }
}
