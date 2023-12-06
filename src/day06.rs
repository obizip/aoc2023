use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve1(file: File) {
    println!("[day06 part1]");

    let reader = BufReader::new(file);
    let mut ls = reader.lines();
    let times: Vec<usize> = ls.next().unwrap().unwrap()[11..]
        .split(' ')
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().unwrap())
        .collect();
    let distances: Vec<usize> = ls.next().unwrap().unwrap()[11..]
        .split(' ')
        .filter(|item| !item.is_empty())
        .map(|item| item.parse().unwrap())
        .collect();
    let ans: usize = times
        .into_iter()
        .zip(distances.iter())
        .map(|(t, d)| {
            let s = (1..t).map(|ti| ti * (t - ti)).filter(|di| di > d).count();
            println!("{s}");
            s
        })
        .product();
    println!("{ans:?}");
}

pub fn solve2(file: File) {
    println!("[day06 part2]");

    let reader = BufReader::new(file);
    let mut ls = reader.lines();
    let time: usize = ls.next().unwrap().unwrap()[11..]
        .replace(' ', "")
        .parse()
        .unwrap();
    let distance: usize = ls.next().unwrap().unwrap()[11..]
        .replace(' ', "")
        .parse()
        .unwrap();
    println!("{time}");
    println!("{distance}");
    let ans = (1..time)
        .map(|ti| ti * (time - ti))
        .filter(|di| di > &distance)
        .count();
    println!("{ans:?}");
}
