use std::env;
use std::fs::File;
use std::io::{self, BufRead, Write};

fn diagonalDifference(arr: &[Vec<i32>]) -> i32 {
    let n = arr.len();
    let mut primary_diagonal_sum = 0;
    let mut secondary_diagonal_sum = 0;

    for i in 0..n {
        primary_diagonal_sum += arr[i][i];            // Sum for primary diagonal
        secondary_diagonal_sum += arr[i][n - 1 - i]; // Sum for secondary diagonal
    }

    (primary_diagonal_sum - secondary_diagonal_sum).abs() // Return the absolute difference
}

fn main() {
    let stdin = io::stdin();
    let mut stdin_iterator = stdin.lock().lines();

    let mut fptr = File::create(env::var("OUTPUT_PATH").unwrap()).unwrap();

    let n = stdin_iterator.next().unwrap().unwrap().trim().parse::<usize>().unwrap();

    let mut arr: Vec<Vec<i32>> = Vec::with_capacity(n);

    for _ in 0..n {
        arr.push(stdin_iterator.next().unwrap().unwrap()
            .trim_end()
            .split(' ')
            .map(|s| s.parse::<i32>().unwrap())
            .collect());
    }

    let result = diagonalDifference(&arr);

    writeln!(&mut fptr, "{}", result).ok();
}
