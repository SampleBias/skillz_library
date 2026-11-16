---
name: patrs
description: Develop or modify Rust code using appropriate, idiomatic patterns and minimal abstractions. Use for Rust implementation work, including repairs after VERS review; independent verification belongs to VERS.
---

# PATRS — Patterns for Rust Systems

Develop correct, maintainable Rust. Use ownership and the type system to prevent invalid behavior when practical, while preserving project conventions and choosing the simplest design that meets the requirements. Patterns are tools, not a checklist to apply to every project.

## Workflow

1. Inspect the relevant code, requirements, repository instructions, toolchain, targets, features, tests, and project conventions. Establish the expected behavior and constraints before choosing a design.
2. Implement the smallest cohesive change. Borrow when ownership need not transfer; model meaningful alternatives and errors explicitly; keep resource ownership clear. Avoid unnecessary clones, shared ownership, dependencies, and speculative abstractions.
3. Add meaningful tests for behavior, edge cases, and important invariants as appropriate to the change. Explain non-obvious design decisions in code or project documentation when useful.
4. Run `cargo fmt`, applicable `cargo check` commands, `cargo clippy`, and relevant `cargo test` commands using the project's workspace, features, targets, and CI conventions as appropriate. Choose non-mutating formatting checks when formatting the whole project would alter unrelated files. Cargo commands can execute build scripts and tests: inspect and respect the environment and authorization before execution. Report exact commands, results, and any checks that failed, were unavailable, or were not run; never claim a check passed without evidence.
5. Summarize the changes and remaining risks. Development checks are not independent verification. When VERS is available, hand off the candidate revision and relevant requirements for independent `$vers` review if the host and user authorization permit it; otherwise state that independent review remains pending. Do not claim a VERS verdict or spawn an agent without authorization.

For a reported VERS defect, inspect its evidence, identify the root cause, make the smallest adequate repair, add a regression test when appropriate, and rerun applicable development checks. Distinguish a demonstrated Rust defect from a missing verification prerequisite: resolve an authorized prerequisite or hand it to the appropriate verifier, but do not change code merely to silence a block. Return the changed candidate for a fresh independent review when available. Never weaken tests or bypass verification merely to obtain a pass.

## Design decisions

- Use `Result` for recoverable failure and `Option` for meaningful absence; preserve useful error context. Do not silently discard errors or use panic for ordinary control flow.
- Use enums for meaningful alternatives, newtypes for domain distinctions, and typestate when compile-time state transitions justify the API cost.
- Use traits for actual shared behavior; choose generics or trait objects according to the need for static or runtime dispatch. Prefer composition and cohesive modules to speculative frameworks.
- Let ownership and RAII manage resources. Use interior mutability or `Rc`/`Arc` only for a concrete shared mutation or ownership need.
- Prefer expressive iterators when they improve clarity. Use async for suitable concurrent I/O, channels when they clarify ownership, and replaceable external services where testability or configuration warrants it.
- Prefer safe Rust. If `unsafe` is justified, minimize and encapsulate it, document the invariants and each operation's safety rationale, and expose a safe API only if its contract can always be upheld.

## References

Read only the guidance relevant to the implementation:

- [Ownership and resources](references/ownership.md): borrowing, shared ownership, interior mutability, RAII and iteration.
- [Error handling](references/error-handling.md): `Result`, `Option`, error boundaries and context.
- [Type-driven design](references/type-driven-design.md): enums, newtypes, typestate and builders.
- [Traits and generics](references/traits-generics.md): abstraction and dispatch choices.
- [Concurrency](references/concurrency.md): async, synchronization, channels and cancellation.
- [Architecture](references/architecture.md): composition, dependencies, testing seams and unsafe boundaries.
