use std::fs;


// Modified version of LeTonVonLaser's solution and Didibear's solution

fn main() {
    let input = fs::read_to_string("input.txt")
        .expect("Something went wrong reading the file");

    let slopes = [(3, 1)];
    let slopes2 = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    let result: usize = solve(&slopes, &input);
    let result2: usize = solve(&slopes2, &input);

    println!("{}", result);
    println!("{}", result2);
}

fn solve(slopes: &[(usize, usize)], input: &str) -> usize {
    slopes
        .iter()
        .map(|(right, down)| path(&input, *right, *down))
        .product()
}

fn path(contents: &str, right: usize, down: usize) -> usize {
    contents.lines()
        .step_by(down)
        .enumerate()
        .filter_map(|(index, line)| line.chars().nth(index * right % line.len()))
        .filter(|letter| *letter == '#') // Would've preferd to do this filter step in the previous filter_map, but not sure how to do it
        .count()
}

