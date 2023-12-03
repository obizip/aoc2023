use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(PartialEq, Eq, Clone, Copy)]
enum Kind {
    Number(char),
    Symbol(char),
    NotSymbol,
}

type Engine = Vec<Vec<Kind>>;

fn char_to_kind(c: char) -> Kind {
    match c {
        c @ '0'..='9' => Kind::Number(c),
        '.' => Kind::NotSymbol,
        c => Kind::Symbol(c),
    }
}

fn exists_symbol(engine: &Engine, (x_begin, x_end): (usize, usize), y: usize) -> bool {
    let nrows = engine.len() as isize;
    let ncols = engine[y].len() as isize;
    let left = x_begin as isize - 1;
    let right = x_end as isize + 1;
    let up = y as isize - 1;
    let down = y as isize + 1;

    for ix in left..=right {
        for iy in up..=down {
            if 0 <= ix && ix < nrows && 0 <= iy && iy < ncols {
                if let Kind::Symbol(_) = engine[iy as usize][ix as usize] {
                    return true;
                }
            }
        }
    }

    false
}

fn get_around_asts(
    engine: &Engine,
    (x_begin, x_end): (usize, usize),
    y: usize,
) -> Vec<(usize, usize)> {
    let nrows = engine.len() as isize;
    let ncols = engine[y].len() as isize;
    let left = x_begin as isize - 1;
    let right = x_end as isize + 1;
    let up = y as isize - 1;
    let down = y as isize + 1;

    let mut asts = Vec::new();

    for ix in left..=right {
        for iy in up..=down {
            if 0 <= ix && ix < nrows && 0 <= iy && iy < ncols {
                if let Kind::Symbol('*') = engine[iy as usize][ix as usize] {
                    asts.push((ix as usize, iy as usize));
                }
            }
        }
    }

    asts
}

pub fn solve1(file: File) {
    println!("[day03 part1]");
    let reader = BufReader::new(file);
    let mut engine: Vec<Vec<Kind>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        engine.push(line.chars().map(char_to_kind).collect());
    }
    let mut sum = 0;
    let ncols = engine[0].len();
    let nrows = engine.len();
    // println!("{ncols}, {nrows}");
    for y in 0..nrows {
        let mut nums = String::new();
        let mut x = 0;
        while x < ncols {
            if let Kind::Number(num) = engine[y][x] {
                nums.push(num);
                if x + 1 == ncols {
                    let x_begin = x - nums.len() + 1;
                    let x_end = x;
                    if exists_symbol(&engine, (x_begin, x_end), y) {
                        // println!("({x_begin}, {x_end})");
                        sum += nums.parse::<usize>().unwrap();
                    }
                    nums = String::new();
                }
                // println!("{nums}");
            } else if !nums.is_empty() {
                let x_begin = x - nums.len();
                let x_end = x - 1;
                if exists_symbol(&engine, (x_begin, x_end), y) {
                    // println!("({x_begin}, {x_end})");
                    sum += nums.parse::<usize>().unwrap();
                }
                nums = String::new();
            }
            x += 1
        }
    }
    println!("sum: {sum}");
}

pub fn solve2(file: File) {
    println!("[day03 part2]");
    let reader = BufReader::new(file);
    let mut engine: Vec<Vec<Kind>> = Vec::new();
    for line in reader.lines() {
        let line = line.unwrap();
        engine.push(line.chars().map(char_to_kind).collect());
    }
    let mut sum = 0;
    let ncols = engine[0].len();
    let nrows = engine.len();
    let mut asts_map: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    // println!("{ncols}, {nrows}");
    for y in 0..nrows {
        let mut nums = String::new();
        let mut x = 0;
        while x < ncols {
            if let Kind::Number(num) = engine[y][x] {
                nums.push(num);
                if x + 1 == ncols {
                    let x_begin = x - nums.len() + 1;
                    let x_end = x;
                    let asts = get_around_asts(&engine, (x_begin, x_end), y);
                    for ast in asts {
                        if let Some(arounds) = asts_map.get_mut(&ast) {
                            arounds.push(nums.parse::<usize>().unwrap());
                        } else {
                            asts_map.insert(ast, vec![nums.parse::<usize>().unwrap()]);
                        }
                    }
                    nums = String::new();
                }
                // println!("{nums}");
            } else if !nums.is_empty() {
                let x_begin = x - nums.len();
                let x_end = x - 1;
                let asts = get_around_asts(&engine, (x_begin, x_end), y);
                for ast in asts {
                    if let Some(arounds) = asts_map.get_mut(&ast) {
                        arounds.push(nums.parse::<usize>().unwrap());
                    } else {
                        asts_map.insert(ast, vec![nums.parse::<usize>().unwrap()]);
                    }
                }
                nums = String::new();
            }
            x += 1
        }
    }
    // println!("{:?}", asts_map);
    for (_, nums) in asts_map {
        if nums.len() == 2 {
            sum += nums[0] * nums[1];
        }
    }
    println!("sum: {sum}");
}
