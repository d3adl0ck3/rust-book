pub fn add_two(value: i32) -> i32 {
    internal_adder(value,2)
}
fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}
