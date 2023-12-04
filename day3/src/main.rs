#![feature(lazy_cell)]
#![feature(iter_map_windows)]
use std::{collections::HashSet, sync::LazyLock};

static INPUT: LazyLock<String> = LazyLock::new(|| helper::easy_aoc_input());

fn main() {
    println!("Solution for Day 3, Part 1: {}", day3_part1());
    println!("Solution for Day 3, Part 2: {}", day3_part2());
}

fn day3_part2() -> u64 {
    return 0;
}

fn day3_part1() -> usize {
    // must include first and last line
    let mut input = (0..140).map(|_| '.').collect::<String>() + &INPUT.to_owned();
    input.extend((0..140).map(|_| '.'));

    input
        .lines()
        .map_windows(|[prev, current, next]| {
            let mut sum = 0;
            let mut skips = HashSet::new();
            for (idx, _) in prev
                .match_indices(is_symbol)
                .chain(next.match_indices(is_symbol))
            {
                if skips.contains(&idx) {
                    continue;
                };
                let current = current.as_bytes();

                let range = if (b'0'..=b'9').contains(&current[idx]) {
                    num_bounds(current, idx)
                } else if matches!(current.get(idx + 1), Some(b'0'..=b'9')) {
                    num_bounds(current, idx + 1)
                } else if idx != 0 && matches!(current.get(idx - 1), Some(b'0'..=b'9')) {
                    num_bounds(current, idx - 1)
                } else {
                    continue;
                };

                range.clone().for_each(|x| {
                    skips.insert(x);
                });
                sum += std::str::from_utf8(&current[range])
                    .unwrap()
                    .parse::<usize>()
                    .unwrap();
            }
            sum
        })
        .sum()
}

fn is_symbol(c: char) -> bool {
    match c as u8 {
        b'.' | b'0'..=b'9' => false,
        _ => true,
    }
}

fn num_bounds(arr: &[u8], idx: usize) -> std::ops::Range<usize> {
    let mut i = idx;
    while i != 0 && matches!(arr.get(i - 1), Some(b'0'..=b'9')) {
        i -= 1;
    }
    let mut j = idx;
    while matches!(arr.get(j), Some(b'0'..=b'9')) {
        j += 1;
    }
    i..j
}
