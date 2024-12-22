use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "11";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
125 17
";

const _TEST2: &str = "\
125 17
253000 1 7
253 0 2024 14168
512072 1 20 24 28676032
512 72 2024 2 0 2 4 2867 6032
1036288 7 2 20 24 4048 1 4048 8096 28 67 60 32
2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2
";

const EXPECTED1: usize = 55312;

fn split_stone_rec(stone: &str, max_depth: usize) -> Result<usize> {
    if max_depth == 0 {
        // break recursion
        // println!("{}: {}", max_depth, stone);
        Ok(1)
    } else {
        // cut leading zeros
        let val = stone.parse::<usize>().unwrap();
        let _stone = val.to_string();
        let l = _stone.len();
        // println!("{}: {}, {}", max_depth, stone, l);
        if _stone == "0" {
            split_stone_rec("1", max_depth - 1)
        } else if l % 2 == 0 {
            let nstones = split_stone_rec(&_stone[..l / 2], max_depth - 1).unwrap()
                + split_stone_rec(&_stone[l / 2..], max_depth - 1).unwrap();
            Ok(nstones)
        } else {
            split_stone_rec(&(val * 2024).to_string(), max_depth - 1)
        }
    }
}

fn split_stone_rec_cache(
    stone: &str,
    max_depth: usize,
    cache: &mut HashMap<(usize, usize), usize>,
) -> Result<usize> {
    // println!("{} {}", max_depth, stone);
    let val = stone.parse::<usize>().unwrap();
    if max_depth == 0 {
        // break recursion
        Ok(1)
    } else if cache.contains_key(&(val, max_depth)) {
        // if we seen this stone before, we know how many stones it will split into
        let cache_val = *cache.get(&(val, max_depth)).unwrap();
        Ok(cache_val)
    } else {
        // cut leading zeros
        let _stone = val.to_string();
        let l = _stone.len();
        let mut nstones = 0;
        if _stone == "0" {
            nstones = split_stone_rec_cache("1", max_depth - 1, cache).unwrap();
        } else if l % 2 == 0 {
            let left = _stone[..l / 2].to_string();
            let nleft = split_stone_rec_cache(&left, max_depth - 1, cache).unwrap();
            let right = _stone[l / 2..].to_string();
            let nright = split_stone_rec_cache(&right, max_depth - 1, cache).unwrap();
            nstones = nleft + nright;
        } else {
            let nstr = &(val * 2024).to_string();
            nstones = split_stone_rec_cache(nstr, max_depth - 1, cache).unwrap();
        }
// i guess, this does not work because cache gets updated before final result is calculated?
        let _existed = cache.insert((val, max_depth), nstones);
        match _existed {
            Some(_x) => ( 
                // println!("{}: {} -> {}", val, _x, nstones)
            ),
            None => (),
        }
        Ok(nstones)
    }
}

fn split_stone_rec2(stone: &str, max_depth: usize) -> Result<usize> {
    let mut cache = HashMap::new();
    split_stone_rec_cache(stone, max_depth, &mut cache)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R) -> Result<usize> {
        // Solve Part 1 of the puzzle
        // collect stones
        let mut stones = Vec::new();
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            line.split_whitespace().for_each(|stone| {
                stones.push(stone.trim().to_string());
            });
        });

        // recursively split stones
        let mut nstones = 0;
        for stone in stones.iter() {
            nstones += split_stone_rec(stone, 25).unwrap();
        }

        Ok(nstones)
    }

    // TEST result 1
    assert_eq!(EXPECTED1, part1(BufReader::new(TEST.as_bytes()))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file)?);
    println!("Result = {}", result);
    // endregion

    // region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(reader: R, max_depth: usize) -> Result<usize> {
        // Solve Part 1 of the puzzle
        // collect stones
        let mut stones = Vec::new();
        reader.lines().for_each(|line| {
            let line = line.unwrap();
            line.split_whitespace().for_each(|stone| {
                stones.push(stone.trim().to_string());
            });
        });

        // recursively split stones
        let mut nstones = 0;
        for stone in stones.iter() {
            nstones += split_stone_rec2(stone, max_depth).unwrap();
        }

        Ok(nstones)
    }

    // TEST result 2
    assert_eq!(EXPECTED1, part2(BufReader::new(TEST.as_bytes()), 25)?);

    println!("Running part 2 with max depth {}", 75);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, 75)?);
    println!("Result = {}", result);
    // endregion

    Ok(())
}
