use std::cmp::Ordering;
use std::env;
use std::fs;

const PROBLEM_THRESHOLD: i32 = 0; // For part 1, set to 0. For part two, set to 1.

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <path to input file>");
    }
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let mut data: Vec<Vec<i32>> = Vec::with_capacity(contents.lines().count());
    for line in contents.lines() {
        let parts = line.split_whitespace();
        let mut numbers: Vec<i32> = Vec::with_capacity(parts.clone().count());
        for part in parts {
            numbers.push(part.parse().unwrap());
        }
        data.push(numbers);
    }
    let mut safe_indices: Vec<bool> = Vec::with_capacity(contents.lines().count());
    for line in &data {
        let mut num_probs = 0;
        let mut safe = true;
        let mut iter = line.iter();
        let mut lt = false;
        let mut gt = false;
        let mut curr = iter.next();
        let mut next = iter.next();
        while next.is_some() {
            let diff = curr.unwrap() - next.unwrap();
            match diff.cmp(&0) {
                Ordering::Greater => gt = true,
                Ordering::Less => lt = true,
                Ordering::Equal => ()
            }
            if diff.abs() == 0 || diff.abs() > 3 || gt && lt {
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
    println!("Number of safe reports: {}", safe_indices.iter().filter(|&x| *x ).count());
    let mut safe_indices: Vec<bool> = Vec::with_capacity(contents.lines().count());
    for line in &data {
        let mut global_safe = false;
        for leave_out_idx in 0..(*line).len() {
            let mut lt = false;
            let mut gt = false;
            let mut line_clone = line.clone();
            line_clone.remove(leave_out_idx);
            let mut iter = line_clone.iter();
            let mut curr = iter.next();
            let mut next = iter.next();
            let mut safe = true;
            while next.is_some() {
                let diff = curr.unwrap() - next.unwrap();
                curr = next;
                next = iter.next();
                match diff.cmp(&0) {
                    Ordering::Greater => gt = true,
                    Ordering::Less => lt = true,
                    Ordering::Equal => ()
                }
                if diff.abs() == 0 || diff.abs() > 3 || gt && lt {
                    safe = false;
                    next = None;
                }
            }
            if safe {
                global_safe = true;
                break;
            }
        }
        safe_indices.push(global_safe);
    }
    println!("Number of safe reports: {}", safe_indices.iter().filter(|&x| *x ).count());
}
