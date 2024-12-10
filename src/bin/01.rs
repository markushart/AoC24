use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::fs::File;
use std::io::{BufRead, BufReader};
use regex::Regex;
use std::isize;
use itertools::izip;

const DAY: &str = "01"; // TODO: Fill the day
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
3  4
4  3
2  5
1  3
3  9
3  3
";

const EXPECTED1: usize = 11;
const EXPECTED2: usize = 31;

fn split_lists<R: BufRead>(reader: R) -> Result<(Vec<isize>, Vec<isize>)> {
    let mut left = Vec::new();
    let mut right = Vec::new();

    let re = Regex::new(r"(\d+)\s+(\d+)")?;

    for line in reader.lines() {
        let line = line?;
        let cap = re.captures(&line).unwrap();
        let l_val = isize::from_str_radix(cap.get(1).unwrap().as_str(), 10)?;
        let r_val = isize::from_str_radix(cap.get(2).unwrap().as_str(), 10)?;
        left.push(l_val);
        right.push(r_val);
    }

    Ok((left, right))
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Solve Part 1 of the puzzle
        let (mut col1, mut col2) = split_lists(reader)?;

        // sort each vector
        col1.sort();
        col2.sort();

        // get the elementwise total difference between the two vectors
        if col1.len() != col2.len() {
            panic!("Vectors are not the same length");
        }

        let answer = col1
            .iter()
            .zip(col2.iter())
            .map(|(a, b)| {isize::abs(a -b )} as usize)
            .sum();

        Ok(answer)
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
        let (mut col1, mut col2) = split_lists(reader)?;

        // sort each vector
        col1.sort();
        col2.sort();

        // step 1: get unique elements of col1 and
        //         the number of their occurences from col1
        let mut col1_uni: Vec<isize> = Vec::new();
        let mut col1_num: Vec<isize> = Vec::new();
        col1.iter().for_each(|&x| {
            if !col1_uni.contains(&x) {
                col1_uni.push(x);
                col1_num.push(1);
            } else {
                let len = col1_num.len() - 1;
                col1_num[len] += 1;
            }
        });

        // step 2: get the number of occurences of col1 elements in col2
        let mut col2_num: Vec<isize> = Vec::new();
        col1_uni.iter().for_each(|&x| {
            let count2 = col2.iter().filter(|&y| *y == x).count();
            col2_num.push(count2 as isize);
        });

        // step 3: get the similarity score as described in AoC
        let zipped = izip!(col1_uni.iter(), col1_num.iter(), col2_num.iter());
        let sim_score = zipped
            .map(|(u1, n1, n2)| {
                let result = u1 * n1 * n2;
                // println!("{} * {} * {} = {}", u1, n1, n2, result);
                result
            })
            .sum::<isize>();

        Ok(sim_score as usize)
    }

    // TEST result
    assert_eq!(EXPECTED2, part2(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file)?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
