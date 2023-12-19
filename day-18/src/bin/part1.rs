use std::time::Instant;

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn from_str(s: &str) -> Option<Direction> {
        match s {
            "U" => Some(Direction::Up),
            "D" => Some(Direction::Down),
            "L" => Some(Direction::Left),
            "R" => Some(Direction::Right),
            _ => None,
        }
    }

    fn to_vec(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, 1),
            Direction::Down => (0, -1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn enclosed(pos: (i64, i64), grid: &Vec<Vec<bool>>) -> bool {
    //count wall while going up
    if grid[pos.1 as usize][pos.0 as usize] {
        return true;
    }
    if pos.1 == grid.len() as i64 - 1 {
        return false;
    }
    let wall_count = grid[pos.1 as usize][..pos.0 as usize]
        .iter()
        .enumerate()
        .filter(|(i, &b)| (b && grid[pos.1 as usize + 1][*i]))
        .count();

    // println!("wall_count: {}", wall_count);
    wall_count % 2 == 1
}

fn solve(file: &str) -> i64 {
    let mut edge_cords = vec![];
    let mut pos = (0, 0);
    edge_cords.push(pos);
    let mut x_range = (0, 0); // (min, max)
    let mut y_range = (0, 0); // (min, max)

    file.lines().for_each(|line| {
        let mut parts = line.splitn(3, " ");
        let dir = Direction::from_str(parts.next().unwrap()).unwrap();
        let dist = parts.next().unwrap().parse::<i64>().unwrap();
        // let color = parts.next().unwrap().parse::<i64>().unwrap();
        for _ in 0..dist {
            let (x, y) = dir.to_vec();
            pos = (pos.0 + x, pos.1 + y);
            edge_cords.push(pos);

            // Update ranges
            if pos.0 < x_range.0 {
                x_range.0 = pos.0;
            }
            if pos.0 > x_range.1 {
                x_range.1 = pos.0;
            }
            if pos.1 < y_range.0 {
                y_range.0 = pos.1;
            }
            if pos.1 > y_range.1 {
                y_range.1 = pos.1;
            }
        }
    });

    let mut grid = vec![
        vec![false; (x_range.1 + 1 - x_range.0) as usize];
        (y_range.1 + 1 - y_range.0) as usize
    ];
    for (x, y) in edge_cords {
        grid[(y - y_range.0) as usize][(x - x_range.0) as usize] = true;
    }

    let mut count = 0;
    for y in 0..grid.len() {
        let mut currently_enclosed = false;
        for x in 0..grid[y].len() {
            if grid[y][x] {
                count += 1;
                if y + 1 < grid.len() && grid[y + 1][x] {
                    currently_enclosed = !currently_enclosed;
                }
                continue;
            }
            if currently_enclosed {
                count += 1;
            }
        }
    }

    count
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
        let input = "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)";
        assert_eq!(solve(input), 62);
    }
}
