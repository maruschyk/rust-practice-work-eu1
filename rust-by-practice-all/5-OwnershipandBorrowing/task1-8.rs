fn main() {
   let t = (String::from("hello"), String::from("world"));

   let _s = &t.0;  // Borrow `t.0` instead of moving it

   // Now `t` is still valid, and we can print it
   println!("{:?}", t);  // This works because `t` is not moved
}
