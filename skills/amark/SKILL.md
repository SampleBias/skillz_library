---
name: amark
description: Apply minimal, purposeful comments and documentation when developing or reviewing Rust code, alongside idiomatic Rust practices.
---

# AMARK — Minimal Annotation for Rust

Prefer code that clearly expresses what it does. Use comments to explain essential information the code cannot convey clearly, especially why a non-obvious choice is necessary. Improve names, types, and structure where practical instead of using comments to compensate for unclear code.

## Authorized comments

Use comments when needed for:

- **Safety:** Use `// SAFETY:` to explain the invariants and rationale that justify an unsafe operation. Document safety requirements of public unsafe APIs.
- **Non-obvious decisions:** Explain meaningful constraints or tradeoffs that cannot be made clear through the code.
- **Necessary workarounds:** Explain the actual constraint, link a tracking issue or reference when available, and state the condition for removal when known.
- **Public API contracts:** Use `///` documentation when callers need behavior, error, panic, or safety guarantees that the signature does not convey.

Preserve documentation and license notices required by project policy or applicable licenses. Do not impose a comment-count limit: every genuine safety explanation must remain, even when several are needed in one file.

Do not add comments that narrate obvious operations, repeat names, explain basic Rust syntax, decorate code with unnecessary section headers, or substitute for clear code. Avoid redundant TODOs, generated explanations embedded in source files, and comments that no longer describe the behavior. Preserve useful existing comments unless they are obsolete; do not remove them merely to reduce their number.

## Workarounds

Before introducing a workaround, identify and attempt to address the root cause, and prefer the simplest idiomatic Rust solution. Use a workaround only when that solution is impractical and the reason is established. Document why it is necessary, a tracking reference when available, and the removal condition when known. Never invent a constraint, issue, or removal condition.

## Development and review

Use AMARK alongside PATRS for idiomatic Rust: favor clear ownership, appropriate `Result` and `Option` handling, type-driven design, minimal abstractions, safe Rust, and composition. AMARK does not replace required safety or API documentation, functional checks, or project instructions.

Before completion, check comments against the actual code, remove unnecessary or obsolete comments, preserve required and useful documentation, and confirm workarounds have justified explanations. Hand the candidate and relevant requirements to VERS for independent verification when available and authorized; otherwise report that independent verification remains pending. Do not claim a VERS verdict or spawn an agent without authorization. Return repairs for a fresh independent review.
