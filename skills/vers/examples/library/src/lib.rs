/// Increment an integer, returning None at the representable boundary.
pub fn increment(value: u32) -> Option<u32> {
    value.checked_add(1)
}
