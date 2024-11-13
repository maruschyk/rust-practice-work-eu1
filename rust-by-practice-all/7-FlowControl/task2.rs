fn main() {
    let n = 5;

    let big_n = if n < 10 && n > -10 {
        println!("{}, and is a small number, increase ten-fold", n);
        10 * n // Return an integer result
    } else {
        println!("{}, and is a big number, halve the number", n);
        n / 2 // Return an integer result
    };

    println!("{} -> {}", n, big_n);
}
