use std::time::Instant;

fn solve(file: &str) -> i64 {
    let matrix: Vec<Vec<char>> = file
        .lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect();

    let mut result = 0;
    for column in 0..matrix[0].len() {
        let mut current_index = matrix.len();
        for row in 0..matrix.len() {
            match matrix[row][column] {
                '#' => {
                    current_index = matrix.len() - row - 1;
                }
                'O' => {
                    result += current_index;
                    current_index -= 1;
                }
                _ => {}
            }
        }
    }

    result as i64
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
        assert_eq!(solve(input), 136)
    }
}
