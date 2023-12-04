use std::time::Instant;

fn get_numbrs(row: &str) -> Vec<u32> {
    // println!("Some numbers: {}", row);
    let o = &row
        .split(" ")
        .filter(|w| !w.is_empty())
        .map(|n| n.parse::<u32>().unwrap())
        .collect::<Vec<u32>>();

    // dbg!(o);
    o.to_vec()
}
fn solve(file: &str) -> usize {
    file.lines()
        .map(|l| l.split(":").nth(1).unwrap())
        // .inspect(|s| println!("NUMBER PART: {}", s))
        .map(|l| {
            let number_strings = l.split("|").collect::<Vec<&str>>();
            assert_eq!(number_strings.len(), 2);
            let winning_numbers = get_numbrs(number_strings[0]);
            let holding_numbers = get_numbrs(number_strings[1]);

            let win_count: usize = holding_numbers
                .iter()
                .filter(|n| winning_numbers.contains(n))
                .count();

            // println!("Win_count: {}", win_count);

            // 2 ^ (win_count as i32 - 1) as u32
            let mut res = 0;
            if win_count > 0 {
                res = 2_usize.pow(win_count as u32 - 1)
            }

            res
        })
        // .inspect(|c| println!("{}", c))
        .sum()
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
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(solve(input), 13);
    }
}
