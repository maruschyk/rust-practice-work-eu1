fn main() {
    assert!((0.1 + 0.2 - 0.3).abs() < f64::EPSILON);

    println!("Success!");
}
