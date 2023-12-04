use std::collections::HashMap;
use std::collections::HashSet;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve1(file: File) {
    println!("[day03 part1]");
    let mut sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let pos_colon = line.find(':').unwrap();
        let pos_bar = line.find('|').unwrap();
        let (win_nums, mynums) = (&line[pos_colon + 1..pos_bar], &line[pos_bar + 1..]);
        let to_set = |str_nums: &str| -> HashSet<usize> {
            HashSet::from_iter(str_nums.split(' ').filter_map(|num| {
                if let Ok(num) = num.parse::<usize>() {
                    Some(num)
                } else {
                    None
                }
            }))
        };
        let (win_nums, mynums) = (to_set(win_nums), to_set(mynums));
        let count = win_nums.len() + mynums.len() - win_nums.union(&mynums).count();
        let mut points = 0;
        if count > 0 {
            points = 2_i32.pow(count as u32 - 1);
        }
        sum += points;
    }
    println!("  {sum}");
}

pub fn solve2(file: File) {
    println!("[day03 part2]");
    let mut copies: HashMap<usize, usize> = HashMap::new();
    let reader = BufReader::new(file);
    for (i, line) in reader.lines().enumerate() {
        let line = line.unwrap();
        let pos_colon = line.find(':').unwrap();
        let pos_bar = line.find('|').unwrap();
        let (win_nums, mynums) = (&line[pos_colon + 1..pos_bar], &line[pos_bar + 1..]);
        let to_set = |str_nums: &str| -> HashSet<usize> {
            HashSet::from_iter(str_nums.split(' ').filter_map(|num| {
                if let Ok(num) = num.parse::<usize>() {
                    Some(num)
                } else {
                    None
                }
            }))
        };
        let (win_nums, mynums) = (to_set(win_nums), to_set(mynums));
        let nsame = win_nums.len() + mynums.len() - win_nums.union(&mynums).count();
        let ninstance = match copies.get(&i) {
            Some(n) => *n,
            None => {
                copies.insert(i, 1);
                1
            }
        };
        for k in i + 1..i + nsame + 1 {
            match copies.get_mut(&k) {
                Some(n) => *n += ninstance,
                None => {
                    copies.insert(k, 1 + ninstance);
                }
            }
        }
    }
    println!("  {}", copies.values().sum::<usize>());
}
