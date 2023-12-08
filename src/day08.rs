use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::iter;

pub fn solve1(file: File) {
    println!("[day08 part1]");
    let mut maps: HashMap<String, (String, String)> = HashMap::new();
    let reader = BufReader::new(file);
    let mut inputs = reader.lines();
    let insts = inputs.next().unwrap().unwrap();
    inputs.next();
    for node in inputs {
        let node = node.unwrap();
        let (name, dest) = node.split_once(" = ").unwrap();
        println!("{name}, {dest}");
        let (left, right) = dest[1..dest.len() - 1].split_once(", ").unwrap();
        maps.insert(name.to_string(), (left.to_string(), right.to_string()));
    }

    let mut currnode = "AAA".to_string();
    let mut nstep = 0;
    'instloop: for insts in iter::repeat_with(|| insts.chars()) {
        for inst in insts {
            println!("{nstep}");
            println!("{:?}", maps.get(&currnode).unwrap());
            println!("{inst}");
            println!("before: {currnode}");
            match inst {
                'L' => currnode = maps.get(&currnode).unwrap().0.to_owned(),
                'R' => currnode = maps.get(&currnode).unwrap().1.to_owned(),
                _ => {}
            }
            println!("after: {currnode}");
            nstep += 1;
            if currnode == "ZZZ" {
                break 'instloop;
            }
        }
    }
    println!("{nstep}");
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

pub fn solve2(file: File) {
    println!("[day08 part2]");
    let mut maps: HashMap<String, (String, String)> = HashMap::new();
    let reader = BufReader::new(file);
    let mut inputs = reader.lines();
    let insts = inputs.next().unwrap().unwrap();
    inputs.next();
    for node in inputs {
        let node = node.unwrap();
        let (name, dest) = node.split_once(" = ").unwrap();
        let (left, right) = dest[1..dest.len() - 1].split_once(", ").unwrap();
        maps.insert(name.to_string(), (left.to_string(), right.to_string()));
    }

    let mut currnodes: Vec<String> = maps
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect();

    let mut counts: HashMap<String, usize> = HashMap::new();
    let mut nstep = 0;
    'instloop: for insts in iter::repeat_with(|| insts.chars()) {
        for inst in insts {
            match inst {
                'L' => {
                    currnodes = currnodes
                        .iter()
                        .map(|node| maps.get(node).unwrap().0.to_owned())
                        .collect()
                }
                'R' => {
                    currnodes = currnodes
                        .iter()
                        .map(|node| maps.get(node).unwrap().1.to_owned())
                        .collect()
                }
                _ => {}
            }
            nstep += 1;
            for node in currnodes.iter() {
                if node.ends_with('Z') {
                    counts.entry(node.clone()).or_insert(nstep);
                }
            }
            if counts.keys().count() == currnodes.len() {
                break 'instloop;
            }
        }
    }
    println!(
        "{}",
        lcm(&counts.values_mut().map(|n| *n).collect::<Vec<usize>>())
    );
}
