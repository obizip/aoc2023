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

pub fn find_first_and_last_digits(s: String) -> (usize, usize) {
    let cs: Vec<char> = s.chars().collect();
    let nums = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);
    let minlen = 3;
    let maxlen = 5;

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

    let mut last_digit = None;
    {
        let mut numlen = minlen;
        let mut i: isize = (cs.len() - 1) as isize;
        while i >= 0 {
            if cs[i as usize].is_ascii_digit() {
                last_digit = Some(cs[i as usize].to_digit(10).unwrap() as usize);
                break;
            }
            // find spelled digit
            if i - numlen as isize + 1 >= 0 {
                let sep = &s[(i - numlen as isize + 1) as usize..=i as usize];
                // println!("sep: {sep}");
                if let Some(num) = nums.get(sep) {
                    last_digit = Some(*num);
                    break;
                }
            }
            if numlen == maxlen {
                numlen = minlen;
                i -= 1;
            } else {
                numlen += 1;
            }
        }
    }

    match (first_digit, last_digit) {
        (Some(first), Some(last)) => (first, last),
        _ => {
            panic!("first or last number does not found!");
        }
    }
}

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
    let (rest, cubes) = separated_list0(char(','), parse_cubes)(i)?;
    Ok((rest, cubes))
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
