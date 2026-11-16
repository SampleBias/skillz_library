# Scoring and evidence

## Establish the review

Identify scope and its requirements and risk, applicable mandatory checks, configuration, and evidence before scoring. The user or project may establish different thresholds, weights, or deductions; document agreed changes and their rationale in the report rather than silently changing them. Category maxima must still total 100. Critical findings always reject, mandatory failures always reject, and missing mandatory evidence always blocks acceptance. The default 85 and 90 are proposed governance settings, not calibrated probabilities or guarantees of correctness. Calibrate with seeded defects and observed outcomes before relying on them for consequential decisions.

File acceptance requires a SHA-256 of the exact bytes, applicable PATRS and AMARK review, and an overall PASS from relevant independent VERS verification for the candidate and its dependencies. An overall VERS PASS is also mandatory for milestone and project acceptance. Assess all six categories for the file in context. A project or milestone assessment independently examines requirements, module interactions, dependencies, runtime behavior and relevant targets/features, plus accepted files where required. File scores are not project scores. Record the verification tool and version, verdict, check identifiers, candidate identity, evidence location and relevant configuration. Confirm freshness against actual bytes and relevant inputs; a commit ID alone omits dirty and untracked content. Note external services, dependencies, toolchain, environment and other inputs that hashes do not capture.

VERS v0.1 hashes included paths, contents and permissions, including project `.qc` content, but excludes root `.vers/reports`. Its overall `BLOCKED` cannot be overridden by a separate review or category score. Store QC output under `.qc/reports/` at a trusted evidence root **outside** the workspace being verified; separately protect it from candidate-controlled code. Putting it inside the workspace changes the candidate and makes prior VERS evidence stale. Do not rely on ignore rules to change VERS's source hash. If other tools use a different snapshot scheme, verify exclusions and evidence independence explicitly.

## Rubric

Start each category at its maximum only after it has been adequately assessed. Record evidence for each assessed category, including where no defects were found. If the scope lacks sufficient evidence for any category, leave the total and affected category score null; do not convert uncertainty into zero deductions or a passing score.

| Category | Max | Assess |
| --- | ---: | --- |
| correctness | 25 | Contracts, behavior, edge cases |
| rust_safety | 25 | Ownership, unsafe invariants, concurrency |
| testing | 15 | Relevant passing tests, regressions, boundaries |
| maintainability | 15 | PATRS and AMARK compliance, architecture, simplicity |
| security | 10 | Validation, dependencies, trust boundaries |
| reliability | 10 | Error recovery, resource management, runtime behavior |

Under the default policy, for each confirmed finding, record a stable ID, criterion, affected location, supporting evidence, severity, root cause and recommended action. Deduct critical 25, high 10, medium 4, or low 1 from one primary category. Use severity based on demonstrated impact: critical includes confirmed undefined behavior or exploitable security defects; high includes incorrect core behavior or reproducible corruption; medium includes a noncritical unhandled edge case; low includes a demonstrated minor maintainability problem. Document the rationale. Information and unverified suspicions are limitations, not scored findings. Consolidate findings with the same root cause, assigning the highest applicable severity and one primary category; retain all affected locations and evidence. Do not deduct the same root cause twice.

For each category: `score = max(0, maximum - sum(deductions))`. Total is the sum of the six category scores, 0–100. Category caps prevent spillover. List raw deductions and final category scores so the arithmetic can be audited. Findings remain visible even if their category is capped. A critical finding triggers rejection even where its deduction is capped.

## Decision precedence

1. Any confirmed critical finding or failed mandatory check: `REJECT`. List any blocked checks too.
2. Otherwise, missing/stale mandatory evidence, inadequate assessment, or unknown required applicability: `BLOCKED` (and no total if a category is inadequately assessed).
3. Otherwise, score below the applicable threshold: `REJECT`.
4. Otherwise: `ACCEPT` only after all scope-specific conditions hold. An accepted file is tied to its exact content hash and candidate context; it is not a project acceptance.

A noncritical finding may coexist with acceptance when the threshold is met, no mandatory rule fails, and project policy allows it. Record remediation and risk explicitly. Use `NOT_APPLICABLE` only with a rationale. Preserve failed and blocked evidence as well as passing evidence.

Reports follow [report.schema.json](../schemas/report.schema.json); schema conformance alone does not prove arithmetic, freshness, authorization, or truth. Record the assessment timestamp, effective policy and its source, candidate and evidence identities, findings, checks, calculations, decision, lifecycle state and next action. Never place secrets in reports. This package includes no scorer or validator; the agent must perform and verify the review and arithmetic itself.
