use std::fs::read_to_string;
use std::env::args;
use std::time::Instant;
use std::fs;


// Modified version of LeTonVonLaser's solution and Didibear's solution

fn main() {
    let filename = "input.txt";
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let res1 = path(&contents, 3, 1);
    let res2 = path(&contents, 1, 1);
    let res3 = path(&contents, 5, 1);
    let res4 = path(&contents, 7, 1);
    let res5 = path(&contents, 1, 2);
    println!("First: {}", res2);
    println!("Second: {}", res1*res2*res3*res4*res5);
}

fn path(contents: &str, right: usize, down: usize) -> usize {
    contents.lines()
        .step_by(down)
        .enumerate()
        .filter_map(|(index, line)| line.chars().nth(index * right % line.len()))
        .filter(|letter| *letter == '#') // Would've preferd to do this filter step in the previous filter_map, but not sure how to do it
        .count()
}

