use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub fn solve1(file: File) {
    println!("[day05 part1]");
    let reader = BufReader::new(file);

    let mut ls = reader.lines();
    let seed_str = ls.next().unwrap().unwrap();
    let seeds: Vec<usize> = seed_str[7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut current_nums = seeds;

    let mut seed_type = String::new();
    let mut seed_map: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    for line in ls {
        let line = line.unwrap();
        match line {
            l if l.is_empty() => continue,
            l if l.contains("map") => {
                if !seed_type.is_empty() {
                    current_nums = current_nums
                        .iter()
                        .map(|n| {
                            for (dest, source, rangelen) in seed_map[&seed_type].clone().into_iter()
                            {
                                if source <= *n && *n < source + rangelen {
                                    return dest + (*n - source);
                                }
                            }
                            *n
                        })
                        .collect();
                }
                seed_type = String::from(l.split(' ').next().unwrap());
            }
            l => {
                let nums: Vec<usize> = l.split(' ').filter_map(|c| c.parse().ok()).collect();
                let (dest, source, rangelen) = (nums[0], nums[1], nums[2]);

                seed_map
                    .entry(seed_type.clone())
                    .and_modify(|v| {
                        v.push((dest, source, rangelen));
                    })
                    .or_insert(vec![(dest, source, rangelen)]);
            }
        }
    }

    current_nums = current_nums
        .iter()
        .map(|n| {
            for (dest, source, rangelen) in seed_map[&seed_type].clone().into_iter() {
                if source <= *n && *n < source + rangelen {
                    return dest + (*n - source);
                }
            }
            *n
        })
        .collect();
    println!("{:?}", current_nums.iter().min());
}

fn to_len(begin: usize, end: usize) -> usize {
    end - begin + 1
}
fn to_pair(begin: usize, end: usize) -> (usize, usize) {
    (begin, to_len(begin, end))
}

pub fn solve2(file: File) {
    println!("[day05 part1]");
    let reader = BufReader::new(file);

    let mut ls = reader.lines();
    let seed_str = ls.next().unwrap().unwrap();
    let seeds: Vec<usize> = seed_str[7..]
        .split(' ')
        .map(|s| s.parse().unwrap())
        .collect();
    let mut newseeds: Vec<(usize, usize)> = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        newseeds.push((seeds[i], seeds[i + 1]));
    }
    let seeds = newseeds;
    let mut curnums: Vec<(usize, usize)> = seeds;
    curnums.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    let mut seed_type = String::new();
    let mut seed_map: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    for line in ls {
        let line = line.unwrap();
        match line {
            l if l.is_empty() => continue,
            l if l.contains("map") => {
                if !seed_type.is_empty() {
                    let mut nextnums = Vec::new();
                    for (n, nlen) in curnums.iter() {
                        let mut begin = *n;
                        let end = n + nlen - 1;

                        let mut mapvec = seed_map[&seed_type].clone();
                        mapvec.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
                        let mvlen = &mapvec.len();
                        for (i, (dest, source, rangelen)) in mapvec.into_iter().enumerate() {
                            let send = source + rangelen - 1;
                            if begin < source && end < source {
                                nextnums.push(to_pair(begin, end));
                                break;
                            } else if begin < source && end <= send {
                                nextnums.push(to_pair(begin, source - 1));
                                nextnums.push((dest, to_len(source, end)));
                                break;
                            } else if source <= begin && end <= send {
                                let slen = to_len(begin, end);
                                nextnums.push((dest + begin - source, slen));
                                break;
                            } else if source <= begin && begin <= send && send < end {
                                let slen = to_len(begin, send);
                                nextnums.push((dest + begin - source, slen));
                                begin = send + 1;
                            }
                            if i == mvlen - 1 {
                                nextnums.push(to_pair(begin, end));
                            }
                        }
                    }
                    curnums = nextnums;
                }
                seed_type = String::from(l.split(' ').next().unwrap());
            }
            l => {
                let nums: Vec<usize> = l.split(' ').filter_map(|c| c.parse().ok()).collect();
                let (dest, source, rangelen) = (nums[0], nums[1], nums[2]);

                seed_map
                    .entry(seed_type.clone())
                    .and_modify(|v| {
                        v.push((dest, source, rangelen));
                    })
                    .or_insert(vec![(dest, source, rangelen)]);
            }
        }
    }

    let mut nextnums = Vec::new();
    for (n, nlen) in curnums.iter() {
        let mut begin = *n;
        let end = n + nlen - 1;

        let mut mapvec = seed_map[&seed_type].clone();
        mapvec.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        let mvlen = &mapvec.len();
        for (i, (dest, source, rangelen)) in mapvec.into_iter().enumerate() {
            let send = source + rangelen - 1;
            if begin < source && end < source {
                nextnums.push(to_pair(begin, end));
                break;
            } else if begin < source && end <= send {
                nextnums.push(to_pair(begin, source - 1));
                nextnums.push((dest, to_len(source, end)));
                break;
            } else if source <= begin && end <= send {
                let slen = to_len(begin, end);
                nextnums.push((dest + begin - source, slen));
                break;
            } else if source <= begin && begin <= send && send < end {
                let slen = to_len(begin, send);
                nextnums.push((dest + begin - source, slen));
                begin = send + 1;
            }
            if i == mvlen - 1 {
                nextnums.push(to_pair(begin, end));
            }
        }
    }
    curnums = nextnums;
    println!("  {:?}", curnums.iter().map(|(n, _)| n).min());
}
