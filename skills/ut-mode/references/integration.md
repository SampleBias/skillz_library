# Integration and handoffs

## Install for Codex

Install `ut-mode`, `patrs`, `amark`, `vers`, and `qc` as **separate** skill directories. Install `packet` too if prompt work is wanted. The orchestrator discovers their installed instructions; it does not bundle copies or install dependencies automatically. The same skills may be invoked separately as `$patrs`, `$amark`, `$vers`, `$qc`, and `$packet`. Start a new Codex session or reload the host after installation so its catalog can discover them. `$ut-mode` starts the coordinated Rust workflow. UT_MODE is the human-facing mode name; Codex's skill-creator validator requires hyphenated names.

After these files have been published to the repository, one installer command can select multiple paths:

```text
$skill-installer Install skills/ut-mode, skills/patrs, skills/amark, skills/vers, skills/qc, and skills/packet from https://github.com/SampleBias/skillz_library
```

Alternatively, from a trusted local checkout, copy each complete skill directory into the Codex skills directory according to the host's installation rules. Do not overwrite existing installed skills without checking their contents and the user's authorization. Merely copying `ut-mode` does not install the other skills. The VERS executable has its own local toolchain and dependency requirements; consult its installed integration reference.

## Handoff record

Carry forward the user goal, requirements and acceptance criteria; candidate path, branch, revision, dirty/untracked status and relevant content identities; scope, toolchain, targets/features and external inputs; actual checks, results and evidence locations; findings and residual risks; QC decision/state; and the next action. Record the verifier's identity and whether it was independent of implementation. Candidate hashes and git revisions are evidence identifiers, not guarantees that dependencies, tools or external systems are unchanged. Use QC's own report and checkpoint format when that skill calls for persistence; do not write internal orchestration status into source files.

For a confirmed defect, include its reproduction and affected contract when handing it back. After repair, re-identify the candidate and rerun the checks and reviews affected by the change. If a gate is blocked by a missing tool, access, independence, or inadequate evidence, preserve that status and specify what would unblock it. Do not repeatedly rerun an unchanged blocked check as a repair loop. Continue other independent in-scope tasks if they do not require acceptance of the blocked candidate.

## VERS and QC trust boundaries

VERS v0.2 can incorporate independent reviews using the two-stage workflow in its `references/review-evidence.md`. Its `--trust-reviews` flag records an operator trust decision; reviewer labels and artifact hashes do not establish actual reviewer identity, independence, or review quality. Only use it after provenance is independently verified. Missing or invalid evidence remains BLOCKED, and strict mode cannot PASS because its extra coverage is unimplemented. QC requires a qualifying overall VERS PASS and independently assesses the evidence. Passing checks do not establish software correctness. The harness does not sandbox candidate code or enforce reviewer separation; inspect and isolate execution. QC remains a manual specification, not an enforcing checkpoint controller.
