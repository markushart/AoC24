use adv_code_2024::*;
use anyhow::*;
use code_timing_macros::time_snippet;
use const_format::concatcp;
use itertools::Itertools;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader};

const DAY: &str = "20";
const INPUT_FILE: &str = concatcp!("input/", DAY, ".txt");

const TEST: &str = "\
###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############
";

const EXPECTED1: usize = 44;

const EXPTECTED2: [(usize, usize); 14] = [
    (50, 32),
    (52, 31),
    (54, 29),
    (56, 39),
    (58, 25),
    (60, 23),
    (62, 20),
    (64, 19),
    (66, 12),
    (68, 14),
    (70, 12),
    (72, 22),
    (74, 4),
    (76, 3),
];

#[derive(PartialEq, Debug)]
enum Cell {
    Start,
    End,
    Wall,
    Track,
}
impl fmt::Display for Cell {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Cell::Start => return write!(f, "S"),
            Cell::End => return write!(f, "E"),
            Cell::Wall => return write!(f, "#"),
            Cell::Track => return write!(f, "."),
        };
    }
}
#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Coord {
    x: usize,
    y: usize,
}

#[derive(PartialEq, PartialOrd, Clone, Debug)]
struct Cheat {
    from: usize,
    to: usize,
}

pub fn argsort<T: Ord>(data: &[T]) -> Vec<usize> {
    let mut indices = (0..data.len()).collect::<Vec<_>>();
    indices.sort_by_key(|&i| &data[i]);
    indices
}

fn search_field(map: &Vec<Vec<Cell>>, field: &Cell) -> Result<Coord, &'static str> {
    for (r, row) in map.iter().enumerate() {
        for (c, cell) in row.iter().enumerate() {
            if *cell == *field {
                return Result::Ok(Coord { x: r, y: c });
            }
        }
    }
    return Result::Err("Field not found");
}

fn path_len(map: &Vec<Vec<Cell>>) -> Result<usize> {
    let ntracks = map.iter().flatten().filter(|c| **c == Cell::Track).count();
    Ok(ntracks + 2)
}

fn get_path(map: &Vec<Vec<Cell>>, start: &Coord) -> Result<Vec<Coord>, &'static str> {
    let mut path = Vec::new();

    let dirs: [(isize, isize); 4] = [(0, 1), (0, -1), (-1, 0), (1, 0)];
    let map_shape = Coord {
        x: map.len(),
        y: map[0].len(),
    };
    let mut loop_cnt = (map_shape.x * map_shape.y) as isize;

    path.push(start.clone());
    'inf_loop: while loop_cnt > 0 {
        let pos = path.last().unwrap();
        let mut npos: Coord;
        'for_dirs: for (dx, dy) in dirs.iter() {
            // make sure we do not leave bounds of map
            if pos.x < 1 || pos.y < 1 || pos.x >= map_shape.x - 1 || pos.y >= map_shape.y - 1 {
                continue;
            }

            npos = Coord {
                x: (pos.x as isize + dx) as usize,
                y: (pos.y as isize + dy) as usize,
            };

            if !path.contains(&npos) {
                match map[npos.x][npos.y] {
                    Cell::Track => {
                        path.push(npos);
                        break 'for_dirs;
                    }
                    Cell::End => {
                        path.push(npos);
                        break 'inf_loop;
                    }
                    _ => {
                        // println!("no match on {:?}: {:?}", npos, map[npos.x][npos.y])
                    }
                }
            }
        }
    }

    if loop_cnt <= 0 {
        Result::Err("infinite loop detected")
    } else {
        Result::Ok(path)
    }
}

fn coord_cmp(a: &Coord, b: &Coord, axis: Option<usize>) -> Ordering {
    fn coord_cmp_u(a1: &usize, a2: &usize, b1: &usize, b2: &usize) -> Ordering {
        if a1 < b1 || (a1 == b1 && a2 < b2) {
            Ordering::Less
        } else if a1 == b1 && a2 == b2 {
            Ordering::Equal
        } else {
            Ordering::Greater
        }
    }

    let dir = match axis {
        Some(v) => v,
        None => 0,
    };

    if dir == 0 {
        coord_cmp_u(&a.x, &a.y, &b.x, &b.y)
    } else {
        coord_cmp_u(&a.y, &a.x, &b.y, &b.x)
    }
}

