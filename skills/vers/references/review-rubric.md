# Independent review and loop

G0: establish exact requirements, HEAD, full dirty state and source identity, tool versions, workspace members, features, supported targets and MSRV. Review project CI/lints and execution risks before running code. Do not assume `--all-features` is valid; use the project's supported feature matrix and report untested combinations.

G3: derive observable behavior contracts independent of the implementation. Exercise valid, boundary and malformed inputs, state transitions, failures, serialization and arithmetic edge cases. Run relevant unit, integration and regression tests; add property, differential, fuzz or concurrency/model tests only where appropriate and authorized. Preserve reproducible counterexamples; never weaken an assertion to pass.

G4: inspect dependencies, provenance, licenses, advisories, secrets, permissions, command invocation, input validation, deserialization, paths, network trust boundaries and exhaustion. Run `cargo audit` and configured `cargo deny` when available and appropriate; distinguish unavailable tools and stale/offline advisory databases from a clean result.

G5: assess module boundaries, ownership, public APIs, error handling, duplication, avoidable allocations/cloning and complexity. Report material defects separately from subjective preferences; avoid architecture changes solely for style. Check lints and suppressions without silently hiding baseline warnings.

Track findings by evidence and reproduction. IMPLEMENT → VERIFY → TRIAGE → REPAIR → VERIFY; repairs belong to the implementer. Stop when the candidate passes all applicable requirements, verification is blocked, a finding repeats without progress, or the agreed budget expires. All evidence belongs to one exact candidate: re-snapshot and rerun after every change. The v0.1 harness cannot synthesize an expert review or a PASS for these gates.
