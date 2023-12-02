extern crate dirs;

use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
mod libs;
use libs::*;

fn main() -> Result<(), std::io::Error> {
    let homedir = dirs::home_dir().unwrap();
    let inputdir: PathBuf = [homedir, Path::new("Code/aoc2023/inputs").to_path_buf()]
        .iter()
        .collect();
    // day1-1
    if false {
        let file = File::open(inputdir.join("day1")).unwrap();
        // let file = File::open(inputdir.join("day1.ex"))?;
        let reader = BufReader::new(file);
        let mut sum = 0;
        for line in reader.lines() {
            let mut trimed: Vec<char> = line.unwrap().chars().filter(|c| c.is_numeric()).collect();
            let first_digit = trimed[0].to_digit(10).unwrap();
            trimed.reverse();
            let last_digit = trimed[0].to_digit(10).unwrap();
            sum += 10 * first_digit + last_digit
        }
        println!("{sum}");
    }

    // day1-2
    if false {
        // let file = File::open(inputdir.join("day1.2.ex")).unwrap();
        let file = File::open(inputdir.join("day1.2")).unwrap();
        let reader = BufReader::new(file);
        let mut sum = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            // println!("original: {line}");
            let (first_digit, last_digit) = find_first_and_last_digits(line);
            sum += 10 * first_digit + last_digit;
            // println!("first: {first_digit}, last: {last_digit}");
            // println!("{sum}");
            println!();
        }
        println!("{sum}");
    }

    if false {
        // day2-1
        let true_cubes: HashMap<Color, usize> =
            HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

        // let file = File::open(inputdir.join("day2.ex")).unwrap();
        let file = File::open(inputdir.join("day2")).unwrap();
        let reader = BufReader::new(file);
        let mut sum = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let (_, game) = parse_game(line.as_str()).unwrap();
            // println!("{game:?}");
            if ispossible(&game, &true_cubes) {
                sum += game.count;
            }
        }
        println!("{sum}");
    }
    println!("{inputdir:?}");

    if true {
        // day2-2
        // let file = File::open(inputdir.join("day2.ex")).unwrap();
        let file = File::open(inputdir.join("day2")).unwrap();
        let reader = BufReader::new(file);
        let mut sum = 0;
        for line in reader.lines() {
            let line = line.unwrap();
            let (_, game) = parse_game(line.as_str()).unwrap();
            // println!("{game:?}");
            let (red, green, blue) = get_fewest_nums(&game);
            // println!("{red}, {green}, {blue}");
            let power = red * green * blue;
            // println!("{power}");
            sum += power;
        }
        println!("{sum}");
    }

    Ok(())
}