fn get_if_cheat(
    xa: &usize,
    ia: &usize,
    xb: &usize,
    ib: &usize,
    cd: &usize,
    cl: &usize,
) -> Option<Cheat> {
    let dy = *xa as isize - *xb as isize;
    let dy2 = (dy * dy) as usize;

    // the saved distance magnitude is 1 less,
    // because of the travel through the wall
    let mut saved_dist = *ia as isize - *ib as isize;
    if saved_dist < 0 {
        saved_dist = (saved_dist + *cd as isize) as isize;
    } else {
        saved_dist = (saved_dist - *cd as isize) as isize;
    }
    let sd2 = (saved_dist * saved_dist) as usize;

    // println!("{} {}", dy, saved_dist);
    if (dy2 == cd * cd) && (sd2 >= cl * cl) {
        // push the indices of the tuple in path
        // println!("push");
        if ia < ib {
            return Some(Cheat { from: *ia, to: *ib });
        } else {
            // return Some(Cheat { from: *ib, to: *ia });
            return Some(Cheat { from: *ia, to: *ib });
        }
    }
    None
}

fn get_cheats(
    path: &Vec<Coord>,
    map: &Vec<Vec<Cell>>,
    cheat_lim: Option<usize>,
) -> Result<Vec<Cheat>> {
    // path is a sequence of coordinates telling you the path
    // from start to end,
    // cheat_lim checks if the cheat saves enough pico seconds to be relevant
    let cl = cheat_lim.unwrap_or(1);

    let mut cheats = Vec::new();

    // 1. lexicographic sort of path (order  x, y)
    // 2. iter rows, find which Tracks are 2 fields appart and seperated by a wall
    //    1. split into slices where row idx is equal
    //    2. forward diff
    //    3. filter for abs(diff) == 2 -> thes are the cheats in x-dir
    // 3. check the length we could safe by using this cheat
    //    1. get index of cheat.from and cheat.to in path
    //    2. diff in indices is saved path length
    // 4. filter for length >= cheat_lim
    // 5. repeat for y (y, x)

    const CHEAT_DIST: usize = 2;

    // cheats in y direction
    path.iter()
        .enumerate()
        .sorted_by(|(ia, a), (ib, b)| coord_cmp(a, b, Some(0)))
        .tuple_windows()
        .filter(|((ia, a), (ib, b))| a.x == b.x)
        .for_each(|((ia, a), (ib, b))| {
            match get_if_cheat(&a.y, &ia, &b.y, &ib, &CHEAT_DIST, &cl) {
                Some(v) => cheats.push(v),
                None => {}
            }
        });

    // cheats in x direction
    path.iter()
        .enumerate()
        .sorted_by(|(ia, a), (ib, b)| coord_cmp(a, b, Some(1)))
        .tuple_windows()
        .filter(|((ia, a), (ib, b))| a.y == b.y)
        .for_each(|((ia, a), (ib, b))| {
            match get_if_cheat(&a.x, &ia, &b.x, &ib, &CHEAT_DIST, &cl) {
                Some(v) => cheats.push(v),
                None => {}
            }
        });

    // print path
    // path.iter().for_each(|c| {
    // println!("{:?}", c);
    // });

    // let mut test_map = HashMap::new();
    // cheats.iter().for_each(|c| {
    //     let dist = c.to as isize - c.from as isize - 2;
    //     match test_map.get_mut(&dist) {
    //         Some(v) => *v += 1,
    //         None => {
    //             test_map.insert(dist, 1);
    //         }
    //     }
    // });
    // // print result
    // println!("{:?}", test_map);

    // print result
    // cheats
    //     .iter()
    //     // .sorted_by_key(|c| c.from)
    //     .for_each(|c| {
    //         let mut saved_dist = c.to as isize - c.from as isize;
    //         if saved_dist < 0 {
    //             saved_dist = (saved_dist as usize + CHEAT_DIST) as isize;
    //         } else {
    //             saved_dist = (saved_dist as usize - CHEAT_DIST) as isize;
    //         }
    //         println!(
    //             "({:?}, {:?}, '{}', '{}', ia: {} ib: {}, dist: {}) ",
    //             path[c.from],
    //             path[c.to],
    //             map[path[c.from].x][path[c.from].y],
    //             map[path[c.to].x][path[c.to].y],
    //             c.from,
    //             c.to,
    //             saved_dist
    //         );
    //     });

    Result::Ok(cheats)
}

fn manhattan_dist(a: &Coord, b: &Coord) -> isize {
    let mut mdx = 0;
    let mut mdy = 0;
    if a.x < b.x {
        mdx = b.x as isize - a.x as isize;
    } else {
        mdy = a.x as isize - b.x as isize;
    }
    if a.y < b.y {
        mdy = b.y as isize - a.y as isize;
    } else {
        mdy = a.y as isize - b.y as isize;
    }
    return mdx + mdy;
}

fn coord_argsort(v: &Vec<Coord>, axis: &usize) -> Vec<usize> {
    v.iter()
        .enumerate()
        .sorted_by(|(ia, a), (ib, b)| coord_cmp(a, b, Some(*axis)))
        .map(|(i, x)| i)
        .collect()
}

