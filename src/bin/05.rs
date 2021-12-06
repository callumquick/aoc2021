/// Solution to Advent of Code Challenge Day 05.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Point(u32, u32);

impl FromStr for Point {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<_> = parse_input_with(s, |s| s.split(','));
        Ok(Self(parsed[0], parsed[1]))
    }
}

#[derive(Debug, Clone, Copy)]
struct Line {
    start: Point,
    end: Point,
}

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<_> = parse_input_with(s, |s| s.split(" -> "));
        Ok(Self {
            start: parsed[0],
            end: parsed[1],
        })
    }
}

impl Line {
    fn vertical(&self) -> bool {
        self.start.0 == self.end.0
    }

    fn horizontal(&self) -> bool {
        self.start.1 == self.end.1
    }

    fn diagonal(&self) -> bool {
        !(self.horizontal() || self.vertical())
    }

    fn points(&self) -> Vec<Point> {
        let mut points = Vec::new();

        let x_range: Vec<_> = if self.end.0 > self.start.0 {
            (self.start.0..=self.end.0).collect()
        } else {
            (self.end.0..=self.start.0).rev().collect()
        };

        let y_range: Vec<_> = if self.end.1 > self.start.1 {
            (self.start.1..=self.end.1).collect()
        } else {
            (self.end.1..=self.start.1).rev().collect()
        };

        if self.vertical() {
            // For the vertical case, expect the x value to remain the same.
            for j in y_range {
                points.push(Point(self.start.0, j));
            }
        } else if self.horizontal() {
            // For the horizontal case, expect the y value to remain the same.
            for i in x_range {
                points.push(Point(i, self.start.1));
            }
        } else {
            // For the diagonal case, expect the two ranges to be the same length.
            for (i, j) in x_range.into_iter().zip(y_range) {
                points.push(Point(i, j));
            }
        }

        points
    }
}

fn calculate_overlaps(input: Vec<Line>) -> u64 {
    let mut x_max = 0;
    let mut y_max = 0;

    for line in &input {
        if line.start.0 > x_max {
            x_max = line.start.0;
        }
        if line.end.0 > x_max {
            x_max = line.end.0;
        }
        if line.start.1 > y_max {
            y_max = line.start.1;
        }
        if line.end.1 > y_max {
            y_max = line.end.1;
        }
    }

    let mut position_counts: Vec<Vec<u32>> = Vec::new();

    for _ in 0..=x_max {
        let mut ys = Vec::new();
        for _ in 0..=y_max {
            ys.push(0)
        }
        position_counts.push(ys)
    }

    for line in &input {
        for point in line.points() {
            let x: usize = point.0.try_into().unwrap();
            let y: usize = point.1.try_into().unwrap();
            position_counts[x][y] += 1;
        }
    }

    let mut overlaps = 0;
    for xs in position_counts {
        for y in xs {
            if y > 1 {
                overlaps += 1;
            }
        }
    }

    overlaps
}

fn part_one(input: &Vec<Line>) -> u64 {
    calculate_overlaps(
        input
            .iter()
            .filter(|l| !l.diagonal())
            .map(|l| l.to_owned())
            .collect(),
    )
}

fn part_two(input: &Vec<Line>) -> u64 {
    calculate_overlaps(input.to_vec())
}
fn main() {
    let input = get_day_input("05");
    let inputs = parse_input_lines(&input);
    println!("Day 05:");
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "0,9 -> 5,9
8,0 -> 0,8
9,4 -> 3,4
2,2 -> 2,1
7,0 -> 7,4
6,4 -> 2,0
0,9 -> 2,9
3,4 -> 1,4
0,0 -> 8,8
5,5 -> 8,2"
            .to_string();

        let inputs = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 5);
        assert_eq!(part_two(&inputs), 12);
    }
}
