use colored::Colorize;
use std::time::Instant;

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

impl Direction {
    fn all() -> Vec<Direction> {
        vec![
            Direction::Left,
            Direction::Right,
            Direction::Up,
            Direction::Down,
        ]
    }
    fn opposite(&self) -> Direction {
        match self {
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Node {
    position: (i64, i64),
    cost: i64,
    prev_dir: Vec<Direction>,
    path: Vec<(usize, usize)>,
}

impl Node {
    fn new() -> Node {
        Node {
            position: (0, 0),
            cost: -1,
            prev_dir: vec![],
            path: vec![],
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct PosProperties {
    cost: i64,
    prev_three_dir: Vec<Direction>,
}
fn solve(file: &str) -> i64 {
    let grid: Vec<Vec<i64>> = file
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).unwrap() as i64)
                .collect()
        })
        .collect();

    // alternate implementation of dijkstra's algorithm
    let width = grid[0].len() as i64;
    let height = grid.len() as i64;
    let mut tracked = vec![vec![vec![]; width as usize]; height as usize];
    tracked[0][0].push(PosProperties {
        cost: 0,
        prev_three_dir: vec![Direction::Right; 3],
    });

    let target = (width - 1, height - 1);

    let mut end_nodes = Vec::new();
    end_nodes.push(Node {
        position: (0, 0),
        cost: 0,
        prev_dir: vec![Direction::Right; 10],
        path: vec![(0, 0)],
    });

    end_nodes.push(Node {
        position: (0, 0),
        cost: 0,
        prev_dir: vec![Direction::Down; 10],
        path: vec![(0, 0)],
    });

    const MAX_DIST: usize = 10;
    const MIN_DIST: usize = 4;

    loop {
        // Find the node with the lowest cost and remove it from the list
        let index = end_nodes
            .iter()
            .enumerate()
            .min_by_key(|(_, n)| n.cost)
            .unwrap()
            .0;
        let current_node = end_nodes.remove(index);

        for dir in Direction::all() {
            // Check if direction is possible:
            // Do not go backwards
            if current_node.prev_dir.last() == Some(&dir.opposite()) {
                continue;
            }
            // Go at leas MIN_DIST in the same direction
            //if the last 4 are differnt, do not go any direction except the last one
            if current_node.prev_dir[current_node.prev_dir.len() - MIN_DIST..]
                .iter()
                .any(|d| *d != *current_node.prev_dir.last().unwrap())
                && dir != *current_node.prev_dir.last().unwrap()
            {
                continue;
            }
            // Do not go in the same direction too long
            if current_node.prev_dir.len() >= MAX_DIST
                && current_node.prev_dir[current_node.prev_dir.len() - MAX_DIST..]
                    .iter()
                    .all(|d| *d == dir)
            {
                continue;
            }

            // Do not go out of bounds:
            let new_pos = match dir {
                Direction::Left => (current_node.position.0 - 1, current_node.position.1),
                Direction::Right => (current_node.position.0 + 1, current_node.position.1),
                Direction::Up => (current_node.position.0, current_node.position.1 - 1),
                Direction::Down => (current_node.position.0, current_node.position.1 + 1),
            };

            if new_pos.0 < 0 || new_pos.0 >= width || new_pos.1 < 0 || new_pos.1 >= height {
                continue;
            }

            // Check if the new node is already tracked from the same direction
            let mut dir_hist =
                current_node.prev_dir[current_node.prev_dir.len() - (MAX_DIST - 1)..].to_vec();
            dir_hist.push(dir);

            if tracked[new_pos.1 as usize][new_pos.0 as usize]
                .iter()
                .any(|p| p.prev_three_dir == dir_hist)
            {
                continue;
            }

            let new_cost = current_node.cost + grid[new_pos.1 as usize][new_pos.0 as usize];

            // Check if the new node is already found with a lower const
            if tracked[new_pos.1 as usize][new_pos.0 as usize]
                .iter()
                .any(|p| p.cost <= (new_cost - 30))
            {
                continue;
            }

            // Check if the end has been reached
            if new_pos == target {
                // Only valid if the last 4 directions are the same
                if dir_hist[dir_hist.len() - (MIN_DIST)..]
                    .iter()
                    .any(|d| *d != dir)
                {
                    continue;
                }
                println!("Path:");
                for (y, row) in grid.iter().enumerate() {
                    for (x, cell) in row.iter().enumerate() {
                        if current_node.path.contains(&(x, y)) {
                            print!("{}", cell.to_string().green());
                        } else {
                            print!("{}", cell);
                        }
                    }
                    println!();
                }

                return new_cost;
            }

            // Add the new node to the tracked list
            tracked[new_pos.1 as usize][new_pos.0 as usize].push(PosProperties {
                cost: new_cost,
                prev_three_dir: dir_hist.clone(),
            });

            let mut new_path = current_node.path.clone();
            new_path.push((new_pos.0 as usize, new_pos.1 as usize));
            // Add the new node to the end_nodes list
            end_nodes.push(Node {
                position: new_pos,
                cost: new_cost,
                prev_dir: dir_hist,
                path: new_path,
            });
        }
    }
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
        let input = "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533";
        let t0 = Instant::now();
        let result = solve(input);
        println!("Time: {:?}", t0.elapsed());
        assert_eq!(result, 94);
    }

    #[test]
    fn test_different_case() {
        let input = "111111111111
999999999991
999999999991
999999999991
999999999991";
        assert_eq!(solve(input), 71);
    }
}
