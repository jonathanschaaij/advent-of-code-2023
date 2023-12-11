use std::{collections::HashSet, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Space {
    Empty,
    Galaxy,
}
impl From<char> for Space {
    fn from(c: char) -> Self {
        match c {
            '.' => Space::Empty,
            '#' => Space::Galaxy,
            _ => panic!("Invalid space"),
        }
    }
}
fn solve(file: &str) -> i64 {
    let mut space = file
        .lines()
        .map(|l| l.chars().map(Space::from).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    //Go through rows and map empty rows
    let mut empty_rows = space
        .iter()
        .enumerate()
        .filter(|(_, row)| row.iter().all(|s| *s == Space::Empty))
        .map(|(i, _)| i as i64)
        .collect::<Vec<_>>();

    // Go through columns and map empty columns
    let mut empty_cols = Vec::new();
    for i in 0..space[0].len() {
        if space.iter().all(|row| row[i] == Space::Empty) {
            empty_cols.push(i as i64);
        }
    }

    let galaxies: Vec<(i64, i64)> = space
        .iter()
        .enumerate()
        .flat_map(|(y, row)| {
            row.iter()
                .enumerate()
                .filter(|(_, s)| **s == Space::Galaxy)
                .map(move |(x, _)| (x as i64, y as i64))
        })
        .collect();

    // Move galaxies
    let mut moved_galaxies = Vec::new();
    for (x, y) in galaxies.iter() {
        let newx = x + empty_cols.iter().filter(|&&c| c < *x).count() as i64;
        let newy = y + empty_rows.iter().filter(|&&r| r < *y).count() as i64;
        moved_galaxies.push((newx, newy));
    }

    let mut total_dist = 0;
    for i in 0..(moved_galaxies.len() - 1) {
        for j in (i + 1)..moved_galaxies.len() {
            let (x1, y1) = moved_galaxies[i];
            let (x2, y2) = moved_galaxies[j];

            // manhattan distance
            let dist = (x1 as i64 - x2 as i64).abs() + (y1 as i64 - y2 as i64).abs();
            total_dist += dist;
        }
    }

    total_dist
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
        let input = "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....";
        assert_eq!(solve(input), 374);
    }
}
