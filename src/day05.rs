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
    println!("{seeds:?}");
    let mut current_nums = seeds;

    // let mut sum = 0;
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
                    println!("{current_nums:?}");
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
    println!("{current_nums:?}");
    println!("{:?}", current_nums.iter().min());
    // println!("  {sum}");
}

fn to_n_len(begin: usize, end: usize) -> (usize, usize) {
    (begin, end - begin + 1)
}

struct RangeSet {
    start: usize,
    range: usize,
}

impl RangeSet {
    fn new(start: usize, range: usize) -> Self {
        Self { start, range }
    }
    fn from_start_end(start: usize, end: usize) -> Self {
        Self {
            start,
            range: end - start + 1,
        }
    }
    fn get_end(&self) -> usize {
        self.start + self.range - 1
    }

    fn intersect(&self, set: Self) -> Vec<Self> {
        let mut in_sets = Vec::new();
        //
        let a_begin = self.start;
        let a_end = self.start + self.range - 1;

        let b_begin = set.start;
        let b_end = set.start + set.range - 1;

        if b_begin <= a_begin && a_end <= b_end {
            in_sets.push(RangeSet::from_start_end(a_begin, a_end));
        } else if b_begin <= a_begin && a_begin <= b_end && a_end > b_end {
            in_sets.push(RangeSet::from_start_end(a_begin, b_end));
        } else if a_begin <= b_begin && b_end <= a_end {
            in_sets.push(RangeSet::from_start_end(b_begin, b_end));
        } else if b_begin <= a_end && a_end <= b_begin {
            in_sets.push(RangeSet::from_start_end(b_begin, a_end));
        }
        in_sets
    }

    fn is_subset(&self, subset: &Self) -> bool {
        let end = self.get_end();
        let subend = subset.get_end();

        self.start <= subset.start && subend <= end
    }

    fn substract_by_subset(&self, subset: &Self) -> Option<Vec<RangeSet>> {
        if self.is_subset(subset) {
            let mut outs = Vec::new();
            let end = self.get_end();
            let subend = subset.get_end();

            outs.push(RangeSet::from_start_end(self.start, subset.start - 1));
            outs.push(RangeSet::from_start_end(subend + 1, end));

            return Some(outs);
        }

        None
    }

    // fn in_or_out(&self, set: Self) -> (Vec<Self>, Vec<Self>) {
    //     let mut in_sets = Vec::new();
    //     let mut out_sets = Vec::new();
    //     //
    //     let a_begin = self.start;
    //     let a_end = self.start + self.range - 1;
    //
    //     let b_begin = set.start;
    //     let b_end = set.start + set.range - 1;
    //
    //     if b_begin <= a_begin && a_end <= b_end {
    //         in_sets.push(RangeSet::from_start_end(a_begin, a_end));
    //     } else if b_begin <= a_begin && a_begin <= b_end && a_end > b_end {
    //         in_sets.push(RangeSet::from_start_end(a_begin, b_end));
    //         out_sets.push(RangeSet::from_start_end(b_end + 1, a_end));
    //     } else if a_begin <= b_begin && b_end <= a_end {
    //         out_sets.push(RangeSet::from_start_end(a_begin, b_begin - 1));
    //         in_sets.push(RangeSet::from_start_end(b_begin, b_end));
    //         out_sets.push(RangeSet::from_start_end(b_end + 1, a_end));
    //     } else if b_begin <= a_end && a_end <= b_begin {
    //         out_sets.push(RangeSet::from_start_end(a_begin, b_begin - 1));
    //         in_sets.push(RangeSet::from_start_end(b_begin, a_end));
    //     } else {
    //         out_sets.push(RangeSet::from_start_end(a_begin, a_end));
    //     }
    //
    //     (in_sets, out_sets)
    // }
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
    println!("{seeds:?}");
    let mut curnums: Vec<(usize, usize)> = seeds;
    curnums.sort_by(|x, y| x.0.partial_cmp(&y.0).unwrap());

