use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day01").unwrap();

    let part1: usize = input.lines().map(calibrate).sum();
    println!("part 1: {}", part1);

    let part2: usize = input.lines().map(calibrate_written_nums).sum();
    println!("part 2: {}", part2);
}

fn calibrate(line: &str) -> usize {
    let first = line.chars().find(|char| char.is_numeric()).unwrap();
    let last = line.chars().rev().find(|char| char.is_numeric()).unwrap();
    format!("{}{}", first, last).parse::<usize>().unwrap()
}

fn calibrate_written_nums(line: &str) -> usize {
    let replace_values = [
        ("one", "o1e"),
        ("two", "t2o"),
        ("three", "t3e"),
        ("four", "f4r"),
        ("five", "f5e"),
        ("six", "s6x"),
        ("seven", "s7n"),
        ("eight", "e8t"),
        ("nine", "n9e"),
    ];

    let mut replaced = String::from(line);
    for replc in replace_values {
        replaced = replaced.replace(replc.0, replc.1);
    }

    calibrate(&replaced)
}
