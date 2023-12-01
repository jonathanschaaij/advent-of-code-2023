use std::time::Instant;

const NUMBER_WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

trait InsertNumberInWord {
    fn replace_word_numbers(&self) -> String;
}
impl InsertNumberInWord for String {
    fn replace_word_numbers(&self) -> String {
        let mut str: String = self.clone();
        for (i, word) in NUMBER_WORDS.iter().enumerate() {
            // words are reinserted in case of overlap with letters
            // Can be optimized by iterating over the string once and
            str = str.replace(word, &format!("{}{}{}", word, i + 1, word));
        }
        str
    }
}

fn string_to_calibration_value(line: &str) -> u32 {
    let mut digits: Vec<u32> = vec![];
    let mut wordlength = vec![0; 9];
    line.chars().for_each(|c| {
        if let Some(d) = c.to_digit(10) {
            digits.push(d);
            wordlength = vec![0; 9];
        } else {
            for (i, cur_l) in wordlength.iter_mut().enumerate() {
                if c == NUMBER_WORDS[i].chars().nth(*cur_l).unwrap() {
                    *cur_l += 1;
                    if *cur_l == NUMBER_WORDS[i].len() {
                        digits.push(i as u32 + 1);
                        *cur_l = 0;
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
    });
    digits[0] * 10 + digits[digits.len() - 1]
}

fn run_part2(file: &str) -> u32 {
    file.lines()
        .map(|l| {
            l.parse::<String>()
                .unwrap()
                .replace_word_numbers()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<_>>()
        })
        .map(|v| {
            format!("{}{}", v[0], v[v.len() - 1])
                .parse::<u32>()
                .unwrap()
        })
        .sum()
}

fn improved_run(file: &str) -> usize {
    file.lines()
        .map(|l| {
            let mut digits: Vec<usize> = vec![];
            for (i, c) in l.chars().enumerate() {
                if c.is_digit(10) {
                    digits.push(c.to_digit(10).unwrap() as usize);
                } else {
                    for (j, word) in NUMBER_WORDS.iter().enumerate() {
                        if l[i..].starts_with(word) {
                            digits.push(j as usize + 1);
                        }
                    }
                }
            }
            digits[0] * 10 + digits[digits.len() - 1]
        })
        .sum()
}

fn improved_run_2(file: &str) -> usize {
    file.lines()
        .map(|l| string_to_calibration_value(l) as usize)
        .sum()
}

fn main() {
    let file = include_str!("input.txt");
    let t0 = Instant::now();
    let res_1 = run_part2(file);
    let t1 = Instant::now();
    let res_2 = improved_run(file);
    let t2 = Instant::now();
    let res_3 = improved_run_2(file);
    let t3 = Instant::now();

    println!("Sum: {}, Time: {}", res_1, (t1 - t0).as_micros());
    println!("Sum: {}, Time: {}", res_2, (t2 - t1).as_micros());
    println!("Sum: {}, Time: {}", res_3, (t3 - t2).as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_replace_word_numbers() {
        assert_eq!(
            "onetwothreefourfivesixseveneightnine"
                .to_string()
                .replace_word_numbers()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<String>(),
            "123456789"
        );
    }
    #[test]
    fn test_whole_file() {
        let testfile = include_str!("test2.txt");
        assert_eq!(improved_run2(testfile), 281);
    }
}
