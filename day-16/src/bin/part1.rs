use std::time::Instant;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum GridItem {
    Empty,
    SplitVertical,
    SplitHoriontal,
    MirrorLeftUp,   // Same as MirrorRightDown
    MirrorLeftDown, // Same as MirrorRightUp
}
impl From<char> for GridItem {
    fn from(c: char) -> Self {
        match c {
            '.' => GridItem::Empty,
            '|' => GridItem::SplitVertical,
            '-' => GridItem::SplitHoriontal,
            '/' => GridItem::MirrorLeftUp,
            '\\' => GridItem::MirrorLeftDown,
            _ => panic!("Invalid grid item: {}", c),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn to_vec(&self) -> (i64, i64) {
        match self {
            Direction::Up => (0, -1),
            Direction::Down => (0, 1),
            Direction::Left => (-1, 0),
            Direction::Right => (1, 0),
        }
    }

    fn opposite(&self) -> Self {
        match self {
            Direction::Up => Direction::Down,
            Direction::Down => Direction::Up,
            Direction::Left => Direction::Right,
            Direction::Right => Direction::Left,
        }
    }
}

struct Light {
    position: (i64, i64),
    direction: Direction,
}

fn solve(file: &str) -> i64 {
    let grid: Vec<Vec<GridItem>> = file
        .lines()
        .map(|line| line.chars().map(|c| c.into()).collect())
        .collect();
    let width = grid[0].len();
    let height = grid.len();

    let mut light_directions: Vec<Vec<Vec<Direction>>> = vec![vec![vec![]; width]; height];
    let mut light_endpoints: Vec<Light> = Vec::new();
    light_endpoints.push(Light {
        position: (0, 0),
        direction: Direction::Right,
    });

    while let Some(mut light) = light_endpoints.pop() {
        // Trace the endpoint, until it can go no further
        loop {
            let (x, y) = light.position;
            //Check out of bounds
            if x >= width as i64 || y >= height as i64 || x < 0 || y < 0 {
                break;
            }
            let (x, y) = (x as usize, y as usize);

            // check if it had already been there
            if light_directions[y][x].contains(&light.direction) {
                break;
            }
            // The direction does not matter for splitter
            if grid[y][x] == GridItem::SplitHoriontal || grid[y][x] == GridItem::SplitVertical {
                if !light_directions[y][x].is_empty() {
                    break;
                }
            }
            // Mirror are not symmetrical, light from left behaves differently than light from right
            if grid[y][x] != GridItem::MirrorLeftUp && grid[y][x] != GridItem::MirrorLeftDown {
                if light_directions[y][x].contains(&light.direction.opposite()) {
                    break;
                }
            }

            light_directions[y][x].push(light.direction);

            // Move the light to the next grid position
            let current_grid_item = &grid[y][x];
            match current_grid_item {
                GridItem::Empty => {
                    let (dx, dy) = light.direction.to_vec();
                    light.position = (x as i64 + dx, y as i64 + dy);
                }
                GridItem::SplitHoriontal => {
                    // Create new endpoint to the left
                    light_endpoints.push(Light {
                        position: (x as i64 - 1, y as i64),
                        direction: Direction::Left,
                    });
                    // Move the current light to the right
                    light.position = (x as i64 + 1, y as i64);
                    light.direction = Direction::Right;
                }
                GridItem::SplitVertical => {
                    // Create new endpoint to the top
                    light_endpoints.push(Light {
                        position: (x as i64, y as i64 - 1),
                        direction: Direction::Up,
                    });
                    // Move the current light to the bottom
                    light.position = (x as i64, y as i64 + 1);
                    light.direction = Direction::Down;
                }
                GridItem::MirrorLeftDown => {
                    light.direction = match light.direction {
                        Direction::Up => Direction::Left,
                        Direction::Down => Direction::Right,
                        Direction::Left => Direction::Up,
                        Direction::Right => Direction::Down,
                    };
                    let (dx, dy) = light.direction.to_vec();
                    light.position = (x as i64 + dx, y as i64 + dy);
                }
                GridItem::MirrorLeftUp => {
                    light.direction = match light.direction {
                        Direction::Up => Direction::Right,
                        Direction::Down => Direction::Left,
                        Direction::Left => Direction::Down,
                        Direction::Right => Direction::Up,
                    };
                    let (dx, dy) = light.direction.to_vec();
                    light.position = (x as i64 + dx, y as i64 + dy);
                }
            }
        }
    }

    // Count grid positions that have been visited by light
    light_directions
        .iter()
        .map(|row| row.iter().filter(|v| !v.is_empty()).count() as i64)
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
        let input = ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....";
        assert_eq!(solve(input), 46);
    }
}
