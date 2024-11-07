fn main() {
    let s1 = String::from("hi,中国");

    // Access the first character as a byte slice
    let h = &s1[0..1];
    assert_eq!(h, "h");

    // Access the third and fourth bytes, which represent the Chinese character "中"
    let h1 = &s1[3..6];
    assert_eq!(h1, "中");

    println!("Success!");
}
