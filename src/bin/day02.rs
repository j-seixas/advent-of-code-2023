use std::cmp;
use std::fs::read_to_string;

struct Game {
    id: usize,
    blues: usize,
    reds: usize,
    greens: usize,
}

fn main() {
    let input = read_to_string("input/day02.txt").unwrap();

    let part1: usize = input
        .lines()
        .map(|line| parse_games(line))
        .filter(|game| possible_game(game, 14, 12, 13))
        .map(|game| game.id)
        .sum();
    println!("part 1: {}", part1);

    let part2: usize = input
        .lines()
        .map(|line| power_game(parse_games(line)))
        .sum();
    println!("part 2: {}", part2);
}

fn parse_games(line: &str) -> Game {
    let (game, sets) = line.split_once(": ").unwrap();
    let id: usize = game.replace("Game ", "").parse().unwrap();

    let sets_list = sets.split("; ");

    let mut max_cubes = vec![0, 0, 0];
    for set in sets_list {
        let set_cubes = set.split(", ");
        for cubes in set_cubes {
            let (count, cube_color) = cubes.split_once(" ").unwrap();
            let count_num: usize = count.parse().unwrap();
            match cube_color {
                "blue" => max_cubes[0] = cmp::max(count_num, max_cubes[0]),
                "red" => max_cubes[1] = cmp::max(count_num, max_cubes[1]),
                "green" => max_cubes[2] = cmp::max(count_num, max_cubes[2]),
                _ => println!("Found error in cube color"),
            }
        }
    }

    Game {
        id: id,
        blues: max_cubes[0],
        reds: max_cubes[1],
        greens: max_cubes[2],
    }
}

fn possible_game(game: &Game, blues: usize, reds: usize, greens: usize) -> bool {
    game.blues <= blues && game.reds <= reds && game.greens <= greens
}

fn power_game(game: Game) -> usize {
    game.blues * game.reds * game.greens
}
