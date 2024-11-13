fn main() {
    for n in 1..100 { // The range now excludes 100
        if n == 100 {
            panic!("NEVER LET THIS RUN");
        }
    }

    println!("Success!");
}
