# Error handling

## Decisions

- Use `Option<T>` for expected absence and `Result<T, E>` for recoverable failure. Preserve the distinction when missing data and failed lookup have different meanings; `Result<Option<T>, E>` can express both.
- Propagate with `?` and add useful context at boundaries. Use typed variants when callers must distinguish causes; applications may use a general error wrapper when their policy is to report rather than branch. Follow the existing project's conventions.
- Handle errors from I/O, sends, parsing, and cleanup deliberately. If ignoring one is intentional, explain why and ensure it cannot hide a required outcome. Avoid `unwrap`/`expect` on inputs or ordinary failures; a panic can be appropriate for an established invariant, with a clear explanation.

## Example

```rust
// Good: absence differs from a failed query.
fn find_user(id: u64) -> Result<Option<User>, StoreError> {
    store_lookup(id)
}

// Poor: hides store failures as missing users.
fn find_user_lossy(id: u64) -> Option<User> {
    store_lookup(id).ok().flatten()
}
```

`User`, `StoreError`, and `store_lookup` stand for project types and functions. Mapping errors to absence can be appropriate only when the contract explicitly permits that loss. Common generated-code antipatterns include catching every error and returning a default, discarding `Result`s, and converting all errors to strings too early for callers to make decisions.
