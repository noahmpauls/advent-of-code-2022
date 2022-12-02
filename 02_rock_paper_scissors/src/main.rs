use clap::Parser;
use std::fs;
use rock_paper_scissors::{Shape, game_score};
use rock_paper_scissors::parse::parse_letter_pair;

/// Solver for Part 1.
fn part_one(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let game_scores = contents.lines()
        .filter_map(|line| parse_letter_pair(line).ok())
        .map(|(opponent, me)| {
            let opponent = match opponent {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => panic!("could not parse line")
            };
            let me = match me {
                'X' => Shape::Rock,
                'Y' => Shape::Paper,
                'Z' => Shape::Scissors,
                _ => panic!("could not parse line")
            };
            game_score(&me, &opponent)
        });
    println!("{}", game_scores.sum::<i32>());
}

/// Solver for Part 2.
fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let game_scores = contents.lines()
        .filter_map(|line| parse_letter_pair(line).ok())
        .map(|(opponent, me)| {
            let opponent = match opponent {
                'A' => Shape::Rock,
                'B' => Shape::Paper,
                'C' => Shape::Scissors,
                _ => panic!("could not parse line")
            };
            let me = match me {
                'X' => opponent.beats(),
                'Y' => opponent,
                'Z' => opponent.beat_by(),
                _ => panic!("could not parse line")
            };
            game_score(&me, &opponent)
        });
    println!("{}", game_scores.sum::<i32>());
}

////////////////////////////////////////////////////////////////////////////////

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_parser = validate_part)]
    part: u8,

    #[arg(short, long)]
    file: String,
}

fn validate_part(s: &str) -> Result<u8, String> {
    let part: u8 = s
        .parse()
        .map_err(|_| "day must be either 1 or 2")?;
    
    if (1..=2).contains(&part) {
        Ok(part)
    } else {
        Err(String::from("day must be either 1 or 2"))
    }
}

fn main() {
    let cli = Cli::parse();
    let part = cli.part;
    let file = &cli.file;
    run(part, file);
}

fn run(part: u8, file: &str) {
    match part {
        1 => part_one(file),
        2 => part_two(file),
        _ => (),
    }
}

