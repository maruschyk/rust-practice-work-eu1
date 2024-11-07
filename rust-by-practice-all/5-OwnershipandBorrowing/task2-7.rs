fn main() {
    let mut s = String::from("hello");

    let r1 = &mut s;
    // Remove the second mutable reference to `s`
    // let r2 = &mut s;

    println!("{}", r1);

    println!("Success!");
}
