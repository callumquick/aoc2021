use std::collections::HashSet;
use std::ops::RangeInclusive;

/// Solution to Advent of Code Challenge Day 17.
use aoc2021::{get_day_input, print_elapsed_time};
use std::io;
use std::str::FromStr;

const DAY: &str = "17";

#[derive(Debug, Clone)]
struct TargetArea {
    xrange: RangeInclusive<isize>,
    yrange: RangeInclusive<isize>,
}

impl FromStr for TargetArea {
    type Err = io::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let x_idx = s.find('x').unwrap();
        let y_idx = s.find('y').unwrap();
        let comma = s.find(',').unwrap();

        let xs: Vec<isize> = s[x_idx + 2..comma]
            .split("..")
            .map(|s| s.parse().unwrap())
            .collect();
        let ys: Vec<isize> = s[y_idx + 2..]
            .split("..")
            .map(|s| s.parse().unwrap())
            .collect();

        Ok(Self {
            xrange: xs[0]..=xs[1],
            yrange: ys[0]..=ys[1],
        })
    }
}

fn part_one(input: &TargetArea) -> i64 {
    // The highest dy would be such that by the time it crossed back over y=0 it
    // enough velocity to carry it past the target area. Since it's symmetric
    // that would be when the initial upwards velocity is more than the distance
    // between the start and the edge of the target area.
    // The highest dy which lands in the area would be one less than this velocity.
    let dy_init_max = isize::abs(*input.yrange.start()) - 1;

    let mut dy = dy_init_max;
    let mut y = 0;
    let mut y_max = 0;
    while y > *input.yrange.start() {
        y += dy;
        dy -= 1;
        if y > y_max {
            y_max = y;
        }
    }
    y_max as i64
}

fn part_two(input: &TargetArea) -> u64 {
    let dy_init_max = input.yrange.clone().map(|y| y.abs()).max().unwrap();
    let dx_init_max = input.xrange.clone().max().unwrap();

    let mut good_inits = HashSet::new();
    for dx_init in (-dx_init_max)..=dx_init_max {
        for dy_init in (-dy_init_max)..=dy_init_max {
            let mut x = 0;
            let mut y = 0;
            let mut dx = dx_init;
            let mut dy = dy_init;

            while y > *input.yrange.start() && x < *input.xrange.end() {
                x += dx;
                y += dy;
                if dx != 0 {
                    dx -= dx.signum();
                }
                dy -= 1;

                if input.yrange.contains(&y) && input.xrange.contains(&x) {
                    good_inits.insert((dx_init, dy_init));
                }
            }
        }
    }

    good_inits.len() as u64
}

fn main() {
    let input = get_day_input(DAY);
    let inputs = input.parse().unwrap();
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
        let area = "target area: x=20..30, y=-10..-5".parse().unwrap();

        // Check each gives the right answer.
        assert_eq!(part_one(&area), 45);
        assert_eq!(part_two(&area), 112);
    }
}
