
// Structures
struct Person {
    name: String
    age: u32
}
// The Option monad
enum Option<A> {
    Some<A>,
    None
}
let a: Option<i32> = Some(123);
let a: Option<i32> = None;
// The Option in practice.
let mut h = HashMap::new();
h.insert("World", "Hello");
// This would be false!
h.get("Hello").is_some();
// Another enum exists, the Result monad.
enum Result<T, E> {
    Ok(T),
    Err(E)
}
let a = Ok(123);
