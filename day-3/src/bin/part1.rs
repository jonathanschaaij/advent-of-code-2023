use std::ops::RangeBounds;
use std::task::Wake;
use std::time::Instant;

#[derive(Debug, Clone, Copy)]
struct NumberPosition {
    number: u32,
    start_ind: i32,
    end_ind: i32,
    summed: bool,
}

fn solve(file: &str) -> u32 {
    let mut prev_line: Vec<NumberPosition> = vec![];
    let mut prev_line_symbols: Vec<i32> = vec![];
    let mut sum = 0;
    for line in file.lines() {
        let mut line_numbers: Vec<NumberPosition> = vec![];
        let mut start_ind = 0;
        let mut cur_number: Vec<char> = vec![];
        let mut symbol_positions: Vec<i32> = vec![];
        for (i, c) in line.chars().enumerate() {
            if c.is_digit(10) {
                if cur_number.len() == 0 {
                    start_ind = i;
                }
                cur_number.push(c);
            } else {
                if cur_number.len() > 0 {
                    let end_ind = i - 1;
                    let number: u32 = cur_number.iter().collect::<String>().parse().unwrap();
                    line_numbers.push(NumberPosition {
                        number,
                        start_ind: start_ind as i32,
                        end_ind: end_ind as i32,
                        summed: false,
                    });
                    cur_number = vec![];
                }

                if c != '.' {
                    symbol_positions.push(i as i32);
                }
            }
        }
        if cur_number.len() > 0 {
            let end_ind = line.len() - 1;
            let number: u32 = cur_number.iter().collect::<String>().parse().unwrap();
            line_numbers.push(NumberPosition {
                number,
                start_ind: start_ind as i32,
                end_ind: end_ind as i32,
                summed: false,
            });
        }

        let all_symbols: Vec<i32> = [prev_line_symbols, symbol_positions.clone()].concat();
        // Check for numbers in range
        for symbol_pos in all_symbols.iter() {
            for number in &mut line_numbers {
                if number.summed {
                    continue;
                }
                if number.start_ind <= *symbol_pos + 1 && number.end_ind >= *symbol_pos - 1 {
                    sum += number.number;
                    number.summed = true;
                }
            }
            for number in prev_line.iter().filter(|n| !n.summed) {
                if number.start_ind <= *symbol_pos + 1 && number.end_ind >= *symbol_pos - 1 {
                    sum += number.number;
                }
            }
        }
        prev_line = line_numbers;
        prev_line_symbols = symbol_positions;
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
        assert_eq!(solve(input), 4361);
    }
    #[test]
    fn test_single_line() {
        assert_eq!(10, solve("10*....."));
        assert_eq!(10, solve(".....*10"));
        assert_eq!(10, solve(".....*10....."));
        assert_eq!(10, solve(".....10*....."));
        assert_eq!(0, solve(".....10.*...."));
        assert_eq!(0, solve("....*.10..*"));
    }
    #[test]
    fn test_two_lines() {
        assert_eq!(10, solve("10........\n*........."));
        assert_eq!(00, solve("10........\n........*."));
        assert_eq!(10, solve("10........\n..*......."));
        assert_eq!(10, solve("...10.....\n..*......."));
        assert_eq!(10, solve("10........\n.*........"));
    }
    #[test]
    fn test_sybol_at_zero() {
        assert_eq!(10, solve("*10......"));
    }
}
