/// Solution to Advent of Code Challenge Day 11.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const DAY: &str = "11";

type Power = u32;

#[derive(Debug, Clone)]
struct Row(Vec<Power>);

impl FromStr for Row {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect(),
        ))
    }
}

fn get_neighbours(input: &[Row], i: usize, j: usize) -> Vec<(usize, usize)> {
    let y_len = input.len();
    let x_len = input[0].0.len();

    let mut neighbours = Vec::new();

    if j > 0 {
        neighbours.push((i, j - 1));
        if i > 0 {
            neighbours.push((i - 1, j - 1));
        }
    }
    if j < y_len - 1 {
        neighbours.push((i, j + 1));
        if i < x_len - 1 {
            neighbours.push((i + 1, j + 1));
        }
    }
    if i > 0 {
        neighbours.push((i - 1, j));
        if j < y_len - 1 {
            neighbours.push((i - 1, j + 1));
        }
    }
    if i < x_len - 1 {
        neighbours.push((i + 1, j));
        if j > 0 {
            neighbours.push((i + 1, j - 1));
        }
    }

    neighbours
}

// Iterate the octopi one step and return how many flashed.
fn octopus_iteration(octopi: &mut Vec<Row>) -> u64 {
    octopi
        .iter_mut()
        .for_each(|row| row.0.iter_mut().for_each(|o| *o += 1));

    let mut flashing = HashSet::new();
    let mut processing = Vec::new();

    // Setup the processing with every octopus now flashing
    for (j, row) in octopi.iter().enumerate() {
        for (i, octopus) in row.0.iter().enumerate() {
            if *octopus > 9 {
                processing.push((i, j));
                flashing.insert((i, j));
            }
        }
    }

    // Then for all octopi that flashed, increase the power level of the
    // neighbour octopi, which may trigger more flashing. Process until the
    // processing queue is empty.
    while !processing.is_empty() {
        let octopus = processing.pop().unwrap();
        for (i, j) in get_neighbours(octopi, octopus.0, octopus.1) {
            octopi[j].0[i] += 1;
            // If this has enough energy to flash but that flash hasn't
            // already been processed, add it to the processing queue.
            if octopi[j].0[i] > 9 && !flashing.contains(&(i, j)) {
                processing.push((i, j));
                flashing.insert((i, j));
            }
        }
    }

    // Set any octopus that flashed to energy 0.
    for (i, j) in &flashing {
        octopi[*j].0[*i] = 0;
    }

    flashing.len() as u64
}

fn part_one(input: &[Row], steps: usize) -> u64 {
    let mut octopi = input.to_vec();
    let mut flashes = 0;

    for _ in 0..steps {
        flashes += octopus_iteration(&mut octopi);
    }

    flashes
}

fn part_two(input: &[Row]) -> u64 {
    let mut octopi = input.to_vec();
    let mut flashes = 0;
    let mut steps = 0;

    while flashes != (octopi.len() * octopi[0].0.len()) as u64 {
        steps += 1;
        flashes = octopus_iteration(&mut octopi);
    }

    steps
}

fn main() {
    let input = get_day_input(DAY);
    let inputs: Vec<_> = parse_input_lines(&input);
    println!("Day {}:", DAY);
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&inputs, 100))
    );
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs, 100), 1656);
        assert_eq!(part_two(&inputs), 195);
    }

    #[test]
    fn test_other_example() {
        let input: String = "11111
19991
19191
19991
11111"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs, 2), 9);
    }
}
