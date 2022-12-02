use clap::Parser;
// use regex::Regex;
use std::{fs, cmp::max, collections::BinaryHeap};

/// Solver for Part 1.
fn part_one(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let calories = contents.lines().map(|line| {
        match line.parse::<u32>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    });

    let mut current = 0;
    let mut most = 0;
    for value in calories {
        if let Some(count) = value {
            current += count;
        } else {
            most = max(current, most);
            current = 0;
        }
    }

    println!("{most}");
}

/// Solver for Part 2.
fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let calories = contents.lines().map(|line| {
        match line.parse::<u32>() {
            Ok(value) => Some(value),
            Err(_) => None,
        }
    });
    
    let mut elves = BinaryHeap::new();
    let mut current = 0;
    for value in calories {
        if let Some(count) = value {
            current += count;
        } else {
            elves.push(current);
            current = 0;
        }
    }

    let total: u32 = (0..3).map(|_| elves.pop().unwrap_or(0)).sum();
    println!("{total}");
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

