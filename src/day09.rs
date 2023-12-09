use std::fs::File;
use std::io::{BufRead, BufReader};

fn extrapolate_backward(nums: Vec<isize>) -> isize {
    nums[nums.len() - 1] + extrapolate_backward_inner(nums, Vec::new())
}

fn extrapolate_backward_inner(nums: Vec<isize>, mut lasts: Vec<isize>) -> isize {
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }

    let last = next[next.len() - 1];
    lasts.push(last);

    if next.iter().all(|n| *n == 0) {
        0
    } else {
        last + extrapolate_backward_inner(next, lasts)
    }
}

pub fn solve1(file: File) {
    println!("[day09 part1]");
    let mut sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        println!("{:?}", line);
        let nums: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        sum += extrapolate_backward(nums);
    }
    println!("{sum}");
}

fn extrapolate_forward(nums: Vec<isize>) -> isize {
    nums[0] - extrapolate_forward_inner(nums, Vec::new())
}

fn extrapolate_forward_inner(nums: Vec<isize>, mut firsts: Vec<isize>) -> isize {
    let mut next = Vec::new();
    for i in 1..nums.len() {
        next.push(nums[i] - nums[i - 1]);
    }

    let first = next[0];
    firsts.push(first);

    if next.iter().all(|n| *n == 0) {
        0
    } else {
        first - extrapolate_forward_inner(next, firsts)
    }
}

pub fn solve2(file: File) {
    println!("[day09 part2]");
    let mut sum = 0;
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        sum += extrapolate_forward(nums);
    }
    println!("{sum}");
}
