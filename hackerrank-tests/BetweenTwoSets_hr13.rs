use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};
use std::iter::FromIterator;

/// Computes the Greatest Common Divisor (GCD) of two numbers
fn gcd(a: i32, b: i32) -> i32 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// Computes the Least Common Multiple (LCM) of two numbers
fn lcm(a: i32, b: i32) -> i32 {
    a * (b / gcd(a, b))
}

/// Computes the LCM of an array of numbers
fn lcm_of_array(arr: &[i32]) -> i32 {
    arr.iter().cloned().reduce(lcm).unwrap_or(1)
}

/// Computes the GCD of an array of numbers
fn gcd_of_array(arr: &[i32]) -> i32 {
    arr.iter().cloned().reduce(gcd).unwrap_or(1)
}

/// Function to calculate total integers that satisfy the conditions
fn getTotalX(a: &[i32], b: &[i32]) -> i32 {
    let lcm_a = lcm_of_array(a);
    let gcd_b = gcd_of_array(b);

    let mut count = 0;
    let mut multiple = lcm_a;

    while multiple <= gcd_b {
        if gcd_b % multiple == 0 {
            count += 1;
        }
        multiple += lcm_a;
    }

    count
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let first_multiple_input: Vec<String> = stdin_iterator.next().unwrap().unwrap()
        .split(' ')
        .map(|s| s.to_string())
        .collect();

    let n = first_multiple_input[0].trim().parse::<i32>().unwrap();
    let m = first_multiple_input[1].trim().parse::<i32>().unwrap();

    let arr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let brr: Vec<i32> = stdin_iterator.next().unwrap().unwrap()
        .trim_end()
        .split(' ')
        .map(|s| s.to_string().parse::<i32>().unwrap())
        .collect();

    let total = getTotalX(&arr, &brr);

    writeln!(&mut fptr, "{}", total).ok();
}
