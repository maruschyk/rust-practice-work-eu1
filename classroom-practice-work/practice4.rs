fn main() {
    const SIZE: usize = 5; // Розмір ромба

    for i in 0..SIZE {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }

    for i in (1..SIZE).rev() {
        let spaces = " ".repeat(SIZE - i - 1);
        let stars = "*".repeat(2 * i + 1);
        println!("{}{}", spaces, stars);
    }
}
