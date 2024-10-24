use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn compareTriplets(a: &[i32], b: &[i32]) -> Vec<i32> {
    let mut score_a = 0;
    let mut score_b = 0;

    for (x, y) in a.iter().zip(b.iter()) {
        if x > y {
            score_a += 1;
        } else if x < y {
            score_b += 1;
        }
    }

    vec![score_a, score_b]
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();
