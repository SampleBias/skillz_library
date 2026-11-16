---
name: qc
description: Evaluate Rust file or project acceptance, lifecycle checkpoints, and readiness for human testing using documented verification evidence. Use when asked for a QC gate; independent technical verification belongs to VERS.
---

# QC — Quality Control and Continuous Development Gate

Make an evidence-based acceptance decision for the requested Rust file, milestone, or project. QC assesses existing work and verification; it does not replace implementation, AMARK review, independent VERS verification, or human acceptance. This skill is a specification and starter configuration, not an executable controller. Do not claim a state transition, remote verification, score, or approval that has not occurred.

## Review

1. Establish the requested scope, requirements, acceptance criteria, risks, candidate identity, repository state, and mandatory checks. Agree on any project-specific checks and any allowed configuration changes before evaluating; missing criteria or evidence block the affected decision. Read [scoring.md](references/scoring.md) and use [qc.toml](qc.toml) as defaults, not as proof of execution.
2. Inspect the candidate and trustworthy, relevant evidence, including PATRS and AMARK compliance and VERS results. Verify evidence identifies the same unchanged candidate, scope, configuration, and relevant external inputs. A file hash alone cannot validate its dependencies or integrated behavior. Report unavailable, stale, failed, and not-applicable checks separately, with reasons; never infer a pass from missing findings. Do not run untrusted project code without the applicable trust and isolation decision.
3. Apply the documented rubric, consolidate duplicate root causes, and record findings and category calculations. Produce a score only when the review and evidence are sufficient to score all categories. A critical finding or failed mandatory check rejects regardless of score; incomplete or stale mandatory evidence blocks acceptance. The default thresholds are 85 for a file and 90 for a milestone or project; passing a threshold only makes it eligible. Project review covers integration and requirements, never an average of file scores.
4. Record the decision and its evidence under `.qc/reports/` in a trusted **external evidence root** by default, not inside the candidate workspace. Follow [scoring.md](references/scoring.md) and the [report schema](schemas/report.schema.json). Explain any unresolved findings, limitations, and next action. Do not insert QC scores in Rust source. If persistence is unavailable, report the assessment as unsaved and do not claim a durable checkpoint.
5. Return defects to the implementer or user; each repair is a new candidate requiring fresh applicable verification and QC. Do not spawn agents, commit, push, or publish without authorization. Do not treat a check or review as complete simply because its skill was invoked.

## Lifecycle

Use [checkpoints.md](references/checkpoints.md) when a checkpoint, pause, or resume is requested or an agreed milestone or work limit calls for one. The states are `IMPLEMENTING`, `VERIFYING`, `QC_REVIEW`, `REPAIR`, `ACCEPTED_MILESTONE`, `AWAITING_PUSH_APPROVAL`, `AWAITING_PUSH_CONFIRMATION`, `PAUSED`, `READY_FOR_HUMAN_TESTING`, `AWAITING_HUMAN_TEST`, and `COMPLETE`. An accepted milestone may continue unless a checkpoint was requested or configured. Preserve state and explicitly identify the hold and next authorized action. Never proceed past a required pause or fabricate human approval.

Use [human-testing.md](references/human-testing.md) for test handoff and completion. Automated readiness is not human acceptance. Human test results and explicit final sign-off are required for `COMPLETE`.

VERS v0.1's bundled harness deliberately reports overall `BLOCKED` while independent review coverage is incomplete; it cannot be treated as `PASS`, including when individual checks pass. An overall VERS pass is mandatory for file and integrated acceptance; hold those gates until genuine qualifying evidence exists. See VERS's own integration and report documentation for its candidate identity and isolation limits.
