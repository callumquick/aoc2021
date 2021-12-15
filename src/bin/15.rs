/// Solution to Advent of Code Challenge Day 15.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAY: &str = "15";

type Risk = u8;

struct Row(Vec<Risk>);

impl FromStr for Row {
    type Err = io::Error;
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

    // if j > 0 {
    //     neighbours.push((i, j - 1));
    // }
    if j < y_len - 1 {
        neighbours.push((i, j + 1));
    }
    // if i > 0 {
    //     neighbours.push((i - 1, j));
    // }
    if i < x_len - 1 {
        neighbours.push((i + 1, j));
    }

    neighbours
}

type Coord = (usize, usize);
type Path = Vec<Coord>;
type Cost = u64;

fn find_paths(
    grid: &[Row],
    path_costs: &mut HashMap<Path, Cost>,
    min_cost: &mut Cost,
    max_len: usize,
    square_side: usize,
    visited: HashSet<Coord>,
    path: Path,
    cost: u64,
) {
    let end = (square_side - 1, square_side - 1);

    if path.len() + 1 >= max_len {
        // Just give up searching along this path: probably isn't the answer.
        return;
    }

    if cost >= *min_cost {
        // Path already worst than our best guess, quit.
        return;
    }

    let len_left = max_len - path.len();

    let curr = path.last().unwrap();
    // println!(
    //     "Starting DFS from {:?} search for {:?}, current path len: {:?}",
    //     curr,
    //     end,
    //     path.len()
    // );
    for neighbour in get_neighbours(grid, curr.0, curr.1) {
        // Don't consider any neighbour which will not take us in range of the
        // target with what length we have left.
        if (end.0 - neighbour.0) + (end.1 - neighbour.1) > len_left {
            continue;
        }

        let mut new_path: Path = path.clone();
        new_path.push(neighbour);

        let new_cost = cost + grid[neighbour.1].0[neighbour.0] as u64;

        if neighbour == end {
            if new_cost < *min_cost {
                *min_cost = new_cost;
                println!("Found new best path, cost: {}", new_cost);
                path_costs.insert(new_path.clone(), new_cost);
            }
        } else if visited.contains(&neighbour) {
            continue;
        } else {
            let mut new_visited = visited.clone();
            new_visited.insert(neighbour);

            find_paths(
                grid,
                path_costs,
                min_cost,
                max_len,
                square_side,
                new_visited,
                new_path,
                new_cost,
            );
        }
    }
}

fn part_one(input: &[Row]) -> u64 {
    // Assume the least risk path will also be as short as possible for a
    // non-diagonal path.
    let square_side = input.len();
    let max_len = 2 * square_side;

    let mut path_costs = HashMap::new();
    let mut min_cost = (max_len * 9) as u64;
    let starting_path = vec![(0, 0)];
    let starting_cost = 0;
    let starting_visisted = HashSet::from([(0, 0)]);

    find_paths(
        input,
        &mut path_costs,
        &mut min_cost,
        max_len,
        square_side,
        starting_visisted,
        starting_path,
        starting_cost,
    );

    min_cost
}

fn part_two(input: &[Row]) -> u64 {
    0
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
        let input: String = "1163751742
1381373672
2136511328
3694931569
7463417111
1319128137
1359912421
3125421639
1293138521
2311944581"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);

        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 40);
        assert_eq!(part_two(&inputs), 0);
    }
}
