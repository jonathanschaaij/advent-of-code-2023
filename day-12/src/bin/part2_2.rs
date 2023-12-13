use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum State {
    Empty,
    Yes,
    Maybe,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::Empty,
            '#' => State::Yes,
            '?' => State::Maybe,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Case {
    length: Vec<i64>,
    groups: Vec<Vec<State>>,
}

impl From<&str> for Case {
    fn from(s: &str) -> Self {
        let mut parts = s.split_whitespace();
        let single_row = parts
            .next()
            .unwrap()
            .chars()
            .map(|c| State::from(c))
            .collect::<Vec<State>>();

        let mut row = single_row.clone();
        for _ in 0..4 {
            row.push(State::Maybe);
            row.extend(single_row.clone());
        }

        let groups = row
            .split(|s| *s == State::Empty)
            .map(|g| g.to_vec())
            .collect();

        let length: Vec<i64> = parts
            .next()
            .unwrap()
            .split(',')
            .map(|p| p.parse::<i64>().unwrap())
            .collect();

        let length = length
            .iter()
            .cycle()
            .take(length.len() * 5)
            .cloned()
            .collect();

        Case { length, groups }
    }
}

impl Case {
    fn solution_count(&self, cache: &mut HashMap<Case, i64>) -> i64 {
        if let Some(c) = cache.get(self) {
            return *c;
        }

        let mut count = 0;

        count += self._find_solutions(cache);

        cache.insert(self.clone(), count);
        // println!("{:?} {}", self, count);
        count
    }

    fn _trivial_solutions(&self) -> Option<i64> {
        if self.length.len() == 0 {
            if self.groups.iter().any(|g| g.contains(&State::Yes)) {
                return Some(0);
            }
            return Some(1);
        }
        if self.groups.len() == 0 {
            return Some(0);
        }
        if self.length.iter().sum::<i64>() > self.groups.iter().map(|g| g.len() as i64).sum::<i64>()
        {
            return Some(0);
        }
        if self
            .groups
            .iter()
            .map(|g| g.iter().filter(|s| **s == State::Yes).count() as i64)
            .sum::<i64>()
            > self.length.iter().sum::<i64>()
        {
            return Some(0);
        }
        None
    }

    fn _find_solutions(&self, cache: &mut HashMap<Case, i64>) -> i64 {
        if let Some(s) = self._trivial_solutions() {
            return s;
        }

        let first_group = self.groups.first().unwrap();
        // remove groups that are too short on the left and right.
        if self.length.first().unwrap() > &(self.groups.first().unwrap().len() as i64) {
            if first_group.contains(&State::Yes) {
                return 0;
            }
            return Case {
                length: self.length.clone(),
                groups: self.groups[1..].to_vec(),
            }
            .solution_count(cache);
        }
        let last_group = self.groups.last().unwrap();
        if self.length.last().unwrap() > &(self.groups.last().unwrap().len() as i64) {
            if last_group.contains(&State::Yes) {
                return 0;
            }
            return Case {
                length: self.length.clone(),
                groups: self.groups[..self.groups.len() - 1].to_vec(),
            }
            .solution_count(cache);
        }

        let mut options = 0;
        // Try not using the first part all
        if !first_group.contains(&State::Yes) {
            options += Case {
                length: self.length.clone(),
                groups: self.groups[1..].to_vec(),
            }
            .solution_count(cache);
        }

        // use the entire first group
        if first_group.len() <= *self.length.first().unwrap() as usize + 1 {
            let mut multiplier = 1;
            if first_group.len() == *self.length.first().unwrap() as usize + 1 {
                if first_group.last().unwrap() == &State::Maybe
                    && first_group.first().unwrap() == &State::Maybe
                {
                    multiplier = 2;
                }
            }
            let part_options = Case {
                length: self.length[1..].to_vec(),
                groups: self.groups[1..].to_vec(),
            }
            .solution_count(cache);

            options += multiplier * part_options;
        } else {
            // use partial first group
            let len = *self.length.first().unwrap() as usize;
            for i in 0..=(first_group.len() - len) {
                let end_ind = i + len;
                match first_group.iter().nth(end_ind) {
                    Some(State::Yes) => {
                        if first_group[i] == State::Yes {
                            break;
                        }
                        continue;
                    }
                    _ => (),
                }

                let mut new_groups = self.groups[1..].to_vec();
                if end_ind + 1 <= first_group.len() - 1 {
                    new_groups.insert(0, first_group[end_ind + 1..].to_vec());
                }
                options += Case {
                    length: self.length[1..].to_vec(),
                    groups: new_groups,
                }
                .solution_count(cache);

                // it is not possible to ignore a YES
                if first_group[i] == State::Yes {
                    break;
                }
            }
        }
        options
    }
}

fn solve(file: &str) -> i64 {
    let cases = file.lines().map(|l| Case::from(l)).collect::<Vec<_>>();

    let mut cache = HashMap::new();

    cases
        .iter()
        .map(|c| c.solution_count(&mut cache))
        .enumerate()
        .inspect(|(i, c)| println!("{} {}", i, c))
        .map(|(_, c)| c)
        .sum::<i64>()
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
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), 21);
    }

    // #[test]
    // fn test_case_1() {
    //     let input = "?#.??#.?#?.??.?????# 1,1,2,1,1,4";
    //     assert_eq!(solve(input), 4);
    // }
    //
    // #[test]
    // fn test_case_2() {
    //     let input = "????##??#.#?? 1,2,1,1";
    //     assert_eq!(solve(input), 3);
    // }
    //
    // #[test]
    // fn test_case_3() {
    //     let input = "?##??#.#?? 1,2,1,1";
    //     assert_eq!(solve(input), 0);
    // }
    //
    // // #[test]
    // fn test_case_4() {
    //     let input = "???#???????##.??. 7,2,1";
    //     assert_eq!(solve(input), 8);
    // }
    // #[test]
    // fn test_case_5() {
    //     let input = "##.?? 1";
    //     assert_eq!(solve(input), 0);
    // }
}
