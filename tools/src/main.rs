use clap::{Parser, Subcommand};
use toml_edit::{Document, value, Value};
use std::{fs, cmp::Ordering};

#[derive(Parser, Debug)]
#[command(name = "AoC Tools")]
#[command(version, author, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    AddMember {
        name: String,
    }
}

fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::AddMember { name } => add_member(name)
    }
}

fn add_member(s: &str) {
    let toml = fs::read_to_string("./Cargo.toml").expect("can't read ./Cargo.toml");
    let mut doc = toml.parse::<Document>().expect("invalid doc");
    add_to_members(&mut doc, s);
    match fs::write("./Cargo.toml", doc.to_string()) {
        Ok(()) => (),
        Err(_) => println!("can't write to ./Cargo.toml"),
    }
}

fn add_to_members(doc: &mut Document, s: &str) {
    let members = doc["workspace"]["members"].as_array_mut().expect("can't get members array");
    let mut members = members.iter().map(|v| v.as_str().unwrap()).collect::<Vec<&str>>();
    if members.contains(&s) {
        return
    }
    members.push(s);
    sort_numbers_first(&mut members);

    let mut arr = toml_edit::Array::new();
    arr.set_trailing_comma(true);
    arr.set_trailing("\n");
    for item in members {
        let quoted = format!("\"{}\"", item);
        arr.push_formatted(quoted.parse::<Value>().unwrap()
            .decorated("\n\t", "")
        )
    } 

    doc["workspace"]["members"] = value(arr);
}

fn sort_numbers_first(vec: &mut Vec<&str>) {
    vec.sort_by(|a , b| {
        let a_start = a.chars().next().unwrap();
        let b_start = b.chars().next().unwrap();

        if a_start.is_numeric() != b_start.is_numeric() {
            if a_start.is_numeric() {
                Ordering::Greater
            } else {
                Ordering::Less
            }
        } else {
            a.cmp(&b)
        }
    });
}

