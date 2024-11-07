fn main() {
    let mut s = String::from("hello, ");

    let r1 = &mut s;
    r1.push_str("world");
    
    // Comment out the following line to allow only one mutable borrow at a time
    let r2 = &mut s;
    r2.push_str("!");
    
    println!("{}", r1);
}
