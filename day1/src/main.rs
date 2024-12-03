use std::{
    env, fs, process
};

fn main() {
    let args: Vec<String> = env::args().collect();
    let num_args = args.len();
    if num_args != 2 {
        println!("Usage: cargo run -- <path to test file>");
        process::exit(-1);
    }
    // All of the code below was ripped from: https://github.com/jayo60013/aoc_2024/blob/main/day01/src/main.rs
    // I wrote up a solution that was wrong, and I'm not sure why, but looking into it.
    // TODO: figure out why my original solution was wrong.
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();
    let size = contents.lines().count();
    left.reserve(size);
    right.reserve(size);

    for line in contents.lines() {
        let mut parts = line.split_whitespace();
        if let (Some(l), Some(r)) = (parts.next(), parts.next()) {
            left.push(l.parse().unwrap());
            right.push(r.parse().unwrap());
        }
    }
    left.sort_unstable();
    right.sort_unstable();
    let diff: i32 = left.iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    println!("The different between list 1 and list 2 is: {diff}");
    // End of ripped code.
    /*
    NOTE: I figured out what my mistake was in my original implementation. I didn't
    realize that each line only had two numbers in it. I thought we were treating each
    line as its own set of two lists, but this was wrong and yielded a wrong solution.
    This new solution I ripped was much better written anyways.
    */
    let mut left_score = 0;
    let mut right_score = 0;
    for number in &left {
        let count = right.iter()
            .filter(|&n| *n == *number)
            .count() as i32;
        left_score += count * number;
    }
    for number in &right {
        let count = left.iter()
            .filter(|&n| *n == *number)
            .count() as i32;
        right_score += count * number;
    }
    println!("The left score is: {left_score} and the right score is: {right_score}");
}
