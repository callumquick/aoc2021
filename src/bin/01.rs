/// Solution to Advent of Code Challenge Day 01.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};

/// Find the number of times the depth increases between measurements.
fn part_one(input: &Vec<u32>) -> u32 {
    let mut inc_count = 0;
    for (num1, num2) in input.iter().zip(&input[1..]) {
        if num2 > num1 {
            inc_count += 1;
        }
    }
    inc_count
}

/// Find the product of the three numbers which sum to the target value.
fn part_two(input: &Vec<u32>) -> u32 {
    let mut window_sums = Vec::new();
    for window in input.windows(3) {
        window_sums.push(window.iter().sum());
    }
    part_one(&window_sums)
}

fn main() {
    let input = get_day_input("01");
    let num_list = parse_input_lines(input);
    println!("Day 01:");
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&num_list)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&num_list)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "199
200
208
210
200
207
240
269
260
263"
        .to_string();

        let num_list = parse_input_lines(input);
        // Check each gives the right answer.
        assert_eq!(part_one(&num_list), 7);
        assert_eq!(part_two(&num_list), 5);
    }
}
