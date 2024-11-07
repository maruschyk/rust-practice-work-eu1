fn main() {
    println!("Success!");
}

fn get_option(tp: u8) -> Option<i32> {
    match tp {
        1 => {
            // Some logic can go here
        }
        _ => {
            // Some logic can go here
        }
    };

    never_return_fn()
}

// Third way: Using infinite recursion to never return
fn never_return_fn() -> ! {
    never_return_fn()  // Calls itself, leading to infinite recursion
}
