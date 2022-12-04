use clap::Parser;
use rucksack_reorganization::{rucksack_repeats, priority, shared_chars};
// use regex::Regex;
use std::fs;

/// Solver for Part 1.
fn part_one(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let priority_sum: u32 = contents.lines()
        .flat_map(|line| rucksack_repeats(line))
        .map(|c| priority(&c))
        .sum();

    println!("{}", priority_sum);
}

/// Solver for Part 2.
fn part_two(file: &str) {
    let contents = fs::read_to_string(file).unwrap();
    let priority_sum: u32 = contents.lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .flat_map(|chunk| shared_chars(chunk.to_vec()))
        .map(|c| priority(&c))
        .sum();

    println!("{}", priority_sum);
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

