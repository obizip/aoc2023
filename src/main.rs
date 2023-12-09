extern crate dirs;

use dirs::home_dir;
use std::env;
use std::fs::File;
use std::path::Path;
use std::path::PathBuf;
mod day01;
mod day02;
mod day03;
mod day04;
mod day05;
mod day06;
mod day07;
mod day08;
mod day09;

fn main() -> Result<(), std::io::Error> {
    let mut args = env::args();
    args.next();

    let mut solve_all = false;
    if let Some(arg) = args.next() {
        match arg.as_str() {
            "all" => solve_all = true,
            _ => {}
        }
    }

    let homedir = home_dir().unwrap();
    let inputdir: PathBuf = [homedir, Path::new("Documents/aoc2023/inputs").to_path_buf()]
        .iter()
        .collect();
    let input = |filename: &str| File::open(inputdir.join(filename)).unwrap();

    let days = vec![
        (day01::solve1 as fn(File), "day1.ex"),
        (day01::solve1 as fn(File), "day1"),
        (day01::solve2 as fn(File), "day1.2.ex"),
        (day01::solve2 as fn(File), "day1.2"),
        (day02::solve1 as fn(File), "day2.ex"),
        (day02::solve1 as fn(File), "day2"),
        (day02::solve2 as fn(File), "day2.ex"),
        (day02::solve2 as fn(File), "day2"),
        (day03::solve1 as fn(File), "day3.ex"),
        (day03::solve1 as fn(File), "day3"),
        (day03::solve2 as fn(File), "day3.ex"),
        (day03::solve2 as fn(File), "day3"),
        (day04::solve1 as fn(File), "day4.ex"),
        (day04::solve1 as fn(File), "day4"),
        (day04::solve2 as fn(File), "day4.ex"),
        (day04::solve2 as fn(File), "day4"),
        (day05::solve1 as fn(File), "day5.ex"),
        (day05::solve1 as fn(File), "day5"),
        (day05::solve2 as fn(File), "day5.ex"),
        (day05::solve2 as fn(File), "day5"),
        (day06::solve1 as fn(File), "day6.ex"),
        (day06::solve1 as fn(File), "day6"),
        (day06::solve2 as fn(File), "day6.ex"),
        (day06::solve2 as fn(File), "day6"),
        (day07::solve1 as fn(File), "day7.ex"),
        (day07::solve1 as fn(File), "day7"),
        (day07::solve2 as fn(File), "day7.ex"),
        (day07::solve2 as fn(File), "day7"),
        (day08::solve1 as fn(File), "day8.ex"),
        (day08::solve1 as fn(File), "day8"),
        (day08::solve2 as fn(File), "day8.2.ex"),
        (day08::solve2 as fn(File), "day8"),
        (day09::solve1 as fn(File), "day9.ex"),
        (day09::solve1 as fn(File), "day9"),
        (day09::solve2 as fn(File), "day9.ex"),
        (day09::solve2 as fn(File), "day9"),
    ];

    if solve_all {
        for (solve, filename) in days {
            solve(input(filename));
            println!("  (input: {filename})");
            println!()
        }
    } else if let Some((solve, filename)) = days.last() {
        solve(input(filename));
        println!("  (input: {filename})");
    }

    Ok(())
}
