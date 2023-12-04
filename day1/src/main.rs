#![feature(lazy_cell)]
use std::sync::LazyLock;

static INPUT: LazyLock<String> = LazyLock::new(|| helper::easy_aoc_input());
static _TEST: &str = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
static WORDS: [&str; 9] = [
    "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

fn main() {
    println!("Solution for Day 1, Part 1: {}", day1_part1());
    println!("Solution for Day 1, Part 2: {}", day1_part2());
}

fn day1_part2() -> u64 {
    INPUT
        .lines()
        .map(|line| {
            let mut numbers = line
                .match_indices(char::is_numeric)
                .map(|(idx, x)| (idx, x.parse::<u64>().unwrap()));
            let word_numbers = WORDS
                .into_iter()
                .flat_map(|word| line.match_indices(word))
                .map(|(idx, word)| (idx, word_to_num(word).unwrap()));

            let first_number = numbers.next();
            let last_number = numbers.last().or(first_number);
            let first_word_number = word_numbers.clone().reduce(|(acc_idx, acc_x), (idx, x)| {
                if acc_idx > idx {
                    (idx, x)
                } else {
                    (acc_idx, acc_x)
                }
            });
            let last_word_number = word_numbers
                .reduce(|(acc_idx, acc_x), (idx, x)| {
                    if acc_idx < idx {
                        (idx, x)
                    } else {
                        (acc_idx, acc_x)
                    }
                })
                .or(first_word_number);

            let first = first_number
                .and_then(|(idx, x)| {
                    first_word_number
                        .map(|(w_idx, w_x)| if idx < w_idx { x } else { w_x })
                        .or(Some(x))
                })
                .unwrap();
            let last = last_number
                .and_then(|(idx, x)| {
                    last_word_number
                        .map(|(w_idx, w_x)| if idx > w_idx { x } else { w_x })
                        .or(Some(x))
                })
                .unwrap();

            first * 10 + last
        })
        .sum()
}

fn word_to_num(word: &str) -> Option<u64> {
    Some(match word {
        "one" => 1,
        "two" => 2,
        "three" => 3,
        "four" => 4,
        "five" => 5,
        "six" => 6,
        "seven" => 7,
        "eight" => 8,
        "nine" => 9,
        _ => return None,
    })
}

fn day1_part1() -> u64 {
    INPUT
        .lines()
        .flat_map(|line| {
            let mut numbers = line
                .matches(char::is_numeric)
                .flat_map(|x| x.parse::<u64>().ok());
            numbers
                .next()
                .and_then(|x| numbers.last().map(|y| x * 10 + y).or(Some(x * 10 + x)))
        })
        .sum()
}
