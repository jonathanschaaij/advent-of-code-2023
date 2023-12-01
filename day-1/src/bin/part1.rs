fn main() {
    let sum: usize = include_str!("input.txt")
        .lines()
        .map(|l| l.chars().filter(|c| c.is_digit(10)).collect::<Vec<_>>())
        .map(|v| {
            format!("{}{}", v[0], v[v.len() - 1])
                .parse::<usize>()
                .unwrap()
        })
        .sum();
    println!("Sum: {}", sum);
}
