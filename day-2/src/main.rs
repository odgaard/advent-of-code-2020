use std::fs::read_to_string;
use std::env::args;
use std::time::Instant;

use parse_display::{Display as PDisplay, FromStr as PFromStr};

#[derive(PDisplay, PFromStr, Debug)]
#[display("{min}-{max} {letter}: {password}")]
struct PasswordRule {
    min: u32,
    max: u32,
    letter: char,
    password: String,
}

fn prob1sol1(entries: &Vec<PasswordRule>) -> i32 {
    let mut result = 0;
    for item in entries {
        let mut sum = 0;
        for c in item.password.chars() {
            if item.letter == c {
                sum += 1;
            }
        }
        if sum <= item.max && sum >= item.min {
            result += 1;
        }
    }
    result
}

fn prob2sol1(entries: &Vec<PasswordRule>) -> i32 {
    let mut result = 0;
    for item in entries {
        if (item.password.chars().nth(item.min as usize -1).unwrap() == item.letter) ^ 
            (item.password.chars().nth(item.max as usize -1).unwrap() == item.letter) {
            result += 1
        }
    }
    result
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = read_to_string(args().nth(1).ok_or("No input file")?)?;
    let entries: Vec<PasswordRule> = input
        .lines()
        .map(|line| line.parse::<PasswordRule>().expect("Expected an int on each line"))
        .collect();


    let mut total_elapsed = 0;
    let mut result = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob1sol1(&entries);
        total_elapsed += now.elapsed().as_micros();
    }
    println!("{}", total_elapsed/1000);
    println!("{}", result);

    let mut total_elapsed = 0;
    let mut result = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob2sol1(&entries);
        total_elapsed += now.elapsed().as_micros();
    }
    println!("{}", total_elapsed/1000);
    println!("{}", result);

    Ok(())
}
