fn main() {
    let arr: [char; 3] = ['中', '国', '人'];

    let slice = &arr[..2];

    // Modify '8' to make it work:
    // Each char in the slice takes 4 bytes, so the total size is 2 * 4 = 8 bytes.
    assert!(std::mem::size_of_val(&slice) == 8);

    println!("Success!");
}
