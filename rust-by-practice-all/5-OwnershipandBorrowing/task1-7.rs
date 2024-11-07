fn main() {
    let x = Box::new(5);
    
    let y = &mut *x;  // Borrow `x` as a mutable reference to the value inside the box
    
    *y = 4;  // Modify the value inside the box
    
    assert_eq!(*x, 5);  // This assertion will now work
    
    println!("Success!");
}