fn get_cheats_rad(
    path: &Vec<Coord>,
    map: &Vec<Vec<Cell>>,
    rad: Option<usize>,
    minimum_saving: Option<usize>,
) -> Result<Vec<Cheat>> {
    // path is a sequence of coordinates telling
    // you the track positions in map from start to end,
    // rad checks possible track positons reachable from
    // the current position
    // minimum_saving filters for the mimimum distance
    // a cheat must save to be relevant
    let r = rad.unwrap_or(1);
    let r2 = r * r;
    let mins = minimum_saving.unwrap_or(0);
    let mins2 = mins * mins;
    let mut cheats = Vec::new();

    // get index sorted in x and y direction
    // let xidx = coord_argsort(path, &0);
    // let yidx = coord_argsort(path, &1);
    for (fi, from) in path.iter().enumerate() {
        // possible cheats must be within a window of +-r
        // around the current coordinate
        path.iter()
            .enumerate()
            // .filter(|(it, to)| {
            //     let dx = to.x as isize - from.x as isize;
            //     (dx * dx) as usize <= r2
            // })
            // .filter(|(it, to)| {
            //     let dy = to.y as isize - from.y as isize;
            //     (dy * dy) as usize <= r2
            // })
            .for_each(|(ti, to)| {
                // manhattan distance
                let md = manhattan_dist(from, to);
                let md2 = (md * md) as usize;
                // coord should be reachable within r
                if md2 <= r2 {
                    // saved distance is distance in path index +- the
                    // cheat distance
                    let mut saved_dist = ti as isize - fi as isize;
                    if saved_dist < 0 {
                        saved_dist = (saved_dist + md as isize) as isize;
                    } else {
                        saved_dist = (saved_dist - md as isize) as isize;
                    }
                    let sd2 = (saved_dist * saved_dist) as usize;

                    // check path direction and minimum saved distance
                    if fi < ti && sd2 >= mins2 {
                        print!("{}, ", saved_dist);
                        cheats.push(Cheat { from: fi, to: ti });
                    }
                }
            });
    }

    cheats.iter().sorted_by_key(|c| c.from).for_each(|c| {
        // println!(
        //     "({:?}, {:?}, '{}', '{}', ia: {} ib: {}, dist: ) ",
        //     path[c.from],
        //     path[c.to],
        //     map[path[c.from].x][path[c.from].y],
        //     map[path[c.to].x][path[c.to].y],
        //     c.from,
        //     c.to,
        // );
    });

    Ok(cheats)
}

fn main() -> Result<()> {
    start_day(DAY);

    //region Part 1
    println!("=== Part 1 ===");

    fn part1<R: BufRead>(reader: R, cheat_lim: Option<usize>) -> Result<usize> {
        let map = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        '#' => Cell::Wall,
                        '.' => Cell::Track,
                        _ => Cell::Wall,
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();

        // get picoseconds a cheat must save to get captured
        let cl = match cheat_lim {
            Some(v) => v,
            None => 1,
        };

        // get start location
        let start = search_field(&map, &Cell::Start).unwrap();
        println!("start: {:?}", start);

        // find the path
        let path = get_path(&map, &start).unwrap();
        let cheats = get_cheats(&path, &map, Some(cl));

        Ok(cheats.unwrap().len())
    }

    // TEST result 1
    assert_eq!(EXPECTED1, part1(BufReader::new(TEST.as_bytes()), Some(0))?);

    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part1(input_file, Some(100))?);
    println!("Result = {}", result);
    //endregion

    //region Part 2
    println!("\n=== Part 2 ===");

    fn part2<R: BufRead>(
        reader: R,
        radius: Option<usize>,
        minimum_saving: Option<usize>,
    ) -> Result<usize> {
        let map = reader
            .lines()
            .map(|l| {
                l.unwrap()
                    .chars()
                    .map(|c| match c {
                        'S' => Cell::Start,
                        'E' => Cell::End,
                        '#' => Cell::Wall,
                        '.' => Cell::Track,
                        _ => Cell::Wall,
                    })
                    .collect::<Vec<Cell>>()
            })
            .collect::<Vec<Vec<Cell>>>();

        // get start and path
        let start = search_field(&map, &Cell::Start).unwrap();
        let path = get_path(&map, &start).unwrap();

        // find the cheats
        let cheats = get_cheats_rad(&path, &map, radius, minimum_saving);

        Ok(cheats.unwrap().len())
    }

    // TEST reesult 2
    let e2: usize = EXPTECTED2.into_iter().map(|(k, n)| n).sum();
    assert_eq!(
        e2,
        part2(BufReader::new(TEST.as_bytes()), Some(20), Some(50))?
    );
    
    let input_file = BufReader::new(File::open(INPUT_FILE)?);
    let result = time_snippet!(part2(input_file, Some(20), Some(100))?);
    println!("Result = {}", result);
    //endregion

    Ok(())
}
