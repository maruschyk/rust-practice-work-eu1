fn main() {
    // Modify the raw string to use a different delimiter
    let raw_str = r#"Escapes don't work here: \x3F \u{211D}"#;
    assert_eq!(raw_str, "Escapes don't work here: ? ℝ");

    // ... rest of the code ...

    // Fill the blank with a long delimiter
    let long_delimiter = r###"Hello, "##""###;
    assert_eq!(long_delimiter, "Hello, \"##\"");

    println!("Success!");
}
