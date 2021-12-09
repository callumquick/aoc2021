/// Solution to Advent of Code Challenge Day 09.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const DAY: &str = "09";

type Height = u32;

#[derive(Debug, Clone)]
struct Row(Vec<Height>);

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
    }
    if j < y_len - 1 {
        neighbours.push((i, j + 1));
    }
    if i > 0 {
        neighbours.push((i - 1, j));
    }
    if i < x_len - 1 {
        neighbours.push((i + 1, j));
    }

    neighbours
}

fn get_lowest(input: &[Row]) -> Vec<(usize, usize)> {
    let mut lowest = Vec::new();

    for (j, row) in input.iter().enumerate() {
        for (i, val) in row.0.iter().enumerate() {
            let neighbours = get_neighbours(input, i, j);
            if neighbours.iter().all(|(i, j)| val < &input[*j].0[*i]) {
                lowest.push((i, j));
            }
        }
    }

    lowest
}

fn part_one(input: &[Row]) -> u32 {
    get_lowest(input)
        .iter()
        .map(|(i, j)| &input[*j].0[*i] + 1)
        .sum()
}

fn part_two(input: &[Row]) -> u32 {
    let lowest = get_lowest(input);
    let mut basin_sizes = Vec::new();

    // Algorithm for finding basins:
    // - From each lowest point, form hashset of coords representing the basin
    // - Add all neighbours of lowest point to hashset which aren't a 9
    // - For each neighbour which is not a 9 and not already in the hashset,
    //   find its neighbours and repeat
    // - Each lowest point is associated with a single basin, so this is all the
    //   basins
    for point in lowest {
        let mut basin = HashSet::new();
        basin.insert(point);

        let mut remaining = vec![point];

        while !remaining.is_empty() {
            let curr = remaining.pop().unwrap();
            let new_basin_neighbours: Vec<_> = get_neighbours(input, curr.0, curr.1)
                .iter()
                .filter(|(i, j)| input[*j].0[*i] != 9 && !basin.contains(&(*i, *j)))
                .map(|p| p.to_owned())
                .collect();

            basin.extend(new_basin_neighbours.iter());
            remaining.extend(new_basin_neighbours.iter());
        }

        basin_sizes.push(basin.len() as u32);
    }

    basin_sizes.sort_unstable();
    basin_sizes.iter().rev().take(3).product()
}

fn main() {
    let input = get_day_input(DAY);
    let inputs: Vec<_> = parse_input_lines(&input);
    println!("Day {}:", DAY);
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "2199943210
3987894921
9856789892
8767896789
9899965678"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 15);
        assert_eq!(part_two(&inputs), 1134);
    }
}
