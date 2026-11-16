# Traits and generics

## Decisions

- Introduce a trait for a real shared capability, a useful boundary, or existing polymorphism, not preemptively for every struct. Keep its methods cohesive and its visibility as narrow as practical.
- Use generics for compile-time polymorphism when they simplify the API or permit useful static dispatch. They can increase compile time, binary size, and exposed type complexity. Use `dyn Trait` when heterogeneous implementations or runtime selection are valuable and the trait supports dynamic dispatch; account for indirection and allocation where applicable.
- Prefer concrete types and functions when there is one implementation and no meaningful substitution need. Avoid generic parameters and lifetimes that leak implementation details without a benefit.

## Example

```rust
use std::io::{self, Write};

// Good: accepts a real standard capability and is easy to test in memory.
fn write_message(out: &mut impl Write, message: &str) -> io::Result<()> {
    out.write_all(message.as_bytes())
}

// Poor: a custom trait with one implementation and no consumer need.
trait MessageWriterFactoryProvider {
    fn make_writer(&self) -> Box<dyn Write>;
}
```

A factory trait can be warranted for runtime-selected external resources. Common generated-code antipatterns include layered one-method traits, boxed trait objects everywhere, and forcing static dispatch when dynamic configuration is a real requirement.
