/// Solution to Advent of Code Challenge Day 14.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::collections::HashMap;
use std::io;
use std::str::FromStr;

const DAY: &str = "14";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Element(char);

impl FromStr for Element {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.chars().next().unwrap()))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Pair([Element; 2]);

impl FromStr for Pair {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let chars = s.chars().collect::<Vec<_>>();
        Ok(Self([Element(chars[0]), Element(chars[1])]))
    }
}

#[derive(Debug, Clone, PartialEq)]
struct Insertion {
    between: Pair,
    insert: Element,
}

impl FromStr for Insertion {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<String> = parse_input_with(s, |s| s.split(" -> "));
        Ok(Self {
            between: parsed[0].parse().unwrap(),
            insert: parsed[1].parse().unwrap(),
        })
    }
}

#[derive(Debug, Clone)]
struct Instructions {
    start: Vec<Element>,
    insertions: Vec<Insertion>,
}

fn calculate_element_difference(input: &Instructions, steps: usize) -> u64 {
    let polymer = input.start.clone();

    // Track which two pairs will be created from a given pair (when the element
    // is inserted between the two elements in the pair)
    let mut pair_inserts: HashMap<Pair, Element> = HashMap::new();
    let mut pair_creates: HashMap<Pair, [Pair; 2]> = HashMap::new();
    for insertion in &input.insertions {
        let new_pairs = [
            Pair([insertion.between.0[0], insertion.insert]),
            Pair([insertion.insert, insertion.between.0[1]]),
        ];
        pair_inserts.insert(insertion.between, insertion.insert);
        pair_creates.insert(insertion.between, new_pairs);
    }

    // Track which pairs exist in the starting formula
    let mut pair_counts: HashMap<Pair, u64> = HashMap::new();
    for window in polymer.windows(2) {
        let pair = Pair([window[0], window[1]]);
        *pair_counts.entry(pair).or_insert(0) += 1;
    }

    // Track the counts of each element
    let mut element_counts: HashMap<Element, u64> = HashMap::new();
    for element in &polymer {
        *element_counts.entry(*element).or_insert(0) += 1;
    }

    for _step in 0..steps {
        let original_pair_counts = pair_counts.clone();
        for (pair, count) in original_pair_counts.iter() {
            let inserts = pair_inserts.get(pair);
            if let Some(e) = inserts {
                // Add in that number of elements
                *element_counts.entry(*e).or_insert(0) += count;
                // Add in that number of each created pair
                let created = pair_creates.get(pair).unwrap();
                *pair_counts.entry(created[0]).or_insert(0) += count;
                *pair_counts.entry(created[1]).or_insert(0) += count;
                // Remove all the now split pairs
                pair_counts.entry(*pair).and_modify(|c| *c -= count);
            }
        }
    }

    element_counts.values().max().unwrap() - element_counts.values().min().unwrap()
}

fn part_one(input: &Instructions) -> u64 {
    calculate_element_difference(input, 10)
}

fn part_two(input: &Instructions) -> u64 {
    calculate_element_difference(input, 40)
}

fn get_instructions(input: &str) -> Instructions {
    let inputs: Vec<String> = parse_input_with(input, |s| s.split("\n\n"));
    Instructions {
        start: parse_input_with(&inputs[0], |s| s.split("").filter(|s| !s.is_empty())),
        insertions: parse_input_lines(&inputs[1]),
    }
}

fn main() {
    let input = get_day_input(DAY);
    let inputs = get_instructions(&input);
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
        let input: String = "NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C"
            .to_string();

        let inputs = get_instructions(&input);

        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 1588);
        assert_eq!(part_two(&inputs), 2188189693529);
    }
}
