use std::time::Instant;

fn solve(file: &str) -> i64 {
    file.split(',')
        .map(|w| {
            w.trim()
                .chars()
                // .inspect(|c| println! {"Char: {}\t Ascii: {}", c, *c as usize})
                .fold(0, |acc, c| {
                    (((acc as usize + c as usize) * 17) % 256) as i64
                })
        })
        .inspect(|v| println! {"Value: {}", v})
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
        let input = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";
        assert_eq!(solve(input), 1320);
    }
}
