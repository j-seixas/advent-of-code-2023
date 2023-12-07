use regex::Regex;
use std::collections::HashSet;
use std::fs::read_to_string;

fn main() {
    let input = read_to_string("input/day03.txt").unwrap();
    let input_vec: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

    let mut engine_parts = 0;
    let mut gear_ratio = 0;
    let re = Regex::new(r"([^.\w\s])").unwrap();
    for (i, line) in input.lines().enumerate() {
        let positions = re
            .captures_iter(line)
            .map(|caps| (caps.get(0).unwrap().as_str(), caps.get(0).unwrap().start()));

        positions.for_each(|(char, j)| {
            let adjacent = adjacent_nums(&input_vec, i, j);
            engine_parts += adjacent.iter().sum::<usize>();

            if char == "*" && adjacent.len() == 2 {
                gear_ratio += adjacent.iter().product::<usize>();
            }
        });
    }

    println!("part 1: {}", engine_parts);
    println!("part 2: {}", gear_ratio);
}

fn adjacent_nums(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> HashSet<usize> {
    let possible = [
        (i - 1, j - 1),
        (i - 1, j),
        (i - 1, j + 1),
        (i, j - 1),
        (i, j + 1),
        (i + 1, j - 1),
        (i + 1, j),
        (i + 1, j + 1),
    ];

    let mut engine_parts = HashSet::new();
    for (x, y) in possible {
        if x >= matrix.len() || y >= matrix[0].len() {
            continue;
        }

        if matrix[x][y].is_numeric() {
            engine_parts.insert(get_part(matrix, x, y));
        }
    }

    engine_parts
}

fn get_part(matrix: &Vec<Vec<char>>, i: usize, j: usize) -> usize {
    let (mut start, mut end) = (j, j);
    while start > 0 && matrix[i][start - 1].is_numeric() {
        start -= 1;
    }

    while end < matrix[i].len() - 1 && matrix[i][end + 1].is_numeric() {
        end += 1;
    }

    let part_number: String = matrix[i][start..end + 1].iter().collect();
    part_number.parse().unwrap()
}
