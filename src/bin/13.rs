/// Solution to Advent of Code Challenge Day 13.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::io;
use std::str::FromStr;

const DAY: &str = "13";
const PAPER_MAX: usize = 1350;

type Paper = [bool; PAPER_MAX * PAPER_MAX];

#[derive(Debug, Clone, PartialEq)]
struct Point(usize, usize);

impl FromStr for Point {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<usize> = parse_input_with(s, |s| s.split(','));
        Ok(Self(parsed[0], parsed[1]))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Fold(usize, usize);

impl FromStr for Fold {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<String> = parse_input_with(s, |s| s.split('='));
        let num = parsed[1].parse().unwrap();
        if parsed[0].contains('x') {
            Ok(Self(num, 0))
        } else {
            Ok(Self(0, num))
        }
    }
}

#[derive(Debug, Clone)]
struct Manual {
    dots: Vec<Point>,
    folds: Vec<Fold>,
}

fn get_coord(paper: &Paper, i: usize, j: usize) -> bool {
    paper[j * PAPER_MAX + i]
}

fn set_coord(paper: &mut Paper, i: usize, j: usize) {
    paper[j * PAPER_MAX + i] = true;
}

fn fold_paper(paper: &mut Paper, fold: &Fold) {
    if fold.0 > 0 {
        // It's a fold along a vertical line where x=fold.0
        for i in (fold.0 + 1)..=(fold.0 + fold.0) {
            for j in 0..PAPER_MAX {
                let fold_i = 2 * fold.0 - i;
                if get_coord(paper, i, j) {
                    set_coord(paper, fold_i, j);
                }
            }
        }
    } else {
        // It's a fold along a horizontal line where y=fold.1
        for j in (fold.1 + 1)..=(fold.1 + fold.1) {
            for i in 0..PAPER_MAX {
                let fold_j = (2 * fold.1) - j;
                if get_coord(paper, i, j) {
                    set_coord(paper, i, fold_j);
                }
            }
        }
    }
}

fn part_one(input: &Manual) -> u64 {
    let mut paper = [false; PAPER_MAX * PAPER_MAX];

    for dot in &input.dots {
        set_coord(&mut paper, dot.0, dot.1);
    }

    // When using the resulting paper: only need to consider up to the last two
    // constraining folds. These will be the last x fold and y fold, respectively.
    // Part one only needs the first fold, so there is only one constraint.
    let mut x_max = PAPER_MAX;
    let mut y_max = PAPER_MAX;
    fold_paper(&mut paper, &input.folds[0]);
    if input.folds[0].0 > 0 {
        x_max = input.folds[0].0;
    } else {
        y_max = input.folds[0].1;
    }

    let mut sum = 0;
    for j in 0..y_max {
        for i in 0..x_max {
            sum += get_coord(&paper, i, j) as u64;
        }
    }
    sum
}

fn part_two(input: &Manual) -> String {
    let mut paper = [false; PAPER_MAX * PAPER_MAX];

    for dot in &input.dots {
        set_coord(&mut paper, dot.0, dot.1);
    }

    // When using the resulting paper: only need to consider up to the last two
    // constraining folds. These will be the last x fold and y fold, respectively.
    let mut x_max = PAPER_MAX;
    let mut y_max = PAPER_MAX;
    for fold in &input.folds {
        fold_paper(&mut paper, fold);
        if fold.0 > 0 {
            x_max = fold.0;
        } else {
            y_max = fold.1;
        }
    }

    // Print it line by line, which is one line for each j.
    (0..y_max)
        .map(|j| {
            paper[(j * PAPER_MAX)..((j * PAPER_MAX) + x_max)]
                .iter()
                .map(|b| if *b { "#" } else { "." })
                .collect::<Vec<_>>()
                .join("")
        })
        .collect::<Vec<_>>()
        .join("\n")
}

fn get_manual(input: &str) -> Manual {
    let inputs: Vec<String> = parse_input_with(input, |s| s.split("\n\n"));
    Manual {
        dots: parse_input_lines(&inputs[0]),
        folds: parse_input_lines(&inputs[1]),
    }
}

fn main() {
    let input = get_day_input(DAY);
    let manual = get_manual(&input);
    println!("Day {}:", DAY);
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&manual)));
    println!("Part two:\n{}", print_elapsed_time(|| part_two(&manual)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5"
            .to_string();

        let expected_code: String = "#####
#...#
#...#
#...#
#####
.....
....."
            .to_string();

        let manual = get_manual(&input);

        // Check each gives the right answer.
        assert_eq!(part_one(&manual), 17);
        assert_eq!(part_two(&manual), expected_code);
    }
}
