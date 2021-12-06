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
    fs::read_to_string(&input_file)
        .unwrap_or_else(|_| panic!("Could not read input file {}", &input_file))
}

/// Get a collection of parseables from an input string, separating it by a
/// generic function.
pub fn parse_input_with<'a, F, S, C, T>(input: &'a str, with: F) -> C
where
    F: Fn(&'a str) -> S,
    S: Iterator<Item = &'a str>,
    C: FromIterator<T>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    with(input)
        .map(|s| s.parse().expect("Failed to parse input"))
        .collect()
}

/// Get a collection of parseables from an input string splitting on lines.
pub fn parse_input_lines<C, T>(input: &str) -> C
where
    C: FromIterator<T>,
    T: FromStr,
    <T as FromStr>::Err: Debug,
{
    parse_input_with(input, str::lines)
}
