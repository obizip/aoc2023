use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve1(file: File) {
    println!("[day03 part1]");
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    let types = vec![
        "2", "3", "4", "5", "6", "7", "8", "9", "T", "J", "Q", "K", "A",
    ];
    let mut games = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(' ').unwrap();
        let mut types_counts: Vec<usize> = types.iter().map(|c| hand.matches(c).count()).collect();
        types_counts.sort();
        println!("{:?}", types_counts);
        let max = types_counts.pop().unwrap();
        let next_max = types_counts.pop().unwrap();
        println!("{}, {}", max, next_max);
        let strength = match (max, next_max) {
            (5, 0) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        };
        println!("{hand}");
        let hand: Vec<usize> = hand
            .chars()
            .map(|c| {
                types
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .find(c)
                    .unwrap()
            })
            .collect();
        let bid: u32 = bid.parse().unwrap();
        games.push((strength, hand, bid));
    }

    for i in (0..5).rev() {
        games.sort_by(|a, b| a.1[i as usize].cmp(&b.1[i as usize]));
    }
    games.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("{:?}", games);
    let mut sum = 0;
    for (i, (strength, hand, bid)) in games.into_iter().enumerate() {
        println!("{:?}, {:?}, {:?}", strength, hand, bid);
        sum += bid * (i as u32 + 1);
    }
    println!("{sum}");
}

pub fn solve2(file: File) {
    println!("[day07 part2]");
    // A, K, Q, J, T, 9, 8, 7, 6, 5, 4, 3, or 2
    let types = vec![
        "J", "2", "3", "4", "5", "6", "7", "8", "9", "T", "Q", "K", "A",
    ];
    let mut games = Vec::new();
    let reader = BufReader::new(file);
    for line in reader.lines() {
        let line = line.unwrap();
        let (hand, bid) = line.split_once(' ').unwrap();
        let mut types_counts: Vec<usize> =
            types[1..].iter().map(|c| hand.matches(c).count()).collect();
        let nj = hand.matches('J').count();

        types_counts.sort();
        println!("{:?}", types_counts);
        let max = types_counts.pop().unwrap() + nj;
        let next_max = types_counts.pop().unwrap();
        println!("{}, {}", max, next_max);
        let strength = match (max, next_max) {
            (5, 0) => 6,
            (4, _) => 5,
            (3, 2) => 4,
            (3, _) => 3,
            (2, 2) => 2,
            (2, _) => 1,
            _ => 0,
        };
        println!("{hand}");
        let hand: Vec<usize> = hand
            .chars()
            .map(|c| {
                types
                    .clone()
                    .into_iter()
                    .collect::<String>()
                    .find(c)
                    .unwrap()
            })
            .collect();
        let bid: u32 = bid.parse().unwrap();
        games.push((strength, hand, bid));
    }

    for i in (0..5).rev() {
        games.sort_by(|a, b| a.1[i as usize].cmp(&b.1[i as usize]));
    }
    games.sort_by(|a, b| a.0.cmp(&b.0));

    // println!("{:?}", games);
    let mut sum = 0;
    for (i, (strength, hand, bid)) in games.into_iter().enumerate() {
        println!("{:?}, {:?}, {:?}", strength, hand, bid);
        sum += bid * (i as u32 + 1);
    }
    println!("{sum}");
}
