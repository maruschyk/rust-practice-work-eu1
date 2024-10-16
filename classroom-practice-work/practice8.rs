fn swap_case(s: &str) -> String {
    s.chars()
        .map(|c| {
            if c.is_ascii_uppercase() {
                c.to_ascii_lowercase()
            } else if c.is_ascii_lowercase() {
                c.to_ascii_uppercase()
            } else {
                c // Не змінюємо символи, які не є буквами
            }
        })
        .collect()
}

fn main() {
    let input = "Hello, World!";
    let result = swap_case(input);
    println!("{}", result); // hELLO, wORLD!
}
