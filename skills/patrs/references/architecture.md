# Architecture and unsafe boundaries

## Decisions

- Group related behavior into small cohesive modules, keep implementation details private, and compose concrete pieces before inventing a framework. Follow existing repository structure and avoid dependencies that do not repay their maintenance cost.
- Make an external service replaceable where the test or application needs it: a function parameter, standard trait, local trait, or configurable client can suffice. Avoid an elaborate dependency injection container for simple code. Test public behavior and invariants rather than implementation details.
- Prefer safe APIs and libraries. When unsafe code is necessary, minimize the unsafe surface, specify caller obligations on unsafe functions, explain why each unsafe operation meets its invariants, and ensure safe callers cannot violate the abstraction. Consider aliasing, lifetimes, initialization, thread safety, and panic paths as relevant. A safety comment is not evidence that the invariant holds.

## Example

```rust
use std::io::{self, Write};

// Good: the caller chooses the output, and tests can provide a Vec<u8>.
fn report(out: &mut impl Write, count: usize) -> io::Result<()> {
    writeln!(out, "processed {count}")
}

// Poor: hides an external side effect and makes it hard to isolate.
fn report_global(count: usize) {
    println!("processed {count}");
}
```

Direct stdout can be appropriate for a small CLI whose contract is specifically console output. Common generated-code antipatterns include wrapper modules that only forward calls, unnecessary dependencies, mocks for pure calculations, and unsafe wrappers that expose unchecked invariants through safe methods.
