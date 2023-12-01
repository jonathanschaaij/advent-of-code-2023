use std::time::Instant;
const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

const REV_NUMBER_WORDS: [&str; 9] = [
    "eno", "owt", "eerht", "ruof", "evif", "xis", "neves", "thgie", "enin",
];

fn find_first_digit(line: &str) -> u32 {
    let mut wordlength = vec![0; 9];

    for c in line.chars() {
        if let Some(d) = c.to_digit(10) {
            return d;
        } else {
            for (i, cur_l) in wordlength.iter_mut().enumerate() {
                if c == NUMBER_WORDS[i].chars().nth(*cur_l).unwrap_or(' ') {
                    *cur_l += 1;
                    if *cur_l == NUMBER_WORDS[i].len() {
                        return i as u32 + 1;
                    }
                } else {
                    if c == NUMBER_WORDS[i].chars().nth(0).unwrap() {
                        *cur_l = 1;
                    } else {
                        *cur_l = 0;
                    }
                }
            }
        }
    }
    0
}

fn find_last_digit(line: &str) -> u32 {
    let mut wordlength = vec![0; 9];

    for c in line.chars().rev() {
        if let Some(d) = c.to_digit(10) {
            return d;
        } else {
            for (i, cur_l) in wordlength.iter_mut().enumerate() {
                if c == REV_NUMBER_WORDS[i].as_bytes()[*cur_l] as char {
                    *cur_l += 1;
                    if *cur_l == NUMBER_WORDS[i].len() {
                        return i as u32 + 1;
                    }
                } else {
                    if c == REV_NUMBER_WORDS[i].chars().nth(0).unwrap() {
                        *cur_l = 1;
                    } else {
                        *cur_l = 0;
                    }
                }
            }
        }
    }
    0
}

fn main() {
    let t0 = Instant::now();
    let sum: u32 = include_str!("input.txt")
        .lines()
        .map(|line| find_first_digit(line) * 10 + find_last_digit(line))
        .sum();
    let t1 = Instant::now();
    println!("Time: {}ms", (t1 - t0).as_micros());
    println!("{}", sum);
}

