/// Solution to Advent of Code Challenge Day 12.
use aoc2021::{get_day_input, parse_input_lines, parse_input_with, print_elapsed_time};
use std::collections::{HashMap, HashSet};
use std::io;
use std::str::FromStr;

const DAY: &str = "12";

#[derive(Debug, Clone, PartialEq, Hash, Eq)]
struct Cave {
    name: String,
    big: bool,
}

impl FromStr for Cave {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self {
            name: s.to_owned(),
            big: s.to_uppercase() == s,
        })
    }
}

type Path = Vec<Cave>;

#[derive(Debug, Clone, PartialEq)]
struct Entry {
    from: Cave,
    to: Cave,
}

impl FromStr for Entry {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parsed: Vec<Cave> = parse_input_with(s, |s| s.split('-'));
        Ok(Self {
            from: parsed[0].to_owned(),
            to: parsed[1].to_owned(),
        })
    }
}

/// Recursive function that builds up every path from a given current path to the end.
fn find_paths(
    paths: &mut HashSet<Path>,
    leads_to: &HashMap<&Cave, Vec<&Cave>>,
    small_visited: &HashSet<Cave>,
    path: Path,
    extra_small_visit: Option<Cave>,
    allow_small_visits: bool,
) {
    let start: Cave = Cave {
        name: String::from("start"),
        big: false,
    };
    let end: Cave = Cave {
        name: String::from("end"),
        big: false,
    };

    // Every cave leads to somewhere (even if it's back where you came from).
    for cave in leads_to.get(path.last().unwrap()).unwrap() {
        let mut new_path: Path = path.clone();
        new_path.push((*cave).to_owned());

        if **cave == end {
            paths.insert(new_path.clone());
            continue;
        } else if **cave == start {
            continue;
        } else if !cave.big {
            let mut allow_visit = false;
            let mut new_extra_small_visit = extra_small_visit.clone();

            if small_visited.contains(cave) {
                if allow_small_visits && extra_small_visit.is_none() {
                    new_extra_small_visit = Some((*cave).to_owned());
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
                new_small_visited.insert((*cave).to_owned());
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
                extra_small_visit.clone(),
                allow_small_visits,
            );
        }
    }
}

fn part_one(input: &[Entry]) -> u64 {
    let mut leads_to: HashMap<_, Vec<_>> = HashMap::new();
    for entry in input {
        // Each entry allows access both forwards and backwards, except start
        // and end.
        leads_to.entry(&entry.from).or_default().push(&entry.to);
        leads_to.entry(&entry.to).or_default().push(&entry.from);
    }

    let start: Cave = Cave {
        name: String::from("start"),
        big: false,
    };

    let mut paths = HashSet::new();
    let path: Path = vec![start.to_owned()];
    let mut small_visited = HashSet::new();
    small_visited.insert(start);
    find_paths(&mut paths, &leads_to, &small_visited, path, None, false);

    paths.len() as u64
}

fn part_two(input: &[Entry]) -> u64 {
    let mut leads_to: HashMap<_, Vec<_>> = HashMap::new();
    for entry in input {
        // Each entry allows access both forwards and backwards.
        leads_to.entry(&entry.from).or_default().push(&entry.to);
        leads_to.entry(&entry.to).or_default().push(&entry.from);
    }

    let start: Cave = Cave {
        name: String::from("start"),
        big: false,
    };

    let mut paths = HashSet::new();
    let path: Path = vec![start.to_owned()];
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
