fn main() {
    let _v: () = ();

    let v = (2, 3);
    assert_eq!(v, implicitly_ret_unit());

    println!("Success!");
}

fn implicitly_ret_unit() -> () { // Explicitly specify the return type as `()`
    println!("I will return a ()");
}
