use std::time::Instant;

enum Operator {
    Equal,
    Dash,
}

impl From<char> for Operator {
    fn from(c: char) -> Self {
        match c {
            '=' => Operator::Equal,
            '-' => Operator::Dash,
            _ => panic!("Invalid operator {}", c),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Lens {
    focal_length: usize,
    label: String,
}

fn get_hash(key: &str) -> usize {
    let mut hash: usize = 0;
    for c in key.chars() {
        hash = ((hash + c as usize) * 17) % 256;
    }
    hash
}

fn solve(file: &str) -> i64 {
    let mut boxes: Vec<Vec<Lens>> = vec![vec![]; 256];
    file.split(',').for_each(|w| {
        // for each instruction
        let (label, operator, value_str) = w.trim().chars().fold(
            (String::new(), Operator::Equal, String::new()),
            |(mut label, mut operator, mut value_str), c| {
                match c {
                    'a'..='z' => label.push(c),
                    '0'..='9' => value_str.push(c),
                    _ => operator = Operator::from(c),
                }
                (label, operator, value_str)
            },
        );

        let hash = get_hash(&label);
        match operator {
            Operator::Equal => {
                // set hash to value
                let value = value_str.parse::<usize>().unwrap();

                if let Some(lens) = boxes[hash].iter_mut().find(|l| l.label == label) {
                    lens.focal_length = value;
                } else {
                    boxes[hash].push(Lens {
                        focal_length: value,
                        label: label.clone(),
                    });
                }
            }
            Operator::Dash => {
                // remove lens
                if let Some(index) = boxes[hash].iter().position(|l| l.label == *label) {
                    // .iter().position(|l| l.label == label) {
                    {
                        boxes[hash].remove(index);
                    }
                }
            }
        }
    });

    // Compute focussing power of hashmap
    boxes
        .iter()
        .enumerate()
        .map(|(box_ind, box_arr)| {
            (box_ind + 1)
                * box_arr
                    .iter()
                    .enumerate()
                    .map(|(lens_ind, lens)| (lens_ind + 1) * lens.focal_length)
                    .sum::<usize>() as usize
        })
        .sum::<usize>() as i64
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
        assert_eq!(solve(input), 145);
    }
}
