trait ReplaceWordNumbers {
    fn replace_word_numbers(&self) -> String;
}
impl ReplaceWordNumbers for String {
    fn replace_word_numbers(&self) -> String {
        let words: Vec<&str> = vec![
            "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let mut str: String = self.clone();
        for (i, word) in words.iter().enumerate() {
            // words are reinserted in case of overlap with letters
            // Can be optimized by iterating over the string once and
            str = str.replace(word, &format!("{}{}{}", word, i + 1, word));
        }
        str
    }
}

fn run_part2(file: &str) -> usize {
    file.lines()
        .map(|l| {
            println!("{:?}", l);
            let s = l.parse::<String>().unwrap();
            println!("{:?}", s);
            let o = s
                .replace_word_numbers()
                .chars()
                .filter(|c| c.is_digit(10))
                .collect::<Vec<_>>();

            println!("{:?}", o);
            o
        })
        .map(|v| {
            format!("{}{}", v[0], v[v.len() - 1])
                .parse::<usize>()
                .unwrap()
        })
        .sum()
}

fn main() {
    let file = include_str!("input.txt");
    println!("Sum: {}", run_part2(file));
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
        assert_eq!(run_part2(testfile), 281);
    }
}
