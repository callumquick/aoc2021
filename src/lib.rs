/// Shared library functionality for the Advent of Code challenges.
///
/// Public API should be accessible within compiled binaries.
///
use std::fmt::Debug;
use std::fs;
use std::str::FromStr;
use std::time::Instant;

/// Time a closure in microseconds and print the results.
pub fn print_elapsed_time<T, F>(function: F) -> T
where
    F: Fn() -> T,
{
    let now = Instant::now();
    let ret = function();
    println!("Took {}µs.", now.elapsed().as_micros());
    ret
}

/// Get a string read from a file in the "input" folder.
pub fn get_day_input(day: &'static str) -> String {
    let input_file = format!("input/{}.txt", day);
    fs::read_to_string(&input_file).expect(&format!("Could not read input file {}", &input_file))
}

/// Get a collection of parseables from an input string.
pub fn parse_input_lines<C, T>(input: String) -> C
where
    C: FromIterator<T>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    input
        .lines()
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect()
}