    // let mut sum = 0;
    let mut seed_type = String::new();
    let mut seed_map: HashMap<String, Vec<(usize, usize, usize)>> = HashMap::new();
    for line in ls {
        let line = line.unwrap();
        match line {
            l if l.is_empty() => continue,
            l if l.contains("map") => {
                if !seed_type.is_empty() {
                    let mut nextnums = Vec::new();
                    println!("    before: curnum: {curnums:?}");
                    for (n, nlen) in curnums.iter() {
                        let mut begin = *n;
                        let end = n + nlen - 1;

                        let mut mapvec = seed_map[&seed_type].clone();
                        mapvec.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
                        let mvlen = &mapvec.len();
                        // println!("map: {mapvec:?}");
                        for (i, (dest, source, rangelen)) in mapvec.into_iter().enumerate() {
                            println!("   {}, {}", begin, end);
                            println!("   {}, {}, {}", dest, source, rangelen);
                            // println!("next: {nextnums:?}");
                            let send = source + rangelen - 1;
                            if begin < source && end < source {
                                println!("    too small");
                                nextnums.push(to_pair(begin, end));
                                break;
                            } else if begin < source && end <= send {
                                println!("    in end");
                                nextnums.push(to_pair(begin, source - 1));
                                nextnums.push((dest, to_len(source, end)));
                                break;
                            } else if source <= begin && end <= send {
                                println!("    inside");
                                let slen = to_len(begin, end);
                                nextnums.push((dest + begin - source, slen));
                                break;
                            } else if source <= begin && begin <= send && send < end {
                                println!("    in begin");
                                let slen = to_len(begin, send);
                                nextnums.push((dest + begin - source, slen));
                                begin = send + 1;
                            } else {
                                println!("    too big");
                            }
                            if i == mvlen - 1 {
                                nextnums.push(to_pair(begin, end));
                            }
                        }
                    }
                    curnums = nextnums;
                    println!("    after: curnum: {curnums:?}");
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
    println!("    before: curnum: {curnums:?}");
    for (n, nlen) in curnums.iter() {
        let mut begin = *n;
        let end = n + nlen - 1;

        let mut mapvec = seed_map[&seed_type].clone();
        mapvec.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap());
        let mvlen = &mapvec.len();
        // println!("map: {mapvec:?}");
        for (i, (dest, source, rangelen)) in mapvec.into_iter().enumerate() {
            println!("   {}, {}", begin, end);
            println!("   {}, {}, {}", dest, source, rangelen);
            // println!("next: {nextnums:?}");
            let send = source + rangelen - 1;
            if begin < source && end < source {
                println!("    too small");
                nextnums.push(to_pair(begin, end));
                break;
            } else if begin < source && end <= send {
                println!("    in end");
                nextnums.push(to_pair(begin, source - 1));
                nextnums.push((dest, to_len(source, end)));
                break;
            } else if source <= begin && end <= send {
                println!("    inside");
                let slen = to_len(begin, end);
                nextnums.push((dest + begin - source, slen));
                break;
            } else if source <= begin && begin <= send && send < end {
                println!("    in begin");
                let slen = to_len(begin, send);
                nextnums.push((dest + begin - source, slen));
                begin = send + 1;
            } else {
                println!("    too big");
            }
            if i == mvlen - 1 {
                nextnums.push(to_pair(begin, end));
            }
        }
    }
    curnums = nextnums;
    println!("    after: curnum: {curnums:?}");
    println!("  {:?}", curnums.iter().map(|(n, _)| n).min());

    // current_nums = current_nums
    //     .iter()
    //     .map(|n| {
    //         for (dest, source, rangelen) in seed_map[&seed_type].clone().into_iter() {
    //             if source <= *n && *n < source + rangelen {
    //                 return dest + (*n - source);
    //             }
    //         }
    //         *n
    //     })
    //     .collect();
    // println!("{current_nums:?}");
    // println!("{:?}", current_nums.iter().min());
    // println!("  {sum}");
}
