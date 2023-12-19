use std::time::Instant;

/* Small illustration of the problem

#######..######
#.....#..#....#
#.....####....#
#.............#
###...#########
..#...#
..#...#
..#...#
###.###
#...#..
#...#..
#...#..
##..###
.#....#
.######
*/

enum Orientation {
    Horizontal,
    Vertical,
}

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

#[derive(Debug, Clone, Copy)]
struct Edge {
    start: (i64, i64),
    end: (i64, i64),
    dir: Direction,
}

fn count_internal_at_height(height: i64, vertical: &Vec<Edge>, horizontal: &Vec<Edge>) -> i64 {
    let cur_horizontal_edges = horizontal
        .iter()
        .filter(|e| e.start.1 == height)
        .collect::<Vec<_>>();
    let cur_vertical_edges = vertical
        .iter()
        .filter(|e| e.start.1 <= height && e.end.1 > height)
        .collect::<Vec<_>>();

    // println!("y: {} \thorizontal: {:?}", height, cur_horizontal_edges);
    // println!("y: {} \tvertical: {:?}", height, cur_vertical_edges);
    let mut ranges = vec![];
    let mut start = None::<i64>;

    for vertical_edge in &cur_vertical_edges {
        if start.is_none() {
            start = Some(vertical_edge.start.0);
        } else {
            ranges.push((start.unwrap() + 1)..vertical_edge.start.0);
            start = None;
        }
    }

    for horizontal_edge in &cur_horizontal_edges {
        let mut new_ranges = vec![];
        for r in ranges {
            if r.is_empty() {
                //This should never happen
                panic!(
                    "Empty range\ny: {} \nhorizontal: {:?}\nvertical: {:?}",
                    height, cur_horizontal_edges, cur_vertical_edges
                );
            }

            if r.contains(&horizontal_edge.start.0) {
                new_ranges.push(r.start..horizontal_edge.start.0);
            }
            if r.contains(&horizontal_edge.end.0) {
                new_ranges.push((horizontal_edge.end.0 + 1)..r.end);
            }
            if !r.contains(&horizontal_edge.start.0) && !r.contains(&horizontal_edge.end.0) {
                new_ranges.push(r);
            }
        }
        ranges = new_ranges;
    }
    // println!("y: {} \tranges: {:?}", height, ranges);
    ranges.iter().map(|r| r.end - r.start).sum()
}

fn solve(file: &str) -> i64 {
    let mut pos = (0, 0);
    let mut edges = vec![];
    let mut count = 0;

    file.lines().for_each(|line| {
        let parts = line.split(" ").collect::<Vec<_>>();
        // let chars = parts[2].chars().collect::<Vec<char>>();
        // let dir = Direction::from_str(&chars[7].to_string()).unwrap();
        // let hex = chars[2..7].iter().collect::<String>();
        // let dist = i64::from_str_radix(&hex, 16).unwrap();

        let dir = Direction::from_str(parts[0]).unwrap();
        let dist = parts[1].parse::<i64>().unwrap();

        let orientation = match dir {
            Direction::Up | Direction::Down => Orientation::Vertical,
            Direction::Left | Direction::Right => Orientation::Horizontal,
        };

        let new_pos = match orientation {
            Orientation::Horizontal => (pos.0 + dir.to_vec().0 * dist, pos.1),
            Orientation::Vertical => (pos.0, pos.1 + dir.to_vec().1 * dist),
        };

        count += dist;
        match dir {
            //This ensures that the start < end
            Direction::Up | Direction::Left => {
                edges.push(Edge {
                    start: new_pos,
                    end: pos,
                    dir,
                });
            }
            Direction::Down | Direction::Right => {
                edges.push(Edge {
                    start: pos,
                    end: new_pos,
                    dir,
                });
            }
        }
        pos = new_pos;
    });

    println!("Final pos: {:?}", pos);
    assert_eq!(pos, (0, 0));
    println! {"N_edge: {}, total_dist: {}", edges.len(), count};

    let (mut horizontal, mut vertical): (Vec<Edge>, Vec<Edge>) =
        edges.iter().partition(|e| match e.dir {
            Direction::Up | Direction::Down => false,
            Direction::Left | Direction::Right => true,
        });

    horizontal.sort_by(|a, b| a.start.1.cmp(&b.start.1));
    vertical.sort_by(|a, b| a.start.0.cmp(&b.start.0));

    let mut relevant_heights = horizontal.iter().map(|e| e.start.1).collect::<Vec<_>>();
    relevant_heights.dedup();

    for heights in relevant_heights.windows(2) {
        let (y_prev, y) = (heights[0], heights[1]);
        if y_prev + 1 < y {
            let num_rows = y - y_prev - 1;
            let add = num_rows * count_internal_at_height(y_prev + 1, &vertical, &horizontal);
            count += add;
        }

        let add = count_internal_at_height(y, &vertical, &horizontal);
        count += add;
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
