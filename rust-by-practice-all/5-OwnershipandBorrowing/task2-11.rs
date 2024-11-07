fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    let r2 = &mut s;

    // Add this line to make the error
    println!("{}, {}", r1, r2);  // Using both `r1` and `r2` at the same time causes the error
}
