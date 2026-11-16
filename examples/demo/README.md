# VERS demo

Run `python3 examples/demo/run.py` from the repository root. The script copies the dependency-free crate into temporary Git repositories, commits the baseline, calls the actual `skills/vers/scripts/verify.sh` wrapper, checks report evidence paths and exit codes, and compares stable report fields with the files in `expected/`. Temporary projects and reports are removed afterward. The crate and seeded changes are local code, but Cargo and runtime commands execute with the caller's privileges.

Requirements: Python 3, Git, `timeout`, Rust, rustfmt, Clippy, and the VERS dependencies cached for its `--offline --locked` build. To also validate every generated VERS report and the illustrative QC checkpoint against their JSON schemas, install Python `jsonschema` and run `python3 examples/demo/run.py --validate-schema`. Schema validity alone does not establish a valid QC decision or trustworthy evidence.

| Scenario | VERS result | What happened | QC consequence |
| --- | --- | --- | --- |
| `blocked` | BLOCKED | Checks and runtime contracts pass, but independent review evidence is absent. | Hold acceptance until qualifying evidence exists. |
| `pass` | PASS | The same crate is rerun with **fabricated** review records and the trust flag to exercise the import protocol. | This synthetic pass is not qualifying evidence and must not be used to accept the crate. |
| `fail-runtime` | FAIL | Help text is changed; tests pass, but the independently expected runtime output differs. Reviews are still blocked. | Reject the failed mandatory check; repair and reverify. |
| `fail-test` | FAIL | Greeting output is changed and the unit test fails. Reviews are still blocked. | Reject the failed mandatory check; repair and reverify. |

The VERS golden files contain only stable verdict, next-action and selected check statuses; hashes, Git identities, tool versions, logs and report paths vary by run. `expected/qc-blocked.json` and `expected/qc-fail.json` show illustrative QC reports with unscored categories, not actual QC assessments; the synthetic VERS pass would still leave QC blocked. The pass scenario deliberately misrepresents review provenance to the harness as a controlled test. Never use this technique on a real candidate: the operator must actually establish reviewer independence and provenance before `--trust-reviews`. See [review evidence](../../skills/vers/references/review-evidence.md). The separate [QC checkpoint example](../../skills/qc/examples/checkpoint.example.json) is fictional too.
