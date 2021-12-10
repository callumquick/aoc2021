/// Solution to Advent of Code Challenge Day 10.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::num::ParseIntError;
use std::str::FromStr;

const DAY: &str = "10";

#[derive(Debug, Clone, Copy, PartialEq)]
enum Bracket {
    OpenNormal,
    CloseNormal,
    OpenSquare,
    CloseSquare,
    OpenCurly,
    CloseCurly,
    OpenAngle,
    CloseAngle,
}

impl FromStr for Bracket {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "(" => Self::OpenNormal,
            ")" => Self::CloseNormal,
            "[" => Self::OpenSquare,
            "]" => Self::CloseSquare,
            "{" => Self::OpenCurly,
            "}" => Self::CloseCurly,
            "<" => Self::OpenAngle,
            ">" => Self::CloseAngle,
            _ => panic!("Invalid bracket detected"),
        })
    }
}

impl Bracket {
    fn get_closer(&self) -> Bracket {
        match &self {
            Bracket::OpenNormal => Bracket::CloseNormal,
            Bracket::OpenSquare => Bracket::CloseSquare,
            Bracket::OpenCurly => Bracket::CloseCurly,
            Bracket::OpenAngle => Bracket::CloseAngle,
            _ => unimplemented!("Not implemented for closing brackets"),
        }
    }
}

#[derive(Debug, Clone)]
struct Line(Vec<Bracket>);

impl FromStr for Line {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(
            s.chars()
                .map(|ch| ch.to_string().parse().unwrap())
                .collect(),
        ))
    }
}

impl Line {
    /// Find all the brackets still open in the incomplete line, unless it was
    /// corrupted (in which case report by what).
    ///
    /// If it was corrupted, the incomplete open brackets should not be used.
    ///
    /// A complete, correct line returns (Vec::new(), None).
    fn get_incomplete_openers(&self) -> (Vec<Bracket>, Option<Bracket>) {
        let mut opened = Vec::new();
        for bracket in &self.0 {
            match bracket {
                Bracket::OpenNormal
                | Bracket::OpenSquare
                | Bracket::OpenCurly
                | Bracket::OpenAngle => opened.push(*bracket),
                Bracket::CloseNormal
                | Bracket::CloseSquare
                | Bracket::CloseCurly
                | Bracket::CloseAngle => match opened.pop() {
                    Some(open) => {
                        if open.get_closer() != *bracket {
                            return (opened, Some(*bracket));
                        }
                    }
                    None => return (opened, Some(*bracket)),
                },
            }
        }
        (opened, None)
    }

    /// Get the sequence of brackets needed to close an incomplete line.
    /// Corrupted lines will return None, complete lines an empty vec.
    fn completing_brackets(&self) -> Option<Vec<Bracket>> {
        let mut completing = Vec::new();
        let (remaining_open, corrupted) = self.get_incomplete_openers();

        if corrupted.is_some() {
            return None;
        }

        completing.extend(remaining_open.iter().rev().map(|b| b.get_closer()));
        Some(completing)
    }
}

fn part_one(input: &[Line]) -> u64 {
    let mut sum = 0;
    for line in input {
        // Get any corrupted lines
        sum += match line.get_incomplete_openers().1 {
            Some(Bracket::CloseNormal) => 3,
            Some(Bracket::CloseSquare) => 57,
            Some(Bracket::CloseCurly) => 1197,
            Some(Bracket::CloseAngle) => 25137,
            Some(_) => panic!("Opening bracket reported as corrupting"),
            None => 0,
        }
    }
    sum
}

fn part_two(input: &[Line]) -> u64 {
    let mut scores = Vec::new();
    for line in input {
        if let Some(completing) = line.completing_brackets() {
            let mut score = 0;
            for bracket in completing {
                score *= 5;
                score += match bracket {
                    Bracket::CloseNormal => 1,
                    Bracket::CloseSquare => 2,
                    Bracket::CloseCurly => 3,
                    Bracket::CloseAngle => 4,
                    _ => panic!("Opening bracket can't be needed to complete line"),
                };
            }

            scores.push(score);
        }
    }

    scores.sort_unstable();
    // In a list of odd length, the middle index is integer division of len/2.
    assert!(scores.len() % 2 != 0);
    scores[scores.len() / 2]
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
        let input: String = "[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 26397);
        assert_eq!(part_two(&inputs), 288957);
    }
}
