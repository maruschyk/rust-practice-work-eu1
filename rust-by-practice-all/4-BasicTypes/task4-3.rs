fn main() {
    never_return();

    println!("Failed!");
}

fn never_return() -> ! {
    loop {
        // Infinite loop, this function never returns
    }
}
