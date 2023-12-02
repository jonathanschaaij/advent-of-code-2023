use std::str::FromStr;
use std::time::Instant;

#[derive(Debug, PartialEq, PartialOrd)]
enum Color {
    Blue(u32),
    Green(u32),
    Red(u32),
}

impl FromStr for Color {
    type Err = color_eyre::Report;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split = s
            .split(" ")
            .filter(|w| !w.is_empty())
            .collect::<Vec<&str>>();
        assert_eq!(split.len(), 2);
        let num: u32 = split[0].parse().unwrap();
        match split[1] {
            "blue" => Ok(Color::Blue(num)),
            "green" => Ok(Color::Green(num)),
            "red" => Ok(Color::Red(num)),
            _ => Err(color_eyre::eyre::eyre!("Invalid color")),
        }
    }
}

#[derive(Debug)]
struct Set(Vec<Color>);

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<Set>,
}

fn parse_game(input: &str) -> Game {
    let split_colon: Vec<&str> = input.split(":").collect();
    let mut game = Game {
        id: 0,
        sets: Vec::new(),
    };
    // Get the game ID
    game.id = split_colon[0].split(" ").nth(1).unwrap().parse().unwrap();
    // Get the sets
    split_colon[1].split(";").for_each(|set_str| {
        let mut set: Set = Set(Vec::new());
        for color in set_str.split(",") {
            set.0.push(color.parse().unwrap());
        }
        game.sets.push(set);
    });
    game
}

fn check_valid(game: &Game, max_color: &[Color; 3]) -> bool {
    for set in &game.sets {
        for max_c in max_color.iter() {
            for color_in_set in set.0.iter() {
                match (max_c, color_in_set) {
                    (Color::Blue(max), Color::Blue(c)) => {
                        if max < c {
                            return false;
                        }
                    }
                    (Color::Green(max), Color::Green(c)) => {
                        if max < c {
                            return false;
                        }
                    }
                    (Color::Red(max), Color::Red(c)) => {
                        if max < c {
                            return false;
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    return true;
}

fn solve(file: &str) -> u32 {
    let max_color = [Color::Blue(14), Color::Green(13), Color::Red(12)];

    file.lines()
        .map(|line| parse_game(line))
        // .inspect(|game| println!("Parsed Game: {:?}", game))
        .filter(|game| check_valid(&game, &max_color))
        .inspect(|game| println!("Filtered  Game: {:?}", game))
        .map(|game| game.id)
        .sum::<u32>()
}

fn main() {
    let input = include_str!("input.txt");
    println!("Starting solution for part 1");
    let t0 = Instant::now();
    let result = solve(input);
    let t1 = Instant::now();

    println!("Result: {}", result);
    println!("Time: {:?}", (t1 - t0).as_micros());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_whole_part() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(solve(input), 8);
    }
}
