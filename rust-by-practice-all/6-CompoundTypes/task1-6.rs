fn main() {
    let s1 = String::from("hello,");
    let s2 = String::from("world!");
    let s3 = s1 + &s2;  // Borrow s2 (reference) instead of taking ownership

    assert_eq!(s3, "hello,world!");
    println!("{}", s1);  // s1 is still usable because we only borrowed s2

}
