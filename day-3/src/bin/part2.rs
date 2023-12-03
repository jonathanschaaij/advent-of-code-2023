use std::time::Instant;

fn solve(input: &str) -> u32 {
    let mut lines = input
        .lines()
        .map(|l| l.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let directions: [[i32; 2]; 8] = [
        [-1, -1],
        [-1, 0],
        [-1, 1],
        [0, -1],
        [0, 1],
        [1, -1],
        [1, 0],
        [1, 1],
    ];

    let mut sum = 0;
    for (l_ind, line) in lines.clone().iter().enumerate() {
        for (star_ind, _) in line.iter().enumerate().filter(|(_, c)| **c == '*') {
            let mut numbers = Vec::new();
            for dir in directions.iter() {
                let row = l_ind as i32 + dir[0];
                let col = star_ind as i32 + dir[1];

                if row < 0 || col < 0 || row >= lines.len() as i32 || col >= line.len() as i32 {
                    continue;
                }
                let row = row as usize;
                let mut col = col as usize;

                if !lines[row][col].is_digit(10) {
                    continue;
                }

                // Move to beginning of number
                loop {
                    if col == 0 || !lines[row][col - 1].is_digit(10) {
                        break;
                    }
                    col -= 1;
                }
                // Place whole number into vec
                let mut number = Vec::new();
                loop {
                    number.push(lines[row][col]);
                    lines[row][col] = ' '; // This consumes the number, I hereby assume that a
                                           // number can only be part of 1 gear ratio..... i hope
                    col += 1;
                    if col >= lines[row].len() || !lines[row][col].is_digit(10) {
                        break;
                    }
                }
                numbers.push(number.iter().collect::<String>().parse::<u32>().unwrap());
            }

            if numbers.len() < 2 {
                continue;
            }

            sum += numbers.iter().product::<u32>();
        }
    }
    sum
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
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(solve(input), 467835);
    }
}
