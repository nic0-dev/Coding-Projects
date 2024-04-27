use std::io::*;
use std::time::{Instant};

fn find_min_combi(num_dials: u32, start: u64, end: u64) -> u32 {
    let mut total = 0;
    for i in 0..num_dials {
        let start_digit = (start / 10u64.pow(i)) % 10;
        let end_digit = (end / 10u64.pow(i)) % 10;

        let forward_steps = if end_digit > start_digit {
            end_digit - start_digit
        } else {
            start_digit - end_digit
        } as u32;
        let backward_steps = 10 - forward_steps;

        total += std::cmp::min(forward_steps, backward_steps);
    }
    total
}

fn main() {
    let mut input = String::new();
    // Read input from stdin
    stdin().read_line(&mut input).unwrap();
    let parts: Vec<u64> = input
        .trim()
        .split_whitespace()
        .map(|x| x.parse().unwrap())
        .collect();

    let n = parts[0] as u32;
    let s = parts[1];
    let e = parts[2];

    let t = 1000;
    let start_t = Instant::now();

    for _ in 0..t {
        let _ = find_min_combi(n, s, e);
    }
    println!("{}", find_min_combi(n, s, e));

    let duration = start_t.elapsed();
    println!("Time taken by function: {:?}", duration);
}