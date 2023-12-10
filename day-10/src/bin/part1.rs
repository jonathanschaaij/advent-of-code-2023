use std::time::Instant;

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

#[derive(Debug, Clone, Copy, PartialEq)]
enum PipeElements {
    Ground,
    Start,
    Vertical,
    Horizontal,
    BendNE,
    BendNW,
    BendSE,
    BendSW,
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
    let mut grid: Vec<Vec<PipeElements>> = file
        .lines()
        .map(|line| line.chars().map(|c| PipeElements::from(c)).collect())
        .collect();

    // Find Starting pos
    let mut start_pos = (0, 0);
    for (y, row) in grid.iter().enumerate() {
        for (x, elem) in row.iter().enumerate() {
            if *elem == PipeElements::Start {
                start_pos = (x as i64, y as i64);
            }
        }
    }

    // Loop to starting pos
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
                current_pos.0 + dir.to_vec().0,
                current_pos.1 + dir.to_vec().1,
            );
            let next_elem = grid[next_pos.1 as usize][next_pos.0 as usize];
            let next_dir = next_elem.get_directions();
            if next_dir.contains(&dir.matching_dir()) {
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
    steps / 2
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
        let input = "..F7.
.FJ|.
SJ.L7
|F--J
LJ...";
        assert_eq!(solve(input), 8);
    }
}
