use std::env;
use std::fs;

const PROBLEM_THRESHOLD: i32 = 1; // For part 1, set to 0. For part two, set to 1.

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <path to input file>");
    }
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut data: Vec<Vec<i32>> = Vec::new();
    let size = contents.lines().count();
    data.reserve(size);
    for line in contents.lines() {
        let parts = line.split_whitespace();
        let mut numbers: Vec<i32> = Vec::new();
        numbers.reserve(parts.clone().count());
        for part in parts {
            numbers.push(part.parse().unwrap());
        }
        data.push(numbers);
    }
    let mut safe_indices: Vec<bool> = Vec::new();
    safe_indices.reserve(size);
    for line in &data {
        let mut num_probs = 0;
        let mut safe = true;
        let mut iter = line.iter();
        let mut lt = false;
        let mut gt = false;
        let mut curr = iter.next();
        let mut next = iter.next();
        while next.is_none() == false {
            let diff = curr.unwrap() - next.unwrap();
            if diff < 0 {
                lt = true;
            } else if diff > 0 {
                gt = true;
            }
            if diff.abs() == 0 || diff.abs() > 3 {
                num_probs += 1;
            } else if gt == true && lt == true {
                num_probs += 1;
            }
            curr = next;
            next = iter.next();
        }
        if num_probs > PROBLEM_THRESHOLD {
            safe = false;
        }
        safe_indices.push(safe);
    }
    for iter in data.iter().zip(safe_indices.iter()) {
        let (line_ref, safe_ref) = iter;
        if *safe_ref == false {
            println!("{:?} is safe: {}", *line_ref, *safe_ref);
        }
    }
    println!("Number of safe reports: {}", safe_indices.iter().filter(|&x| *x == true).count());
}
