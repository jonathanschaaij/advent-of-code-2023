use std::time::Instant;

fn solve(file: &str) -> i32 {
    // First line has times,
    // Second line has distances
    let mut lines = file.lines().collect::<Vec<_>>();
    let times = lines[0]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();
    let dist = lines[1]
        .split(':')
        .nth(1)
        .unwrap()
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    assert_eq!(times.len(), dist.len());
    let mut prod = 1;
    for i in 0..times.len() {
        prod *= (0..times[i])
            .map(|t_hold| t_hold * (times[i] - t_hold))
            .filter(|d| d > &dist[i])
            .count();
    }
    prod as i32
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
        let input = "Time:      7  15   30
Distance:  9  40  200
";
        assert_eq!(solve(input), 288);
    }
}
