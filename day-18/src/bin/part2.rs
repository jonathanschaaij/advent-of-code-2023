use std::time::Instant;

#[derive(Debug, Clone, Copy)]
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
            "0" => Some(Direction::Right),
            "1" => Some(Direction::Down),
            "2" => Some(Direction::Left),
            "3" => Some(Direction::Up),
            _ => None,
        }
    }

    fn to_vec(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }
}

fn solve(file: &str) -> i64 {
    let mut pos = (0, 0);
    let mut edges = vec![];
    let mut points = vec![];
    points.push(pos);
    let mut enclosed_area: i64 = 0;
    let mut total_dist = 0;

    file.lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        let chars = parts[2].chars().collect::<Vec<char>>();
        let dir = Direction::from_str(&chars[7].to_string()).unwrap();
        let hex = chars[2..7].iter().collect::<String>();
        let dist = i64::from_str_radix(&hex, 16).unwrap();

        // let dir = Direction::from_str(parts[0]).unwrap();
        // let dist = parts[1].parse::<i64>().unwrap();

        total_dist += dist;

        let new_pos = match dir {
            Direction::Right | Direction::Left => (pos.0 + dir.to_vec().0 * dist, pos.1),
            Direction::Down | Direction::Up => (pos.0, pos.1 + dir.to_vec().1 * dist),
        };

        match dir {
            //This ensures that the start < end
            Direction::Up | Direction::Left => {
                edges.push((new_pos, pos));
            }
            Direction::Down | Direction::Right => {
                edges.push((pos, new_pos));
            }
        }
        let (x1, y1) = pos;
        let (x2, y2) = new_pos;
        enclosed_area += (y2 + y1) * (x1 - x2);
        pos = new_pos;
        points.push(pos);
    });
    enclosed_area.abs() / 2 + total_dist / 2 + 1
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
    #[test]
    fn test_custom_case_1() {
        let input = "R 2 aaa\nD 2 aaa\nR 2 aaa\nU 2 aaa\nR 2 aaa\nD 3 aaa\nL 6 aaa\nU 3 aaa";
        assert_eq!(solve(input), 26);
    }

    #[test]
    fn test_custom_case_3() {
        let input = "R 2 aaa\nU 2 aaa\nR 2 aaa\nD 2 aaa\nR 2 aaa\nU 3 aaa\nL 6 aaa\nD 3 aaa";
        assert_eq!(solve(input), 26);
    }
    #[test]
    fn test_custom_case_2() {
        let input = "R 9 aaa\nD 8 aaa\nR 3 aaa\nD 7 aaa\nL 4 aaa\nU 4 aaa\nL 4 aaa\nD 4 aaa\nL 4 aaa\nU 6 aaa\nR 3 aaa\nU 5 aaa\nL 3 aaa\nU 4 aaa";
        assert_eq!(solve(input), 160);
    }
    #[test]
    fn test_custom_case_4() {
        let input = "R 2 aaa\nU 2 aaa\nR 3 aaa\nD 3 aaa\nL 2 aaa\nD 2 aaa\nL 3 aaa\nU 3 aaa\n";
        assert_eq!(solve(input), 28);
    }
}
