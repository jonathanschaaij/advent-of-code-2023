use std::time::Instant;

fn find_symetry_col(chunk: &str) -> Option<i64> {
    let lines = chunk.lines().collect::<Vec<_>>();

    let mut mistakes = vec![0; lines[0].len()];
    for l in lines {
        let len = l.len();

        for i in 1..len {
            let first_halve: &str;
            let second_halve: &str;
            if i <= len / 2 {
                first_halve = &l[..i];
                second_halve = &l[i..i + i];
            } else {
                first_halve = &l[i - (len - i)..i];
                second_halve = &l[i..];
            }
            let rev_second = second_halve.chars().rev().collect::<String>();

            for (n, c) in first_halve.chars().enumerate() {
                if c != rev_second.chars().nth(n).unwrap() {
                    mistakes[i] += 1;
                }
            }
        }
    }
    println!("{:?}", mistakes);
    mistakes
        .iter()
        .enumerate()
        .find(|(_, m)| **m == 1)
        .map(|(i, _)| i as i64)
}

fn find_symetry_row(chunk: &str) -> Option<i64> {
    // transpose lines to columns and combine into a string.
    let lines = chunk
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut new_chunk = vec![];
    for col in 0..lines[0].len() {
        for row in 0..lines.len() {
            new_chunk.push(lines[row][col]);
        }
        new_chunk.push('\n');
    }

    let newstr = new_chunk.iter().collect::<String>();
    println!("Original chunk:");
    println!("{}", chunk);
    println!("Transposed chunk:");
    println!("{}", newstr);
    return find_symetry_col(&newstr);
}

fn solve(file: &str) -> i64 {
    let chunks: Vec<_> = file.split("\n\n").collect();

    let mut out = 0;
    for chunk in chunks {
        if let Some(c) = find_symetry_col(chunk) {
            out += c;
        } else if let Some(r) = find_symetry_row(chunk) {
            out += 100 * r;
        } else {
            println!("{}", chunk);
            panic!("No symetry found");
        }
    }
    out
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
        let input = "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#";
        assert_eq!(solve(input), 400);
    }

    // #[test]
    fn test_symetry_row() {
        let input = "#.##..#..######..
....##.##..##..##
###...##..#..#..#
###...##..#..#..#
....##.##..##..##
#.##..#..######..
.#..###.###..###.
####...###...####
######..########.
.#..#....#.##.#..
##...###.#....#.#";
        assert_eq!(find_symetry_row(input), Some(3));
    }

    // #[test]
    fn test_symetry_row_2() {
        let input = ".......##..##
..##...#.#...
.#..#...#.###
.####...#..##
##..##.####.#
#.##.#.#.....
.......#.##..
..##..#.#.###
......#..#..#
.......#.##.#
......#..#.##
#...##..#..##
.####.#.#..##
.####.#.##...
.####.#.##...";
        assert_eq!(find_symetry_row(input), Some(14));
    }
}
