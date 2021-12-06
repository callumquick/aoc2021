/// Solution to Advent of Code Challenge Day 05.
use aoc2021::{get_day_input, parse_input_with, print_elapsed_time};

fn calculate_population(starting: &[u32], cycles: u32) -> u64 {
    let mut counts = [0u64; 9];

    for fish in starting {
        let idx: usize = (*fish).try_into().unwrap();
        counts[idx] += 1;
    }

    for _ in 0..cycles {
        let start_counts = counts;

        // Decrease the days left for all fish (that aren't about to reproduce)
        counts[0..=7].clone_from_slice(&start_counts[1..=8]);

        // Create new fish at day 8
        counts[8] = start_counts[0];

        // Add fish that reproduced to day 6
        counts[6] += start_counts[0];
    }

    counts.iter().sum()
}

fn part_one(input: &[u32]) -> u64 {
    calculate_population(input, 80)
}

fn part_two(input: &[u32]) -> u64 {
    calculate_population(input, 256)
}

fn main() {
    let input = get_day_input("06");
    let inputs: Vec<_> = parse_input_with(&input, |s| s.split(','));
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

        let inputs: Vec<_> = parse_input_with(&input, |s| s.split(','));
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 5934);
        assert_eq!(part_two(&inputs), 26984457539);
    }
}
