use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

fn find_first_digit(
    s: &String,
    nums: &HashMap<String, usize>,
    minlen: usize,
    maxlen: usize,
) -> Option<usize> {
    let cs: Vec<char> = s.chars().collect();
    let mut first_digit = None;
    {
        let mut numlen = minlen;
        let mut i = 0;
        while i < cs.len() {
            if cs[i].is_ascii_digit() {
                first_digit = Some(cs[i].to_digit(10).unwrap() as usize);
                break;
            }
            // find spelled digit
            if i + numlen <= s.len() {
                let sep = &s[i..=i + numlen - 1];
                // println!("sep: {sep}");
                if let Some(num) = nums.get(sep) {
                    first_digit = Some(*num);
                    break;
                }
            }
            if numlen == maxlen {
                numlen = minlen;
                i += 1;
            } else {
                numlen += 1;
            }
        }
    }
    first_digit
}

fn find_first_and_last_digits(s: String) -> (usize, usize) {
    let nums = HashMap::from([
        ("one".to_string(), 1),
        ("two".to_string(), 2),
        ("three".to_string(), 3),
        ("four".to_string(), 4),
        ("five".to_string(), 5),
        ("six".to_string(), 6),
        ("seven".to_string(), 7),
        ("eight".to_string(), 8),
        ("nine".to_string(), 9),
    ]);

    let minlen = 3;
    let maxlen = 5;

    let first_digit = find_first_digit(&s, &nums, minlen, maxlen);

    let s_rev = s.chars().rev().collect();
    let mut nums_rev = HashMap::new();
    for (num_str, num) in nums {
        let num_str = num_str.chars().rev().collect();
        nums_rev.insert(num_str, num);
    }

    let last_digit = find_first_digit(&s_rev, &nums_rev, minlen, maxlen);

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => (first, last),
        _ => {
            panic!("first or last number does not found!");
        }
    }
}

pub fn solve1(file: File) {
    println!("[day01 part1]");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let mut trimed: Vec<char> = line.unwrap().chars().filter(|c| c.is_numeric()).collect();
        let first_digit = trimed[0].to_digit(10).unwrap();
        trimed.reverse();
        let last_digit = trimed[0].to_digit(10).unwrap();
        sum += 10 * first_digit + last_digit
    }
    println!("  {sum}");
}

pub fn solve2(file: File) {
    println!("[day01 part2]");
    let reader = BufReader::new(file);
    let mut sum = 0;
    for line in reader.lines() {
        let line = line.unwrap();
        let (first_digit, last_digit) = find_first_and_last_digits(line);
        sum += 10 * first_digit + last_digit;
    }
    println!("  {sum}");
}
