/// Solution to Advent of Code Challenge Day 05.
use aoc2021::{get_day_input, parse_input_with, print_elapsed_time};
use std::collections::HashMap;

fn calculate_population(starting: &Vec<u32>, cycles: u32) -> u64 {
    let mut counts: HashMap<u32, u64> = HashMap::with_capacity(8);

    for fish in starting {
        *counts.entry(*fish).or_insert(0) += 1;
    }

    for _ in 0..cycles {
        let start_counts = counts.to_owned();

        // Decrease the days left for all fish (that aren't about to reproduce)
        for day in 1..=8 {
            *counts.entry(day - 1).or_insert(0) = *start_counts.get(&day).unwrap_or(&0);
        }

        // Create new fish at day 8
        *counts.entry(8).or_insert(0) = *start_counts.get(&0).unwrap_or(&0);

        // Add fish that reproduced to day 6
        *counts.entry(6).or_insert(0) += start_counts.get(&0).unwrap_or(&0);
    }

    counts.iter().fold(0, |acc, (_, num)| acc + num)
}

fn part_one(input: &Vec<u32>) -> u64 {
    calculate_population(input, 80)
}

fn part_two(input: &Vec<u32>) -> u64 {
    calculate_population(input, 256)
}

fn main() {
    let input = get_day_input("06");
    let inputs = parse_input_with(&input, |s| s.split(','));
    println!("Day 06:");
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "3,4,3,1,2".to_string();

        let inputs = parse_input_with(&input, |s| s.split(','));
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 5934);
        assert_eq!(part_two(&inputs), 26984457539);
    }
}
