fn main() {
    let names = [String::from("liming"), String::from("hanmeimei")];
    for name in &names {  // Borrowing the elements using references
        // Do something with name...
    }

    println!("{:?}", names);  // Now you can access names because the elements were borrowed

    let numbers = [1, 2, 3];
    for n in numbers {  // This works fine because the elements in numbers are Copy
        // Do something with n...
    }
    
    println!("{:?}", numbers);  // You can access numbers because integers are Copy
}
