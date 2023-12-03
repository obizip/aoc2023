use nom::branch::permutation;
use nom::character::complete::{alpha1, multispace0};
use nom::multi::separated_list0;
use nom::sequence::{delimited, preceded};
use nom::IResult;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1},
};
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};

// GAME := "Game" DIGIT: GRABS
// GRABS := GRAB | GRAB; GRAB
// GRAB := CUBES | CUBES, CUBES
// CUBES := DIGIT COLOR
// COLOR := "red" | "blue" | "green"

#[derive(Debug, Hash, PartialEq, Eq)]
pub enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Debug)]
pub struct Cubes {
    count: usize,
    color: Color,
}

impl Cubes {
    pub fn new(count: usize, color: &str) -> Self {
        let color = match color {
            "red" => Color::Red,
            "green" => Color::Green,
            "blue" => Color::Blue,
            _ => panic!("unknown color"),
        };

        Self { count, color }
    }
}

type Grab = Vec<Cubes>;
type Grabs = Vec<Vec<Cubes>>;

#[derive(Debug)]
pub struct Game {
    pub count: usize,
    pub grabs: Grabs,
}

fn parse_cubes(i: &str) -> IResult<&str, Cubes> {
    //let (rest, (count, color)) =
    let (rest, count) = preceded(multispace0, digit1)(i)?;
    let (rest, color) = preceded(multispace0, alpha1)(rest)?;
    let count = count.parse().unwrap();

    let cube = Cubes::new(count, color);

    Ok((rest, cube))
}

fn parse_grab(i: &str) -> IResult<&str, Grab> {
    let (rest, grab) = separated_list0(char(','), parse_cubes)(i)?;
    Ok((rest, grab))
}

pub fn parse_game(i: &str) -> IResult<&str, Game> {
    let (rest, count) = delimited(permutation((tag("Game"), multispace0)), digit1, char(':'))(i)?;
    let (rest, grabs) = separated_list0(char(';'), parse_grab)(rest)?;
    let count = count.parse().unwrap();
    Ok((rest, Game { count, grabs }))
}

pub fn ispossible(game: &Game, true_cubes: &HashMap<Color, usize>) -> bool {
    for grab in &game.grabs {
        for cubes in grab {
            if let Some(max_count) = true_cubes.get(&cubes.color) {
                if max_count < &cubes.count {
                    return false;
                }
            }
        }
    }
    true
}

pub fn get_fewest_nums(game: &Game) -> (usize, usize, usize) {
    let (mut red, mut green, mut blue) = (0, 0, 0);
    for grab in &game.grabs {
        for cubes in grab {
            match cubes.color {
                Color::Red => {
                    if cubes.count > red {
                        red = cubes.count;
                    }
                }
                Color::Green => {
                    if cubes.count > green {
                        green = cubes.count;
                    }
                }
                Color::Blue => {
                    if cubes.count > blue {
                        blue = cubes.count;
                    }
                }
            }
        }
    }

    (red, green, blue)
}

pub fn solve1(file: File) {
    println!("[day02 part1]");
    // day2-1
    let true_cubes: HashMap<Color, usize> =
        HashMap::from([(Color::Red, 12), (Color::Green, 13), (Color::Blue, 14)]);

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
    println!("  {sum}");
}

pub fn solve2(file: File) {
    println!("[day02 part2]");
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
    println!("  {sum}");
}
