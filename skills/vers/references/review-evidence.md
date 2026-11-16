# Independent review evidence

VERS v0.2 accepts external structured review records in a second run. The verifier validates their shape, candidate and configuration identity, configured reviewer labels, artifact hashes, and applicability; it does **not** authenticate reviewer identities, enforce independent review, verify claims, or execute the attached review tools. The operator must establish those facts and protect policy, bundle, artifacts and verifier from candidate-controlled code before explicitly passing `--trust-reviews`. An arbitrary self-attestation, names in a file, or an agent reviewing its own changes does not establish independence. `--trust-reviews` is an operator attestation, not cryptographic authentication. Do not use it unless the necessary provenance and independence have actually been verified. The checks remain BLOCKED without it.

1. Inspect the candidate and execution risks, configure independent runtime contracts, and run `scripts/verify.sh --root /path/to/workspace --execute` in appropriate isolation. The preliminary report contains `source_sha256` and `config_sha256`; its review checks will be BLOCKED until evidence is supplied. Any failed check still needs repair and a new candidate.
2. An independent reviewer examines that exact snapshot and records substantive supporting evidence for the eight IDs below. Use [rust-safety.md](rust-safety.md) and [review-rubric.md](review-rubric.md), plus applicable project requirements. Run appropriate Miri, fuzz, coverage, security, feature and target checks separately in a trusted environment. Document commands, outcomes, applicability and limitations in artifacts. Do not record a pass for missing evidence, a failed check, or a review not performed. If an optional tool is not applicable, record why; tool unavailability is not itself an applicability rationale.
3. An authorized operator maintains the policy, verifies the independent review provenance and artifact contents, and places policy, bundle and artifacts in protected storage **outside the candidate**. Paths inside the candidate are rejected, including `.vers/reports`. Artifacts must be nonempty regular files under the bundle's directory and identified by their SHA-256. Policy and bundle are JSON, version `"1"`; unrecognized fields are rejected.
4. Re-run with the same unchanged candidate and configuration: `scripts/verify.sh --root /path/to/workspace --execute --review-policy /trusted/policy.json --review-bundle /trusted/bundle.json --trust-reviews`. The harness reruns automated checks and validates review inputs against the new snapshot. Any changed source, config or artifact requires a fresh relevant review and bundle. Missing, duplicate, unauthorized, malformed, stale or unverifiable records block approval. A review FAIL remains a failure; FAIL takes precedence over BLOCKED. Strict mode cannot PASS because its extra coverage is not implemented; it reports BLOCKED unless another check fails.

Example **policy** (illustrative labels only; actual provenance must be established outside VERS):

```json
{"version":"1", "implementers":["developer-id"], "authorized_reviewers":["independent-reviewer-id"]}
```

The lists must be nonempty, unique, trimmed and disjoint. The caller selects the policy: VERS cannot tell whether its author is authorized or its labels correspond to real people or agents. Trust and store it independently of the implementer and candidate. A malicious or mistaken operator can create a misleading PASS.

Example **bundle** structure (repeat the review object for each ID; hashes below are placeholders):

```json
{
  "version": "1",
  "source_sha256": "<source_sha256 from preliminary report>",
  "config_sha256": "<config_sha256 from preliminary report>",
  "reviews": [{
    "id": "G2-SAFETY",
    "reviewer": "independent-reviewer-id",
    "status": "PASS",
    "rationale": "Explain what was reviewed, scope, results, and limitations.",
    "artifact": "safety-review.txt",
    "artifact_sha256": "<sha256 of safety-review.txt>"
  }]
}
```

Compute each artifact digest with a trusted tool such as `sha256sum /trusted/safety-review.txt`. Every ID needs exactly one record: `G0-COVERAGE`, `G2-SAFETY`, `G3-CONTRACT`, `G4-SECURITY`, `G5-ARCHITECTURE`, `G2-MIRI`, `G3-FUZZ`, and `G3-COVERAGE`. Each record needs a nonempty rationale, a policy-authorized reviewer distinct from all listed implementers, and an artifact path relative to the bundle directory with a matching SHA-256. Status is `PASS`, `FAIL`, or `NOT_APPLICABLE`. Only Miri, fuzz and coverage can be `NOT_APPLICABLE`, and only when the corresponding `.vers/config.toml` policy is `conditional`; a required check cannot be waived. The first five reviews are always required. A `PASS` for a conditional tool means the reviewer has verified its applicable execution and evidence; VERS itself does not execute it.

A final report includes a copy of the supplied policy and bundle with their hashes under `evidence/reviews.json`; it records artifact locations and hashes but does not copy artifact contents. Preserve the protected artifacts with the report. Reports, rationales, paths and logs may contain sensitive information; inspect them before sharing. Source hashes do not bind external dependencies, services, environment or toolchains; record and assess those separately. The caller must protect evidence throughout execution, including from build scripts and tests. Schema validity and artifact hashes do not prove review quality, authenticity or correctness.
