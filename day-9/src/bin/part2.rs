use std::time::Instant;

fn find_diff_seq(seq: &Vec<i64>) -> i64 {
    let differences = seq.windows(2).map(|w| w[1] - w[0]).collect::<Vec<i64>>();
    let diff = match differences.iter().all(|d| *d == 0) {
        true => *seq.first().unwrap(),
        false => *seq.first().unwrap() - find_diff_seq(&differences),
    };

    println!("diff: {} \t Values: {:?}", diff, seq);
    return diff;
}
fn solve(file: &str) -> i64 {
    file.lines()
        .map(|l| {
            l.split_whitespace()
                .map(|n| n.parse::<i64>().unwrap())
                .collect::<Vec<i64>>()
        })
        .map(|values| find_diff_seq(&values))
        .inspect(|v| println!("{}", v))
        .sum()
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
        let input = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45";
        assert_eq!(solve(input), 2);
    }
}
