use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq)]
enum SpringStatus {
    Operational,
    Damaged,
    Unknown,
}

impl From<char> for SpringStatus {
    fn from(c: char) -> Self {
        match c {
            '.' => SpringStatus::Operational,
            '#' => SpringStatus::Damaged,
            '?' => SpringStatus::Unknown,
            _ => panic!("Invalid spring status"),
        }
    }
}

fn count_groups(map: &Vec<SpringStatus>) -> Vec<i64> {
    let mut count: i64 = 0;
    let mut groups = vec![];
    for s in map {
        match s {
            SpringStatus::Operational => {
                if count > 0 {
                    groups.push(count);
                    count = 0;
                }
            }
            SpringStatus::Damaged => count += 1,
            SpringStatus::Unknown => {
                if count > 0 {
                    groups.push(count);
                    count = 0;
                }
            }
        }
    }
    if count > 0 {
        groups.push(count);
    }
    groups
}

fn count_solutions(
    target_groups: &Vec<i64>,
    current_map: Vec<SpringStatus>,
    unknown_ind: &Vec<i64>,
    missing: i64,
) -> i64 {
    if missing <= 0 {
        //Check whether configuration is valid
        let groups = count_groups(&current_map);
        if groups == *target_groups {
            return 1;
        } else {
            return 0;
        }
    }
    if unknown_ind.len() < missing as usize {
        //Not enough unknowns to fill
        return 0;
    }

    // println!(
    //     "Groups:{:?}\t\tMissing:{}\n{:?}",
    //     target_groups, missing, current_map
    // );

    let mut solutions = 0;
    let mut new_unknown_ind = unknown_ind.clone();
    while new_unknown_ind.len() >= missing as usize {
        let mut new_map = current_map.clone();
        new_map[new_unknown_ind[0] as usize] = SpringStatus::Damaged;
        // println!("New map: {:?}", new_map);
        new_unknown_ind.remove(0);
        solutions += count_solutions(target_groups, new_map, &new_unknown_ind, missing - 1);
    }
    solutions
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

            let unknown_pos = map
                .iter()
                .enumerate()
                .filter(|(_, s)| **s == SpringStatus::Unknown)
                .map(|(i, _)| i as i64)
                .collect::<Vec<i64>>();

            let known_pos = map
                .iter()
                .enumerate()
                .filter(|(_, s)| **s == SpringStatus::Damaged)
                .map(|(i, _)| i)
                .collect::<Vec<_>>();

            let missing: i64 = groups.iter().sum::<i64>() - known_pos.len() as i64;
            // println!(
            // "Missing: {}\tDamaged: {}\tTotal:{}",
            //     missing,
            //     known_pos.len(),
            //     groups.iter().sum::<i64>()
            // );

            let num = count_solutions(&groups, map, &unknown_pos, missing);
            // println!("{}, \t num: {}", line, num);
            println!("{}", num);

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

    #[test]
    fn test_whole_part() {
        let input = "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1
?.???...?? 1,1,1";
        assert_eq!(solve(input), 30);
    }
}
