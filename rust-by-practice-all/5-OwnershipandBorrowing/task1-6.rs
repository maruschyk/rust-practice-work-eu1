fn main() {
    let mut s = String::from("Hello ");  // Make `s` mutable

    let mut s1 = s;  // Make `s1` mutable to modify it

    s1.push_str("World!");  // Modify `s1` (which now owns the String)

    println!("{}", s1);  // Print `s1` after modification
}
