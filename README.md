# Advent of Code 2023

This year I want to do advent of code using a new programming language called Rust. 

## How to use template

To use the template you need to to have a file called .aoc_session which has the Cookie Token, which allows downloading
the days input:
```
export AOC_SESSION=<Your Cookie Token here>
```

Then you can simply use the following command to copy the template, download the days input and open nvim to start
editing:
```bash
./newday <DAY>
```

## How to run the solutions

Navigate to the day-XX folder and use one of the following commands

```bash
cargo run --bin part1
cargo test --bin part1 
cargo run --bin part2
cargo test --bin part 
```

