// The ? macro is how to shortcircuit control flow in a function.
enum Outcome {
    Fizz(i32),
    Buzz(i32),
    Nothing(i32)
}
fn find_fizzbuzz(x: i32) -> Result<i32, Outcome> {
    match (x % 3 == 0, x % 5 == 0) {
        (true, true) => Ok(x),
        (true, false) => Err(Outcome::Fizz(x)),
        (false, true) => Err(Outcome::Buzz(x)),
        (false, false) => Err(Outcome::Nothing(x)),
    }
}
fn pairs_of_fizzbuzz(x: i32, y: i32) -> Result<(i32, i32), Outcome> {
    let x = find_fizzbuzz(x)?;
    let y = find_fizzbuzz(y)?;
    Ok((x, y))
}
// This would be Ok((15, 20))
pairs_of_fizzbuzz(15, 20)
