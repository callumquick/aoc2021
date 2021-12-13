/// Solution to Advent of Code Challenge Day 12.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAY: &str = "12";

type CaveHash = u16;

fn hash_name(s: &str) -> CaveHash {
    match s {
        "start" => 0,
        "end" => u16::MAX,
        // Make up a hash, which also gives whether it is
        // uppercase or not.
        name => {
            let hash: u16 = name
                .chars()
                .enumerate()
                .map(|(i, c)| 128u16.pow(i as u32) * (c as u16))
                .sum();
            if name == name.to_uppercase() {
                hash * 2 + 1
            } else {
                hash * 2
            }
        }
    }
}

fn is_small(h: CaveHash) -> bool {
    match h {
        0 => true,
        u16::MAX => true,
        n => (n % 2) != 1,
    }
}

type Path = Vec<CaveHash>;

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    from: CaveHash,
    to: CaveHash,
}

impl FromStr for Entry {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<CaveHash> = s.split('-').map(|s| hash_name(s)).collect();
        Ok(Self {
            from: parsed[0],
            to: parsed[1],
        })
    }
}

/// Recursive function that builds up every path from a given current path to the end.
fn find_paths(
    paths: &mut HashSet<Path>,
    leads_to: &HashMap<&CaveHash, HashSet<&CaveHash>>,
    small_visited: &HashSet<CaveHash>,
    path: Path,
    extra_small_visit: Option<CaveHash>,
    allow_small_visits: bool,
) {
    let start: CaveHash = hash_name("start");
    let end: CaveHash = hash_name("end");

    // Every cave leads to somewhere (even if it's back where you came from).
    for cave in leads_to.get(path.last().unwrap()).unwrap() {
        let mut new_path: Path = path.clone();
        new_path.push(**cave);

        if **cave == end {
            paths.insert(new_path.clone());
            continue;
        } else if **cave == start {
            continue;
        } else if is_small(**cave) {
            let mut allow_visit = false;
            let mut new_extra_small_visit = extra_small_visit;

            if small_visited.contains(cave) {
                if allow_small_visits && extra_small_visit.is_none() {
                    new_extra_small_visit = Some(**cave);
                    allow_visit = true;
                }
            } else {
                allow_visit = true;
            }

            if allow_visit {
                // This path can exist, and is not a dead end or a bad small cave visited.
                // Need to add this cave to those small caves visited and use
                // that for further recursion, otherwise can save ourselves time
                // and just use the existing info.
                let mut new_small_visited = small_visited.clone();
                new_small_visited.insert(**cave);
                find_paths(
                    paths,
                    leads_to,
                    &new_small_visited,
                    new_path,
                    new_extra_small_visit,
                    allow_small_visits,
                );
            }
        } else {
            find_paths(
                paths,
                leads_to,
                small_visited,
                new_path,
                extra_small_visit,
                allow_small_visits,
            );
        }
    }
}

fn part_one(input: &[Entry]) -> u64 {
    let mut leads_to: HashMap<_, HashSet<_>> = HashMap::new();
    for entry in input {
        // Each entry allows access both forwards and backwards, except start
        // and end.
        leads_to.entry(&entry.from).or_default().insert(&entry.to);
        leads_to.entry(&entry.to).or_default().insert(&entry.from);
    }

    let start: CaveHash = hash_name("start");

    let mut paths = HashSet::new();
    let path: Path = vec![start];
    let mut small_visited = HashSet::new();
    small_visited.insert(start);
    find_paths(&mut paths, &leads_to, &small_visited, path, None, false);

    paths.len() as u64
}

fn part_two(input: &[Entry]) -> u64 {
    let mut leads_to: HashMap<_, HashSet<_>> = HashMap::new();
    for entry in input {
        // Each entry allows access both forwards and backwards.
        leads_to.entry(&entry.from).or_default().insert(&entry.to);
        leads_to.entry(&entry.to).or_default().insert(&entry.from);
    }

    let start: CaveHash = hash_name("start");

    let mut paths = HashSet::new();
    let path: Path = vec![start];
    let mut small_visited = HashSet::new();
    small_visited.insert(start);
    find_paths(&mut paths, &leads_to, &small_visited, path, None, true);

    paths.len() as u64
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
        let input: String = "start-A
start-b
A-c
A-b
b-d
A-end
b-end"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 10);
        assert_eq!(part_two(&inputs), 36);
    }

    #[test]
    fn test_other_example() {
        let input: String = "dc-end
HN-start
start-kj
dc-start
dc-HN
LN-dc
HN-end
kj-sa
kj-HN
kj-dc"
            .to_string();

        let inputs: Vec<_> = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 19);
        assert_eq!(part_two(&inputs), 103);
    }
}
