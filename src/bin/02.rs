/// Solution to Advent of Code Challenge Day 02.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::io;
use std::str::FromStr;

#[derive(PartialEq)]
enum Instruction {
    Forward(u32),
    Down(u32),
    Up(u32),
}

impl FromStr for Instruction {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vec: Vec<_> = s.split_whitespace().collect();
        let (instr, val) = match &vec[..] {
            &[first, second, ..] => (
                first,
                second
                    .parse::<u32>()
                    .expect("Instruction value is not a valid unsigned integer"),
            ),
            _ => unreachable!(),
        };
        Ok(match instr {
            "forward" => Self::Forward(val),
            "down" => Self::Down(val),
            "up" => Self::Up(val),
            _ => panic!("Instruction name is not supported"),
        })
    }
}

/// Find the horizontal and depth positions multiplied together
/// Uses Part One instruction meanings
fn part_one(input: &Vec<Instruction>) -> u32 {
    let mut h = 0;
    let mut d = 0;

    for instr in input {
        match instr {
            Instruction::Forward(val) => h += val,
            Instruction::Down(val) => d += val,
            Instruction::Up(val) => d -= val,
        }
    }

    h * d
}

/// Find the horizontal and depth positions multiplied together
/// Uses Part Two instruction meanings
fn part_two(input: &Vec<Instruction>) -> u32 {
    let mut a = 0;
    let mut h = 0;
    let mut d = 0;

    for instr in input {
        match instr {
            Instruction::Forward(val) => {
                h += val;
                d += a * val;
            }
            Instruction::Down(val) => a += val,
            Instruction::Up(val) => a -= val,
        }
    }

    h * d
}

fn main() {
    let input = get_day_input("02");
    let instructions = parse_input_lines(input);
    println!("Day 02:");
    println!("==========");
    println!(
        "Part one: {}",
        print_elapsed_time(|| part_one(&instructions))
    );
    println!(
        "Part two: {}",
        print_elapsed_time(|| part_two(&instructions))
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "forward 5
down 5
forward 8
up 3
down 8
forward 2"
            .to_string();

        let instructions = parse_input_lines(input);
        // Check each gives the right answer.
        assert_eq!(part_one(&instructions), 150);
        assert_eq!(part_two(&instructions), 900);
    }
}
