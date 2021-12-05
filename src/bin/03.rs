/// Solution to Advent of Code Challenge Day 03.
use aoc2021::{get_day_input, parse_input_lines, print_elapsed_time};

fn get_bit_sums(input: &Vec<String>) -> Vec<(u32, u32)> {
    let bit_num = input[0].len();

    let mut bit_sums: Vec<(u32, u32)> = Vec::with_capacity(bit_num);

    for _ in 0..bit_num {
        bit_sums.push((0, 0));
    }

    for bin_str in input.iter() {
        for (i, ch) in bin_str.chars().enumerate() {
            match ch {
                '0' => bit_sums[i].0 += 1,
                '1' => bit_sums[i].1 += 1,
                _ => panic!("Input string isn't binary"),
            }
        }
    }

    bit_sums
}

fn get_bit_sum(input: &Vec<String>, char_index: usize) -> (u32, u32) {
    let mut bit_sum = (0, 0);
    for bin_str in input.iter() {
        for ch in bin_str.chars().nth(char_index) {
            match ch {
                '0' => bit_sum.0 += 1,
                '1' => bit_sum.1 += 1,
                _ => panic!("Input string isn't binary"),
            }
        }
    }

    bit_sum
}

fn get_filtered_num(mut input: Vec<String>, most_common: bool) -> String {
    let one_majority_ch = if most_common { '1' } else { '0' };
    let one_minority_ch = if most_common { '0' } else { '1' };
    for bit_idx in 0..input[0].len() {
        if input.len() == 1 {
            break;
        }

        let bit_sum = get_bit_sum(&input, bit_idx);
        let zeroes = bit_sum.0;
        let ones = bit_sum.1;

        if ones >= zeroes {
            // Only keep values with the char in position bit_idx needed whena
            // ones are the majority.
            input = input
                .into_iter()
                .filter(|num| num.chars().nth(bit_idx) == Some(one_majority_ch))
                .collect();
        } else {
            // Only keep values with the char in position bit_idx needed whena
            // ones are the minority.
            input = input
                .into_iter()
                .filter(|num| num.chars().nth(bit_idx) == Some(one_minority_ch))
                .collect();
        }
    }
    input[0].clone()
}

/// Find the most common and least common bit combinations and multiply
fn part_one(input: &Vec<String>) -> u64 {
    let mut gamma_str = String::new();
    let mut epsilon_str = String::new();

    for (zeroes, ones) in get_bit_sums(input) {
        if ones > zeroes {
            gamma_str.push('1');
            epsilon_str.push('0');
        } else {
            gamma_str.push('0');
            epsilon_str.push('1');
        }
    }

    let gamma = u64::from_str_radix(&gamma_str, 2).expect("Resulting gamma string wasn't binary");
    let epsilon =
        u64::from_str_radix(&epsilon_str, 2).expect("Resulting epsilon string wasn't binary");

    gamma * epsilon
}

/// Find the filtered numbers when considering bit majorities in each position
fn part_two(input: &Vec<String>) -> u64 {
    let og_string = get_filtered_num(input.clone(), true);
    let cs_string = get_filtered_num(input.clone(), false);

    let og = u64::from_str_radix(&og_string, 2).expect("Resulting OG string wasn't binary");
    let cs = u64::from_str_radix(&cs_string, 2).expect("Resulting CS string wasn't binary");

    og * cs
}

fn main() {
    let input = get_day_input("03");
    let inputs = parse_input_lines(&input);
    println!("Day 03:");
    println!("==========");
    println!("Part one: {}", print_elapsed_time(|| part_one(&inputs)));
    println!("Part two: {}", print_elapsed_time(|| part_two(&inputs)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_given_example() {
        let input: String = "00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010"
            .to_string();

        let inputs = parse_input_lines(&input);
        // Check each gives the right answer.
        assert_eq!(part_one(&inputs), 198);
        assert_eq!(part_two(&inputs), 230);
    }
}
