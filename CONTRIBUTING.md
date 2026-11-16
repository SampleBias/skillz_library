# Contributing

Keep each skill focused and independently usable. Start with a concrete behavior, a failure mode, or a missing instruction; avoid introducing a framework where a small clarification or check will do.

## Local checks

The demo requires Git, Python 3, `timeout`, Rust, rustfmt, Clippy, and cached VERS dependencies. Cargo commands execute project code, so inspect what you run.

```bash
cargo build --locked --manifest-path skills/vers/Cargo.toml
python3 examples/demo/run.py
python3 skills/vers/tests/test_harness.py
cargo test --offline --locked --manifest-path skills/vers/Cargo.toml
```

CI also runs Markdown lint, ShellCheck and JSON Schema validation. Install `jsonschema` for `python3 examples/demo/run.py --validate-schema`. The demo's synthetic reviews exercise the protocol; they are not independent verification or QC acceptance.

## Add a VERS check

Define its contract, applicability, failure versus blocked behavior and evidence first. Implement the check with stable IDs and truthful statuses, update the VERS references and schema if the report format changes, and add a meaningful regression case to the harness tests or demo. Update `examples/demo/expected/` only after inspecting the actual change in behavior. Validate the schema and run the commands above. Keep reports and review evidence outside the candidate's control when independence matters.

## Add a skill

Create `skills/<name>/SKILL.md` with a matching lowercase, hyphenated name, frontmatter description, narrow trigger and actionable instructions. Include references or scripts only when needed, document dependencies and trust boundaries, and explain how it works alone and with existing skills. Update the README, integration guidance when needed, and changelog. Test any executable behavior and avoid claiming that an instruction file enforces a workflow.

## Versioning

Record changes under `Unreleased` in [CHANGELOG.md](CHANGELOG.md). Releases use repository tags such as `v0.1.0`; pin installations to a reviewed commit or release tag when reproducibility matters. The VERS report schema and Rust package have their own versions; a repository release does not automatically change them. A maintainer should create a tag only on an approved release commit, after checks pass; no tag is implied by this guide.
