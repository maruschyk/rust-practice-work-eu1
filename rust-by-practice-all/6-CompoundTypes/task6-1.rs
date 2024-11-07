enum Number {
    Zero,
    One,
    Two,
}

enum Number1 {
    Zero = 0,
    One = 1,
    Two = 2,
}

// C-like enum
enum Number2 {
    Zero = 0.0,
    One = 1.0,
    Two = 2.0,
}

fn main() {
    // An enum variant can be converted to a integer by `as`
    // assert_eq!(Number::One, Number1::One); // Different enum types cannot be compared directly
    assert_eq!(Number1::One as i32, Number2::One as i32); // Compare the underlying integer values

    println!("Success!");
}
