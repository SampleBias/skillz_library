# Type-driven design

## Decisions

- Use an enum instead of independent flags when only certain alternatives are legal. Add newtypes when mixing same-representation values would cause domain mistakes; validate at construction when the invariant needs it.
- Use typestate when a critical, statically known transition benefits callers enough to justify extra types and API complexity. Runtime enums are often simpler for state read from external data or stored in a collection.
- Use a builder for construction with several optional or interdependent settings; validate required fields and cross-field constraints in `build`. A constructor or struct literal is simpler for a few obvious fields.

## Example

```rust
// Good: a job cannot be both queued and finished.
enum JobState {
    Queued,
    Running,
    Finished { output: String },
}

// Poor: contradictory combinations are representable.
struct JobFlags {
    queued: bool,
    running: bool,
    finished: bool,
    output: Option<String>,
}
```

An enum alone does not validate every transition: enforce transition rules in the API or a state machine where needed. Common generated-code antipatterns include a newtype for every primitive without a domain invariant, typestate for trivial settings, and builders that merely reproduce a two-field constructor.
