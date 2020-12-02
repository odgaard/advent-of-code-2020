use std::fs::read_to_string;
use std::time::Instant;
use std::collections::HashSet;
use std::iter::FromIterator;

use clap::Clap;

#[derive(Clap)]
struct Opts {
    input: String,
}

fn prob1sol1(entries: &Vec<i32>, sum: i32) -> i32 {
    let mut result = 0;
    for i_item in entries {
        for j_item in entries {
            if i_item + j_item == sum {
                result = i_item * j_item;
                return result;
            }
        }
    }
    result 
}

fn prob1sol2(entries: &HashSet<i32>, sum: i32) -> i32 {
    let mut result = 0;
    for i_item in entries {
        let remainder = sum - i_item;
        if entries.contains(&remainder) {
            result = i_item * remainder;
            return result;
        }
    }
    result
}

fn prob2sol1(entries: &Vec<i32>, sum: i32) -> i32 {
    let mut result = 0;
    for i_item in entries {
        for j_item in entries {
            for k_item in entries {
                if i_item + j_item + k_item == sum {
                    result = i_item * j_item * k_item;
                    return result;
                }
            }
        }
    }
    result 
}

fn prob2sol2(entries: &HashSet<i32>, sum: i32) -> i32 {
    let mut result = 0;
    for i_item in entries {
        for j_item in entries {
            let remainder = sum - i_item - j_item;
            if entries.contains(&remainder) {
                result = i_item * j_item * remainder;
                return result;
            }
        }
    }
    result 
}

fn main() -> Result<(), std::io::Error> {
    let opts: Opts = Opts::parse();
    let input = read_to_string(opts.input)?;
    let entries: Vec<i32> = input
        .lines()
        .map(|line| line.parse::<i32>().expect("Expected an int on each line"))
        .collect();


    let sum = 2020;

    let mut result = 0;
    let mut total_elapsed = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob1sol1(&entries, sum);
        total_elapsed += now.elapsed().as_micros();
    }
    println!("{}", total_elapsed/1000);
    println!("{}", result);

    let hash_entries = HashSet::<i32>::from_iter(entries.iter().cloned());

    result = 0;
    total_elapsed = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob1sol2(&hash_entries, sum);
        total_elapsed += now.elapsed().as_micros();
    }

    println!("{}", total_elapsed/1000);
    println!("{}", result);

    result = 0;
    total_elapsed = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob2sol1(&entries, sum);
        total_elapsed += now.elapsed().as_micros();
    }

    println!("{}", total_elapsed/1000);
    println!("{}", result);

    result = 0;
    total_elapsed = 0;

    for _ in 0..1000 {
        let now = Instant::now();
        result = prob2sol2(&hash_entries, sum);
        total_elapsed += now.elapsed().as_micros();
    }

    println!("{}", total_elapsed/1000);
    println!("{}", result);

    Ok(())
}
