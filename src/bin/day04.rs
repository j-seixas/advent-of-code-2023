use std::{collections::HashMap, fs::read_to_string};

fn main() {
    let input = read_to_string("input/day04.txt").unwrap();

    let scratchcards_matches = input.lines().map(|line| {
        let (_, scratch) = line.split_once(": ").unwrap();
        let (scratch_winning, scratch_numbers) = scratch.split_once(" | ").unwrap();

        let winning_numbers: Vec<&str> = scratch_winning.split_whitespace().collect();
        let my_numbers: Vec<&str> = scratch_numbers.split_whitespace().collect();

        let matching: u32 = find_matching(winning_numbers, my_numbers);
        matching
    });

    let part1: usize = scratchcards_matches
        .clone()
        .map(|matches| {
            if matches == 0 {
                return 0;
            }

            let base: usize = 2;
            base.pow(matches - 1)
        })
        .sum();
    println!("part 1: {:?}", part1);

    let mut total_scratches: HashMap<usize, usize> = HashMap::new();
    let mut part2: usize = 0;
    for (i, matches) in scratchcards_matches.enumerate() {
        let mut curr_matches: usize = 1;
        if total_scratches.contains_key(&i) {
            curr_matches = total_scratches.get(&i).unwrap() + 1;
        }
        for j in 0..matches.try_into().unwrap() {
            let matches_in_hash = total_scratches.entry(i + j + 1).or_insert(0);
            *matches_in_hash += curr_matches;
        }
        part2 += curr_matches;
    }

    println!("part 2: {}", part2);
}

fn find_matching(winning_nums: Vec<&str>, game_nums: Vec<&str>) -> u32 {
    let mut matching: u32 = 0;
    winning_nums.iter().for_each(|num| {
        if game_nums.contains(num) {
            matching += 1;
        }
    });

    matching
}
