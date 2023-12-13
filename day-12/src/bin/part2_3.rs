use std::{collections::HashMap, time::Instant};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
enum State {
    No,
    Yes,
    Maybe,
}

impl From<char> for State {
    fn from(c: char) -> Self {
        match c {
            '.' => State::No,
            '#' => State::Yes,
            '?' => State::Maybe,
            _ => panic!("Invalid char"),
        }
    }
}

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct Case {
    length: Vec<i64>,
    springs: Vec<State>,
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

        let mut groups = row.clone();
        groups.insert(0, State::No);

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

        Case {
            length,
            springs: groups,
        }
    }
}

impl Case {
    fn solution_count(&self) -> i64 {
        // Dynamic programming variables
        // This variable counts the number of possible solutions given all previous solutions are
        // valid. It the index corresponds to the number of possible solutions
        let mut dp = vec![0; self.springs.len() + 1];
        dp[0] = 1;

        // If no length is required, then any string of only ? and . are valid.
        for (i, s) in self.springs.iter().enumerate() {
            if *s == State::Yes {
                break;
            }
            dp[i + 1] = 1;
        }

        // Consider off required groups one at a time.
        for len in self.length.clone() {
            let mut new_dp = vec![0; self.springs.len() + 1];

            let mut current_group_len = 0;

            for (i, s) in self.springs.iter().enumerate() {
                if *s == State::No {
                    current_group_len = 0;
                } else {
                    current_group_len += 1;
                }

                // If the current spring is not required, it could be ignored, which leaves the
                // number of solutions unchanged compared to the previous state.
                if *s != State::Yes {
                    new_dp[i + 1] = new_dp[i];
                }

                // If the current section (from i-len -> i) is possible, then the number increases
                // by the number of solutions possible with all previous groups valid
                if current_group_len >= len && self.springs[i - len as usize] != State::Yes {
                    new_dp[i + 1] += dp[i - len as usize];
                }
            }
            dp = new_dp;
        }

        *dp.last().unwrap()
    }
}

fn solve(file: &str) -> i64 {
    let cases = file.lines().map(|l| Case::from(l)).collect::<Vec<_>>();

    cases
        .iter()
        .map(|c| c.solution_count())
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
        assert_eq!(solve(input), 525152);
    }

    // #[test]
    // fn test_case_1() {
    // let input = "?#.??#.?#?.??.?????# 1,1,2,1,1,4";
    // assert_eq!(solve(input), 4);
    // }

    // #[test]
    // fn test_case_2() {
    // let input = "????##??#.#?? 1,2,1,1";
    // assert_eq!(solve(input), 3);
    // }

    // #[test]
    // fn test_case_3() {
    // let input = "?##??#.#?? 1,2,1,1";
    // assert_eq!(solve(input), 0);
    // }

    // #[test]
    // fn test_case_4() {
    // let input = "???#???????##.??. 7,2,1";
    // assert_eq!(solve(input), 8);
    // }
    // #[test]
    // fn test_case_5() {
    // let input = "##.?? 1";
    // assert_eq!(solve(input), 0);
    // }
    #[test]
    fn test_case_6() {
        let input = "? 1"; // -> ? ? ? ? ? ? ? ? ? 1,1,1,1,1
        assert_eq!(solve(input), 1);
    }

    #[test]
    fn test_case_7() {
        let input = "#? 1"; // -> #? 1,1,1,1,1
        assert_eq!(solve(input), 1);
    }
    #[test]
    fn test_case_8() {
        let input = "## 1"; // -> #? 1,1,1,1,1
        assert_eq!(solve(input), 0);

        let input = "##???...??? 2";
        assert_eq!(solve(input), 1);
    }
    #[test]
    fn test_case_11() {
        let input = "##?. 3"; // -> ##?.?##?.?##?.?##?.?##?
        assert_eq!(solve(input), 16);
    }
    #[test]
    fn test_case_9() {
        let input = "?. 1"; // -> ?.??.??.??.??. 1,1,1,1,1
        assert_eq!(solve(input), 16);
    }
    #[test]
    fn test_case_10() {
        let input = ".. 1"; // -> ..?..?..?..?.. 1,1,1,1,1
        assert_eq!(solve(input), 0);
    }

    #[test]
    fn test_case_12() {
        let input = "..???.. 2"; // -> ???.???.???.???.???. 1,1,1,1,1
        assert_eq!(solve(input), 32);
    }
}
