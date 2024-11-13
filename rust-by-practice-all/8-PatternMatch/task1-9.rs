fn main() {
    let age = Some(30);
    if let Some(age) = age {
       assert_eq!(age, 30);
    }

    match age {
        Some(age) =>  println!("age is a new variable, it's value is {}", age),
        _ => ()
    }
}
