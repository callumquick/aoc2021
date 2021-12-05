/// Solution to Advent of Code Challenge Day 04.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Copy, Clone, PartialEq)]
struct BingoNum {
    num: u32,
    marked: bool,
}

impl FromStr for BingoNum {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            num: s.parse()?,
            marked: false,
        })
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Line(Vec<BingoNum>);

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(parse_input_with(s, |s| {
            s.split(' ').filter(|s| !s.is_empty())
        })))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct BingoGrid {
    rows: Vec<Line>,
}

impl FromStr for BingoGrid {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            rows: parse_input_lines(s),
        })
    }
}

impl BingoGrid {
    fn mark_num(&mut self, num: u32) {
        for row in &mut self.rows {
            for entry in &mut row.0 {
                if entry.num == num {
                    entry.marked = true;
                }
            }
        }
    }

    fn is_bingo(&self) -> bool {
        for row in &self.rows {
            if row.0.iter().all(|n| n.marked) {
                return true;
            }
        }
        for idx in 0..self.rows[0].0.len() {
            if self.rows.iter().all(|row| row.0[idx].marked) {
                return true;
            }
        }
        false
    }

    fn unmarked_sum(&self) -> u32 {
        let mut sum = 0;
        for row in &self.rows {
            for BingoNum { num, marked } in &row.0 {
                if !marked {
                    sum += num;
                }
            }
        }
        sum
    }
}

fn part_one(input: &Vec<BingoGrid>, draw: &Vec<u32>) -> u32 {
    let mut grids = input.to_vec();
    for num in draw {
        for grid in grids.iter_mut() {
            grid.mark_num(*num);
            if grid.is_bingo() {
                return grid.unmarked_sum() * num;
            }
        }
    }
    0
}

fn part_two(input: &Vec<BingoGrid>, draw: &Vec<u32>) -> u32 {
    let mut grids = input.to_vec();
    for num in draw {
        let mut incomplete_grids = Vec::new();
        for grid in grids.iter_mut() {
            grid.mark_num(*num);
            if !grid.is_bingo() {
                incomplete_grids.push(grid.to_owned());
            }
        }
        if grids.len() == 1 && incomplete_grids.len() == 0 {
            // Final grid has been completed
            return grids[0].unmarked_sum() * num;
        }
        grids = incomplete_grids;
    }
    0
}

fn main() {
    let input = get_day_input("04");
    let draw_input: String = input.lines().take(1).collect::<Vec<_>>().join("\n");
    let bingo_input: String = input.lines().skip(2).collect::<Vec<_>>().join("\n");
    let inputs = parse_input_with(&bingo_input, |s| s.split("\n\n"));
    let draw = parse_input_with(&draw_input, |s| s.split(','));
    println!("Day 04:");
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&inputs, &draw))
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&inputs, &draw))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7"
            .to_string();

        let draw_input: String = input.lines().take(1).collect::<Vec<_>>().join("\n");
        let bingo_input: String = input.lines().skip(2).collect::<Vec<_>>().join("\n");
        let inputs = parse_input_with(&bingo_input, |s| s.split("\n\n"));
        let draw = parse_input_with(&draw_input, |s| s.split(','));
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs, &draw), 4512);
        assert_eq!(part_two(&inputs, &draw), 1924);
    }
}
