# Ownership and resources

## Decisions

- Accept a borrow when the callee only needs temporary access; take ownership when storing, transforming, or transferring a value. Return owned data when it must outlive its source. Use slices and `&str` for borrowed views when suitable.
- Clone deliberately to satisfy a real ownership or lifetime need, not to silence the borrow checker. A cheap clone such as an `Arc` clone is still shared ownership, not an independent copy.
- Start with ordinary ownership and `&mut`; introduce `Rc` for single-threaded shared ownership or `Arc` for cross-thread shared ownership only when needed. Add `Cell`, `RefCell`, a mutex, or an atomic only for an actual shared mutation requirement; respect runtime borrow failures and synchronization costs.
- Bind cleanup to an owning guard or existing RAII type. `Drop` cannot report errors to its caller; expose an explicit fallible `finish` or `close` when callers must observe finalization failures, with `Drop` as best-effort cleanup.
- Use iterators for clear transformations; use a loop when mutation, branching, early exit, or error reporting is clearer.

## Example

```rust
// Good: inspect without consuming or copying the collection.
fn total(values: &[u64]) -> u64 {
    values.iter().sum()
}

// Poor: consumes the caller's Vec just to inspect it.
fn total_consuming(values: Vec<u64>) -> u64 {
    values.iter().sum()
}
```

Taking the `Vec` can be correct when ownership transfer is part of the API. A common generated-code antipattern is cloning every field or wrapping every value in `Arc<Mutex<_>>` without a demonstrated need; both obscure ownership and add costs.
