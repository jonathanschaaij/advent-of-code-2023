use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpringStatus {
    Good,
    Bad,
    Either,
}

impl From<char> for SpringStatus {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringStatus::Good,
            '#' => SpringStatus::Bad,
            '?' => SpringStatus::Either,
            _ => panic!("Invalid spring status"),
        }
    }
}

fn recurive_group_reduction(
    possible_groups: &Vec<Vec<SpringStatus>>,
    target_group_lengths: &Vec<i64>,
) -> i64 {
    // return the number of possible groups

    println!("Possible groups: {:?}", possible_groups);
    println!("Target group lengths: {:?}", target_group_lengths);

    if target_group_lengths.len() == 0 {
        let contains_bad = possible_groups
            .iter()
            .any(|g| g.iter().any(|s| s == &SpringStatus::Bad));
        if contains_bad {
            return 0;
        }
        return 1;
    }
    if possible_groups.len() == 0 {
        return 0;
    }

    let first_group = possible_groups.first().unwrap();
    let first_len = target_group_lengths.first().unwrap();
    // Remove first and last groups that do not fit
    if first_len > &(first_group.len() as i64) {
        return recurive_group_reduction(&possible_groups[1..].to_vec(), &target_group_lengths);
    }
    if target_group_lengths.last().unwrap() > &(possible_groups.last().unwrap().len() as i64) {
        return recurive_group_reduction(
            &possible_groups[..possible_groups.len() - 1].to_vec(),
            &target_group_lengths,
        );
    }

    // try first group on first possible
    let mut options = 0;
    if first_group.len() == *first_len as usize {
        options += recurive_group_reduction(
            &possible_groups[1..].to_vec(),
            &target_group_lengths[1..].to_vec(),
        );
    } else if first_group.len() == *first_len as usize + 1 {
        let new_options = recurive_group_reduction(
            &possible_groups[1..].to_vec(),
            &target_group_lengths[1..].to_vec(),
        );

        if first_group.first().unwrap() == &SpringStatus::Either
            && possible_groups.first().unwrap().last().unwrap() == &SpringStatus::Either
        {
            options += 2 * new_options;
        } else {
            options += new_options;
        }
    } else {
        // Try all possible positions of the first group
        for i in 0..=(first_group.len() - *first_len as usize) {
            let end_ind = i + *first_len as usize;

            if end_ind < first_group.len() && first_group[end_ind] == SpringStatus::Bad {
                if first_group[i] == SpringStatus::Bad {
                    break;
                }
                continue;
            }

            let mut new_possible_groups = possible_groups[1..].to_vec();
            if end_ind < first_group.len() {
                let new_first_group = first_group[(end_ind + 1)..].to_vec();
                // println!("new_first_group: {:?}", new_first_group);
                new_possible_groups.insert(0, new_first_group);
            }
            options +=
                recurive_group_reduction(&new_possible_groups, &target_group_lengths[1..].to_vec());

            if first_group[i] == SpringStatus::Bad {
                println!("Possible groups: {:?}", possible_groups);
                println!("Target group lengths: {:?}", target_group_lengths);
                println!("BREAKING");
                break;
            }
        }
    }
    if !first_group.contains(&SpringStatus::Bad) {
        options += recurive_group_reduction(
            &possible_groups[1..].to_vec(),
            &target_group_lengths.to_vec(),
        );
    }
    println!("Options: {}", options);
    options
}

fn solve(file: &str) -> i64 {
    file.lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            let map = parts
                .next()
                .unwrap()
                .chars()
                .map(|c| SpringStatus::from(c))
                .collect::<Vec<_>>();

            let groups = parts
                .next()
                .unwrap()
                .split(',')
                .map(|s| s.parse::<i64>().unwrap())
                .collect::<Vec<_>>();

            // Expand by repeating 5 times
            // let groups = groups
            //     .iter()
            //     .cycle()
            //     .take(groups.len() * 5)
            //     .cloned()
            //     .collect::<Vec<_>>();
            //
            // let mut expanded_map = vec![];
            // for i in 0..5 {
            //     if i != 0 {
            //         expanded_map.push(SpringStatus::Either);
            //     }
            //     expanded_map.extend(map.iter());
            // }
            //
            let mut inner_groups = vec![vec![]];
            for s in map.iter() {
                match s {
                    SpringStatus::Good => {
                        if inner_groups.last().unwrap().len() > 0 {
                            inner_groups.push(vec![]);
                        }
                    }
                    SpringStatus::Bad => {
                        inner_groups.last_mut().unwrap().push(SpringStatus::Bad);
                    }
                    SpringStatus::Either => {
                        inner_groups.last_mut().unwrap().push(SpringStatus::Either);
                    }
                }
            }

            let num = recurive_group_reduction(&inner_groups, &groups);
            println!("{} \t\tnum: {}", line, num);
            num
        })
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

    // #[test]
    fn test_whole_part() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1";
        assert_eq!(solve(input), 525152);
    }

    #[test]
    fn test_whole_part2() {
        let input = "????##??#.#?? 1,2,1,1";
        assert_eq!(solve(input), 3);
    }
}
