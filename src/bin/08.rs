/// Solution to Advent of Code Challenge Day 08.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::num::ParseIntError;
use std::str::FromStr;

const DAY: &str = "08";

const A: u32 = 0x01;
const B: u32 = 0x02;
const C: u32 = 0x04;
const D: u32 = 0x08;
const E: u32 = 0x10;
const F: u32 = 0x20;
const G: u32 = 0x40;

const ZERO: u32 = A | B | C | E | F | G;
const ONE: u32 = C | F;
const TWO: u32 = A | C | D | E | G;
const THREE: u32 = A | C | D | F | G;
const FOUR: u32 = B | C | D | F;
const FIVE: u32 = A | B | D | F | G;
const SIX: u32 = A | B | D | E | F | G;
const SEVEN: u32 = A | C | F;
const EIGHT: u32 = A | B | C | D | E | F | G;
const NINE: u32 = A | B | C | D | F | G;

#[derive(Debug, Clone, PartialEq, Default)]
struct Digit(HashSet<char>);

impl FromStr for Digit {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().collect()))
    }
}

#[derive(Debug, Clone, PartialEq, Default)]
struct Display {
    notes: Vec<Digit>,
    digits: Vec<Digit>,
}

impl FromStr for Display {
    type Err = ParseIntError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<String> = parse_input_with(s, |s| s.split(" | "));

        Ok(Self {
            notes: parse_input_with(&parsed[0], |s| s.split(' ')),
            digits: parse_input_with(&parsed[1], |s| s.split(' ')),
        })
    }
}

impl Display {
    fn get_len_notes(&self, len: usize) -> Vec<&Digit> {
        let mut notes = Vec::new();
        for note in &self.notes {
            if note.0.len() == len {
                notes.push(note);
            }
        }
        notes
    }

    fn calculate_number(&self) -> u32 {
        let one = self.get_len_notes(2)[0];
        let seven = self.get_len_notes(3)[0];
        let four = self.get_len_notes(4)[0];
        let eight = self.get_len_notes(7)[0];

        let five_segments = self.get_len_notes(5);
        let six_segments = self.get_len_notes(6);

        // Map from wire to segment it is meant to light up
        // The segments are represented as the flag that represents them (see
        // const bindings): may be multiple flag is the value for that wire is
        // not determined yet
        let mut mapping: HashMap<char, u32> = HashMap::new();
        let mut rev_map: HashMap<u32, Vec<char>> = HashMap::new();

        // Deduce the first segment since one and seven must share two wires in
        // common: the uncommon wire maps to A
        for ch in &seven.0 - &one.0 {
            mapping.insert(ch, A);
            rev_map.entry(A).or_default().push(ch);
        }

        // The other two common wires map to C or F
        for ch in &seven.0 & &one.0 {
            mapping.insert(ch, C | F);
            rev_map.entry(C).or_default().push(ch);
            rev_map.entry(F).or_default().push(ch);
        }

        // The wires not in common between one and four map to B or D
        for ch in &four.0 - &one.0 {
            mapping.insert(ch, B | D);
            rev_map.entry(B).or_default().push(ch);
            rev_map.entry(D).or_default().push(ch);
        }

        // The wires not in common between eight and either four or seven map to E
        // or G
        for ch in &(&eight.0 - &four.0) - &seven.0 {
            mapping.insert(ch, E | G);
            rev_map.entry(E).or_default().push(ch);
            rev_map.entry(G).or_default().push(ch);
        }

        // The five-segment number subtracts pairwise from eight to either give
        // B or E
        for (i, j) in [(0, 1), (0, 2), (1, 2)] {
            for ch in &(&eight.0 - &five_segments[i].0) - &five_segments[j].0 {
                if mapping.get(&ch).unwrap() & B != 0 {
                    // This character is B
                    mapping.insert(ch, B);
                    for other_ch in rev_map.get(&B).unwrap() {
                        // So the other guess is D
                        if *other_ch != ch {
                            mapping.insert(*other_ch, D);
                        }
                    }
                } else if mapping.get(&ch).unwrap() & E != 0 {
                    // This character is E
                    mapping.insert(ch, E);
                    for other_ch in rev_map.get(&E).unwrap() {
                        // So the other character is G
                        if *other_ch != ch {
                            mapping.insert(*other_ch, G);
                        }
                    }
                } else {
                    panic!("Logic is wrong");
                }
            }
        }

        // Subtracting the six-segment numbers from eight would leave you with
        // either C, E or D. Already know which characters map to E and D, so
        // whichever character isn't one of those is C.
        for digit in six_segments {
            for ch in &eight.0 - &digit.0 {
                let segment = mapping.get(&ch).unwrap();
                if !(segment & D != 0 || segment & E != 0) {
                    // This character is C
                    mapping.insert(ch, C);
                    for other_ch in rev_map.get(&C).unwrap() {
                        // So the other guess is F
                        if *other_ch != ch {
                            mapping.insert(*other_ch, F);
                        }
                    }
                }
            }
        }

        // Check that the algorithm has finished and each entry in the mapping
        // only contains a single segment (no guesses left)
        for (_, segment) in mapping.iter() {
            match *segment {
                A | B | C | D | E | F | G => (),
                _ => panic!("Algorithm didn't complete"),
            }
        }

        let mut number = String::new();
        for digit in &self.digits {
            let mut mapped = 0;
            for ch in &digit.0 {
                mapped |= mapping.get(ch).unwrap();
            }

            match mapped {
                ZERO => number.push('0'),
                ONE => number.push('1'),
                TWO => number.push('2'),
                THREE => number.push('3'),
                FOUR => number.push('4'),
                FIVE => number.push('5'),
                SIX => number.push('6'),
                SEVEN => number.push('7'),
                EIGHT => number.push('8'),
                NINE => number.push('9'),
                _ => panic!("Digit didn't make a real number"),
            }
        }

        number.parse().unwrap()
    }
}

fn part_one(input: &[Display]) -> u32 {
    input
        .iter()
        .map(|d| &d.digits)
        .flatten()
        .map(|d| match d.0.len() {
            // These number of signals produce unique numbers
            2 | 3 | 4 | 7 => 1,
            _ => 0,
        })
        .sum()
}

fn part_two(input: &[Display]) -> u32 {
    let mut sum = 0;
    for display in input {
        sum += display.calculate_number();
    }
    sum
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
        let input: String =
            "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe
edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc
fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg
fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb
aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea
fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb
dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe
bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef
egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb
gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce"
                .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 26);
        assert_eq!(part_two(&inputs), 61229);
    }
}
