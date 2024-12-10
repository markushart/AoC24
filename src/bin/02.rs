use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "02";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9
";

const EXPECTED1: usize = 2;
const EXPECTED2: usize = 4;

fn diff(telegram: &Vec<isize>) -> Vec<isize> {
    telegram
        .windows(2)
        .map(|w| w[1] - w[0])
        .collect::<Vec<isize>>()
}

fn is_safe(telegram: &Vec<isize>) -> Result<bool> {
    let diffs = diff(&telegram);
    print!("{:?} - {:?}", telegram, diffs);

    // check if all positive
    let ascending = diffs.iter().all(|&d| d > 0);
    // check if all negative
    let descending = diffs.iter().all(|&d| d < 0);

    // if neither ascending nor descending
    if !ascending && !descending {
        Ok(false)
    } else {
        // absolute differences squared
        // squared differences are less than 9 -> abs dif < 3
        let safe = diffs.iter().map(|&d| d * d).all(|d| d <= 9);
        Ok(safe)
    }
}

fn is_safe_damped(telegram: &Vec<isize>) -> Result<bool> {
    let diffs = diff(&telegram);
    print!("{:?} - {:?}", telegram, diffs);

    // check if all positive
    let ascending = diffs.iter().all(|&d| d > 0);
    // check if all negative
    let descending = diffs.iter().all(|&d| d < 0);

    Ok(false)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader
            .lines()
            .map(|line| {
                // Solve Part 1 of the puzzle
                let line_nums = line.unwrap();
                let telegram = line_nums
                    .split_whitespace()
                    .map(|s| isize::from_str_radix(s, 10).unwrap())
                    .collect();

                is_safe(&telegram).unwrap() as usize
            })
            .sum();

        Ok(safe_count)
    }

    // TEST result
    assert_eq!(EXPECTED1, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R) -> Result<usize> {
        let safe_count = reader
            .lines()
            .map(|line| {
                // Solve Part 1 of the puzzle
                let line_nums = line.unwrap();
                let telegram = line_nums
                    .split_whitespace()
                    .map(|s| isize::from_str_radix(s, 10).unwrap())
                    .collect();

                is_safe_damped(&telegram).unwrap() as usize
            })
            .sum();

        Ok(safe_count)
    }

    // TEST result
    assert_eq!(EXPECTED2, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
