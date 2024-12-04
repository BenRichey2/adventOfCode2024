use std::env;
use std::fs;
use regex::Regex;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: cargo run -- <path to input file>");
    }
    let filename = args.get(1).unwrap();
    let contents = fs::read_to_string(filename).unwrap();
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)").unwrap();
    let commands: Vec<&str> = re.find_iter(&contents).map(|cmd| cmd.as_str()).collect();
    let first_num_idx = "mul(".len();
    let mut sum = 0;
    for cmd in commands {
        let second_num_idx = (*cmd).find(",").unwrap_or_else(||
            {
                panic!("Failed to find ',' in str: {cmd}");
            }) + 1;
        let first_num: i32 = (*cmd)[first_num_idx..(second_num_idx - 1)].parse()
            .unwrap_or_else(|_| {
                panic!("Failed to parse string: {cmd} from index: {first_num_idx} to {second_num_idx}");
        });
        let end_second_num_idx = (*cmd).len() - 1;
        let second_num: i32 = (*cmd)[second_num_idx..end_second_num_idx].parse()
            .unwrap_or_else(|_| {
                panic!("Failed to parse string: {cmd} from index: {second_num_idx} to {end_second_num_idx}");
        });
        sum += first_num * second_num;
    }
    println!("Part 1: The sum is: {sum}");
    let re = Regex::new(r"mul\([0-9]+,[0-9]+\)|do\(\)|don't\(\)").unwrap();
    let commands: Vec<&str> = re.find_iter(&contents).map(|cmd| cmd.as_str()).collect();
    let mut enabled = true;
    sum = 0;
    for cmd in commands {
        if *cmd == *"do()" {
            enabled = true;
        } else if *cmd == *"don't()" {
            enabled = false;
        } else if enabled {
            let second_num_idx = (*cmd).find(",").unwrap_or_else(||
                {
                    panic!("Failed to find ',' in str: {cmd}");
                }) + 1;
            let first_num: i32 = (*cmd)[first_num_idx..(second_num_idx - 1)].parse()
                .unwrap_or_else(|_| {
                    panic!("Failed to parse string: {cmd} from index: {first_num_idx} to {second_num_idx}");
            });
            let end_second_num_idx = (*cmd).len() - 1;
            let second_num: i32 = (*cmd)[second_num_idx..end_second_num_idx].parse()
                .unwrap_or_else(|_| {
                    panic!("Failed to parse string: {cmd} from index: {second_num_idx} to {end_second_num_idx}");
            });
            sum += first_num * second_num;
        }
    }
    println!("Part 2: The sum is: {sum}");
}
