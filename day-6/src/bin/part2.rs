use std::time::Instant;

fn solve(file: &str) -> u64 {
    // First line has times,
    // Second line has distances
    let mut lines = file.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();
    let dist = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<u64>()
        .unwrap();

    (0..times)
        .map(|t_hold| t_hold * (times - t_hold))
        .filter(|d| d > &dist)
        .count() as u64
}

fn solve_fast(file: &str) -> i64 {
    let lines = file.lines().collect::<Vec<_>>();
    let t = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<f64>()
        .unwrap();
    let dist = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join("")
        .parse::<f64>()
        .unwrap();

    let t_hold_max: f64 = ((-t) - (t * t - 4_f64 * dist).sqrt()) / (-2_f64);
    let t_hold_min: f64 = ((-t) + (t * t - 4_f64 * dist).sqrt()) / (-2_f64);

    t_hold_max.ceil() as i64 - t_hold_min.floor() as i64
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution for part 1");
    let t0 = Instant::now();
    let result = solve_fast(input);
    let t1 = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", (t1 - t0).as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(solve(input), 288);
    }
}
