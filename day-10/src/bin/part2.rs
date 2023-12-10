use std::{fmt, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq)]
enum Direction {
    North,
    South,
    East,
    West,
}
impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::North,
            Direction::South,
            Direction::East,
            Direction::West,
        ]
    }

    fn matching_dir(&self) -> Direction {
        match self {
            Direction::North => Direction::South,
            Direction::South => Direction::North,
            Direction::East => Direction::West,
            Direction::West => Direction::East,
        }
    }

    fn to_vec(&self) -> (i64, i64) {
        match self {
            Direction::North => (0, -1),
            Direction::South => (0, 1),
            Direction::East => (1, 0),
            Direction::West => (-1, 0),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum PipeElements {
    Ground,
    Start,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
    ENCLOSED,
}

impl fmt::Debug for PipeElements {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            PipeElements::Ground => write!(f, "."),
            PipeElements::Start => write!(f, "S"),
            PipeElements::Vertical => write!(f, "|"),
            PipeElements::Horizontal => write!(f, "-"),
            PipeElements::BendNE => write!(f, "L"),
            PipeElements::BendNW => write!(f, "J"),
            PipeElements::BendSE => write!(f, "F"),
            PipeElements::BendSW => write!(f, "7"),
            PipeElements::ENCLOSED => write!(f, "█"),
        }
    }
}

impl PipeElements {
    fn get_directions(&self) -> Vec<Direction> {
        match self {
            PipeElements::Ground => vec![],
            PipeElements::Start => Direction::all(),
            PipeElements::Vertical => vec![Direction::North, Direction::South],
            PipeElements::Horizontal => vec![Direction::East, Direction::West],
            PipeElements::BendNE => vec![Direction::North, Direction::East],
            PipeElements::BendNW => vec![Direction::North, Direction::West],
            PipeElements::BendSE => vec![Direction::South, Direction::East],
            PipeElements::BendSW => vec![Direction::South, Direction::West],
            PipeElements::ENCLOSED => vec![],
        }
    }
}

impl From<char> for PipeElements {
    fn from(c: char) -> Self {
        match c {
            '.' => PipeElements::Ground,
            '|' => PipeElements::Vertical,
            '-' => PipeElements::Horizontal,
            'S' => PipeElements::Start,
            'L' => PipeElements::BendNE,
            'J' => PipeElements::BendNW,
            'F' => PipeElements::BendSE,
            '7' => PipeElements::BendSW,
            _ => panic!("Invalid character"),
        }
    }
}

fn solve(file: &str) -> i64 {
    let grid: Vec<Vec<PipeElements>> = file
        .lines()
        .map(|line| line.chars().map(|c| PipeElements::from(c)).collect())
        .collect();

    // Find Starting pos
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == PipeElements::Start {
                start_pos = (x, y);
            }
        }
    }

    //Map with connected pipe
    let w = grid[0].len();
    let h = grid.len();
    let mut map = vec![vec![PipeElements::Ground; w]; h];

    // Loop to starting pos
    let mut min_pos = (w, h);
    let mut max_pos = (0, 0);
    let mut steps = 0;
    let mut current_pos = start_pos;
    let mut directions = Direction::all();
    loop {
        if current_pos == start_pos && steps > 0 {
            break;
        }
        if directions.len() == 0 {
            panic!("No directions found");
        }

        let mut next_dirs: Option<Vec<Direction>> = None;
        for dir in &directions {
            let next_pos = (
                (current_pos.0 as i64 + dir.to_vec().0) as usize,
                (current_pos.1 as i64 + dir.to_vec().1) as usize,
            );
            let next_elem = grid[next_pos.1 as usize][next_pos.0 as usize];
            let next_dir = next_elem.get_directions();
            if next_dir.contains(&dir.matching_dir()) {
                if next_pos.0 < min_pos.0 {
                    min_pos.0 = next_pos.0;
                }
                if next_pos.1 < min_pos.1 {
                    min_pos.1 = next_pos.1;
                }
                if next_pos.0 > max_pos.0 {
                    max_pos.0 = next_pos.0;
                }
                if next_pos.1 > max_pos.1 {
                    max_pos.1 = next_pos.1;
                }
                map[next_pos.1 as usize][next_pos.0 as usize] = next_elem;
                current_pos = next_pos;
                steps += 1;
                next_dirs = Some(
                    next_dir
                        .iter()
                        .filter(|d| d != &&dir.matching_dir())
                        .cloned()
                        .collect::<Vec<_>>(),
                );
                break; // Break out of directions loop
            }
        }
        directions = next_dirs.unwrap();
    }

    // Now count the walls for every element which could be enclosed towards the left and right
    // Odd -> inside, Even -> outside
    assert!(min_pos.0 < max_pos.0);
    assert!(min_pos.1 < max_pos.1);
    let mut enclosed = 0;
    for y in (min_pos.1 + 1)..max_pos.1 {
        for x in (min_pos.0 + 1)..max_pos.0 {
            let elem = map[y][x];
            if elem != PipeElements::Ground {
                // Part of pipe does not count
                continue;
            }
            // Check left side for vertical pipeWalls
            let mut check_pos = (x, y);
            let mut wall_count_left = 0;
            let mut wall_count_top = 0;
            let mut connections_a = 0;
            let mut connections_b = 0;
            loop {
                check_pos = (check_pos.0 - 1, check_pos.1);
                let check_elem = map[check_pos.1][check_pos.0];

                // if check_elem == PipeElements::Vertical {
                // wall_count_left += 1;
                // }
                if check_elem.get_directions().contains(&Direction::North) {
                    connections_a += 1;
                }
                if check_elem.get_directions().contains(&Direction::South) {
                    connections_b += 1;
                }
                if check_pos.0 <= min_pos.0 {
                    break;
                }
            }
            wall_count_left += connections_a.min(connections_b);
            connections_a = 0;
            connections_b = 0;
            check_pos = (x, y);
            loop {
                check_pos = (check_pos.0, check_pos.1 - 1);
                let check_elem = map[check_pos.1][check_pos.0];

                // if check_elem == PipeElements::Horizontal {
                //     wall_count_top += 1;
                // }
                if check_elem.get_directions().contains(&Direction::East) {
                    connections_a += 1;
                }
                if check_elem.get_directions().contains(&Direction::West) {
                    connections_b += 1;
                }
                if check_pos.1 <= min_pos.1 {
                    break;
                }
            }
            wall_count_top += connections_a.min(connections_b);

            if wall_count_left % 2 == 1 && wall_count_top % 2 == 1 {
                map[y][x] = PipeElements::ENCLOSED;
                println! {"Enclosed: ({}, {})", x, y};
                enclosed += 1;
            }
        }
    }
    println! {"MAP:\t\tmin({:?}) max({:?})", min_pos, max_pos};
    for row in &map {
        for elem in row {
            print!("{:?}", elem);
        }
        println!();
    }
    enclosed
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
        let input = ".F----7F7F7F7F-7....
.|F--7||||||||FJ....
.||.FJ||||||||L7....
FJL7L7LJLJ||LJ.L-7..
L--J.L7...LJS7F-7L7.
....F-J..F7FJ|L7L7L7
....L7.F7||L7|.L7L7|
.....|FJLJ|FJ|F7|.LJ
....FJL-7.||.||||...
....L---J.LJ.LJLJ...";
        assert_eq!(solve(input), 8);
    }
}
