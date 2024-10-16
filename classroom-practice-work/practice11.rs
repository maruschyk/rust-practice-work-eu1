fn is_palindrome(x: u32) -> bool {
    let original = x;
    let mut reversed = 0;
    let mut n = x;

    while n > 0 {
        reversed = reversed * 10 + n % 10;
        n /= 10;
    }

    original == reversed
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let data = [
            (123, false),
            (121, true),
            (1221, true),
        ];

        data.iter().for_each(|(n, exp)| {
            assert_eq!(is_palindrome(*n), *exp);
        });
    }
}

fn main() {
    // You can call your function here for a quick test
    let numbers = [123, 121, 1221];
    for &num in &numbers {
        println!("Is {} a palindrome? {}", num, is_palindrome(num));
    }
}
