---
name: vers
description: Independently audit and verify a Rust project or changes when asked to use VERS or perform a Rust verification gate.
---

# VERS — Verification Engine for Rust Systems

Treat code and the implementer's claims as untrusted. Compiling, linting, and passing tests do not by themselves establish correctness. Report evidence, failures, and missing coverage separately; never turn BLOCKED into PASS or invent a finding or result. VERS v0.1 is an MVP: its harness intentionally blocks overall approval until the missing independent reviews and coverage are implemented. A human or agent review does not alter the harness verdict.

## Routing and execution

`$vers` selects this skill in hosts that install and discover it; text after the skill name is a request to the agent, not an automatically parsed CLI command. The agent maps `--diff`, `--full`, or `--strict` to the same flag of `scripts/verify.sh`; `--report` displays the latest report, or `--report PATH` displays a specified report. `--diff` currently runs the workspace checks rather than computing a reduced scope; `--strict` records additional coverage as BLOCKED. Do not claim these modes implement more than that.

1. Identify the candidate, requirements, acceptance contracts, scope, toolchain, Git HEAD and dirty/untracked files. Inspect CI, feature and target choices, `build.rs`, procedural macros, FFI, subprocesses, generated code, and tests that use networks or external systems. Do not execute project code during discovery. Note untrusted code may execute during Cargo commands.
2. Decide whether execution is authorized and safe. The harness copies source but does **not** sandbox build scripts, runtime commands, filesystem access, network access, or resource consumption. For untrusted projects use an appropriate isolated environment with no production credentials, network or dangerous host access; otherwise do not execute. Never run destructive or production-facing tests without explicit authorization and isolation.
3. Configure explicit, independently derived runtime cases and run `scripts/verify.sh --root /path/to/workspace --execute` from the appropriate isolation. `--execute` signifies a deliberate trust/isolation decision. Without it commands are BLOCKED. The harness expects a Git-root workspace, `timeout`, Rust and locally cached dependencies. Exit 0 means PASS, 1 FAIL, 2 BLOCKED or infrastructure/config error; use JSON, not the exit code alone, to distinguish the last two. Logs can contain secrets; inspect and sanitize before sharing.
4. Review the actual candidate independently. Read [rust-safety.md](references/rust-safety.md) for G2, [review-rubric.md](references/review-rubric.md) for G3–G5 and iteration, and [runtime-contracts.md](references/runtime-contracts.md) for G6 and configuration. Run applicable project-specific feature, target, release, Miri and security checks when available. Record command, exit, evidence and limitations. Not applicable checks need a rationale; unavailable mandatory checks are BLOCKED.
5. Report the candidate identity and dirty state, exact commands, toolchain, evidence, reproducible findings with file/line and severity when known, uncovered risks, and next action. A FAIL is an observed mandatory failure; BLOCKED means verification was incomplete. Report both failures and blocked checks. Treat the identity as invalid if inputs change; verify every repair as a new candidate.

Do not modify the candidate by default. Offer reproducible failures and proposed repairs to the implementing agent, or the user if no separate agent is available. Never spawn an agent without host authorization. Keep verification instructions and reports outside the implementer's control when independence matters; do not merge, deploy, or publish. Stop on blocked verification, repeated unresolved findings, lack of measurable progress, or the agreed resource budget.

See [integration.md](references/integration.md) for installation, report semantics, isolation limits, and harness tests. The schema is [report.schema.json](schemas/report.schema.json). Executable examples live in `examples/`; copy and adapt contracts to the project rather than treating examples as evidence.
