/// Solution to Advent of Code Challenge Day 07.
use aoc2021::{get_day_input, parse_input_with, print_elapsed_time};

fn cost_p1(target: u32, start: u32) -> u32 {
    let a: i32 = start.try_into().unwrap();
    let b: i32 = target.try_into().unwrap();

    // Direct linear cost
    ((b - a).abs()).try_into().unwrap()
}

fn cost_p2(target: u32, start: u32) -> u32 {
    let a: i32 = start.try_into().unwrap();
    let b: i32 = target.try_into().unwrap();

    // Cost is the difference N as a triangular number with formula N(N+1)/2
    let diff: u32 = ((b - a).abs()).try_into().unwrap();
    (diff * (diff + 1)) / 2
}

/// Finds the fuel cost using triangular cost or linear cost
fn find_cost(input: &[u32], target: u32, triangle_cost: bool) -> u32 {
    let mut sum = 0;
    for curr in input {
        if triangle_cost {
            sum += cost_p2(target, *curr);
        } else {
            sum += cost_p1(target, *curr);
        }
    }
    sum
}

fn part_one(input: &[u32]) -> u32 {
    // The sum of differences is minimised by the median: when as many positions
    // are below it as above it
    let mut sorted = input.to_vec();
    sorted.sort_unstable();

    // Assume we're given odd-length inputs
    let median = input[input.len() / 2];

    find_cost(input, median, false)
}

fn part_two(input: &[u32]) -> u32 {
    // The sum of square differences is minimised by one of the whole numbers on
    // either side of the mean: so get the lowest of the two costs
    let floored_mean = input.iter().sum::<u32>() / input.len() as u32;

    (floored_mean..=(floored_mean + 1))
        .map(|target| find_cost(input, target, true))
        .min()
        .unwrap()
}

fn main() {
    let input = get_day_input("07");
    let inputs: Vec<_> = parse_input_with(&input, |s| s.split(','));
    println!("Day 07:");
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "16,1,2,0,4,2,7,1,2,14".to_string();

        let inputs: Vec<_> = parse_input_with(&input, |s| s.split(','));
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 37);
        assert_eq!(part_two(&inputs), 168);
    }
}
