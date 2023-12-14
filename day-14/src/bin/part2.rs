use std::{collections::HashMap, fmt, time::Instant};

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Rocks {
    Rolling,
    Solid,
    Empty,
}

impl From<char> for Rocks {
    fn from(c: char) -> Self {
        match c {
            'O' => Rocks::Rolling,
            '#' => Rocks::Solid,
            '.' => Rocks::Empty,
            _ => panic!("Invalid char"),
        }
    }
}

impl fmt::Display for Rocks {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Rocks::Rolling => write!(f, "O"),
            Rocks::Solid => write!(f, "#"),
            Rocks::Empty => write!(f, "."),
        }
    }
}

fn north_faceing_load(matrix: &Vec<Vec<Rocks>>) -> i64 {
    let mut result = 0;
    for column in 0..matrix[0].len() {
        for row in 0..matrix.len() {
            match matrix[row][column] {
                Rocks::Rolling => {
                    result += matrix.len() - row;
                }
                _ => {}
            }
        }
    }
    result as i64
}

fn rolling_cycle(matrix: &Vec<Vec<Rocks>>) -> Vec<Vec<Rocks>> {
    let mut result = matrix.clone();

    // Push all rolling rocks up/north
    for column in 0..matrix[0].len() {
        let mut current_index = 0;
        for row in 0..matrix.len() {
            match matrix[row][column] {
                Rocks::Solid => {
                    current_index = row + 1;
                }
                Rocks::Rolling => {
                    result[row][column] = Rocks::Empty;
                    result[current_index][column] = Rocks::Rolling;
                    current_index += 1;
                }
                Rocks::Empty => {}
            }
        }
    }

    // Rotate the matrix 90 degrees clockwise:
    let mut rotated = vec![vec![Rocks::Empty; matrix.len()]; matrix[0].len()];
    for row in 0..matrix.len() {
        for column in 0..matrix[0].len() {
            rotated[column][matrix.len() - row - 1] = result[row][column];
        }
    }
    rotated
}
fn solve(file: &str) -> i64 {
    let mut matrix: Vec<Vec<Rocks>> = file
        .lines()
        .map(|line| line.chars().map(|c| Rocks::from(c)).collect::<Vec<_>>())
        .collect();

    const ITERATIONS: usize = 1000000000;

    let mut seen: HashMap<Vec<Vec<Rocks>>, usize> = HashMap::new();

    for i in 0..ITERATIONS {
        // println!(
        //     "{} \n\n",
        //     matrix
        //         .iter()
        //         .map(|row| row
        //             .iter()
        //             .map(|c| c.to_string())
        //             .collect::<Vec<_>>()
        //             .join(""))
        //         .collect::<Vec<_>>()
        //         .join("\n")
        // );

        if seen.contains_key(&matrix) {
            let cycle_length = i - seen.get(&matrix).unwrap();
            let remaining_iterations = ITERATIONS - i;
            let remaining_iterations = remaining_iterations % cycle_length;
            println!("Cycle length: {}", cycle_length);
            for _ in 0..remaining_iterations {
                for _ in 0..4 {
                    matrix = rolling_cycle(&matrix);
                }
                // println!(
                //     "{} \n\n",
                //     matrix
                //         .iter()
                //         .map(|row| row
                //             .iter()
                //             .map(|c| c.to_string())
                //             .collect::<Vec<_>>()
                //             .join(""))
                //         .collect::<Vec<_>>()
                //         .join("\n")
                // );
                println!("{} \n", north_faceing_load(&matrix));
            }
            break;
        }
        seen.insert(matrix.clone(), i);

        for _ in 0..4 {
            matrix = rolling_cycle(&matrix);
        }
        println!("i: {}\t\t Load: {}", i, north_faceing_load(&matrix));
    }
    println!(
        "{} \n\n",
        matrix
            .iter()
            .map(|row| row
                .iter()
                .map(|c| c.to_string())
                .collect::<Vec<_>>()
                .join(""))
            .collect::<Vec<_>>()
            .join("\n")
    );
    north_faceing_load(&matrix)
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
        let input = "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....";
        assert_eq!(solve(input), 64);
    }
}
