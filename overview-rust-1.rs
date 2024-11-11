// Variable bindings (immutable)
let something = 123;
// Variable bindings (mutable) (also shadows!)
let mut something = 456;
// Slices (fixed-size arrays known at compile time)
let a = [123, 456];
// Vectors (arrays with size known at runtime)
let v = vec![123, 456];
// Importing packages ("crates")
use std::collections::HashMap;
// Hashmaps (objects/maps)
let mut h = Hashmap::new();
// Inserting into the hashmap.
h.insert("Hello", "World");
// Function that returns a number.
fn number(x: i32) -> i32 {
  x + 123
}
