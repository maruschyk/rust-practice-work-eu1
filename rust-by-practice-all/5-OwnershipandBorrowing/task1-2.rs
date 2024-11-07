fn take_ownership(s: String) -> String {
    println!("{}", s);
    s // Return the String so ownership is transferred back
}
