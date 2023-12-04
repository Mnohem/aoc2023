#![feature(lazy_cell)]
use std::sync::LazyLock;

static INPUT: LazyLock<String> = LazyLock::new(|| helper::easy_aoc_input());

fn main() {
    println!("Solution for Day 2, Part 1: {}", day2_part1());
    println!("Solution for Day 2, Part 2: {}", day2_part2());
}

fn day2_part2() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let rounds = line
                .split_once(':')
                .map(|(_, s)| {
                    s
                })
                .unwrap();

            let max_set = rounds
                .split(';')
                .flat_map(|set| set.split(',').map(|s| s.trim().split_once(' ')))
                .flatten()
                .fold((0, 0, 0), |(max_r, max_g, max_b), (num, color)| {
                    let num = num.parse::<u64>().unwrap();
                    match color {
                        "red" if max_r < num => (num, max_g, max_b),
                        "green" if max_g < num => (max_r, num, max_b),
                        "blue" if max_b < num => (max_r, max_g, num),
                        _ => (max_r, max_g, max_b),
                    }
                });
            max_set.0 * max_set.1 * max_set.2
        })
        .sum()
}

fn day2_part1() -> usize {
    INPUT
        .lines()
        .enumerate()
        .map(|(index, line)| {
            let index = index + 1;
            let rounds = line
                .split_once(':')
                .map(|(_, s)| {
                    s
                })
                .unwrap();

            let possible = rounds
                .split(';')
                .flat_map(|set| set.split(',').map(|s| s.trim().split_once(' ')))
                .flatten()
                .all(|(num, color)| {
                    let num = num.parse::<u8>().unwrap();
                    match color {
                        "red" => num <= 12,
                        "green" => num <= 13,
                        "blue" => num <= 14,
                        _ => unreachable!(),
                    }
                });
            if possible {
                index
            } else {
                0
            }
        })
        .sum()
}
