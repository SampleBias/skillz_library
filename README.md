# Skillz Library

A collection of reusable agent skills. Each skill lives in its own folder under `skills/` and contains a `SKILL.md` file.

## Skills

| Skill | Purpose |
| --- | --- |
| [AMARK](skills/amark/SKILL.md) | Keep Rust comments and documentation minimal, purposeful, and useful for maintenance. |
| [PACKET](skills/packet/SKILL.md) | Create, improve, or review prompts using Persona, Action, Context, Keywords, Examples, and Target output or Tone. |
| [PATRS](skills/patrs/SKILL.md) | Develop idiomatic Rust with practical patterns and minimal abstractions. |
| [QC](skills/qc/SKILL.md) | Govern Rust file and project acceptance, checkpoints, and human test handoff using verification evidence. |
| [VERS](skills/vers/SKILL.md) | Independently verify Rust projects with executable checks and evidence. |

## Use PACKET with Codex

Install the skill from this repository by asking Codex:

```text
$skill-installer Install the skill from https://github.com/SampleBias/skillz_library/tree/main/skills/packet
```

Start a new Codex session after installation, then try:

```text
$packet Help me create a prompt for a beginner-friendly lesson on budgeting.
```

See the [license](LICENSE) for terms.

## PATRS: develop Rust projects

[PATRS](skills/patrs/SKILL.md) guides Rust implementation and repairs using patterns suited to the task. Install it through a compatible skill host from `https://github.com/SampleBias/skillz_library/tree/main/skills/patrs`, reload the host, and invoke `$patrs`. Use `$vers` separately for independent verification when available.

## VERS: verify Rust projects

[VERS](skills/vers/SKILL.md) is an independent Rust verification skill. Its MVP harness records Cargo checks, runtime contracts, candidate identity and evidence; it deliberately blocks overall approval while independent safety, security, architecture and contract reviews remain incomplete.

After these files have been committed and pushed, install it through a compatible skill host from `https://github.com/SampleBias/skillz_library/tree/main/skills/vers`, then reload the host and invoke `$vers`. The verifier can also run directly using `skills/vers/scripts/verify.sh --root /path/to/project --execute`. Read the [integration guide](skills/vers/references/integration.md) before running project code; the harness does not sandbox it.

## PATRS: develop Rust projects

[PATRS](skills/patrs/SKILL.md) guides Rust implementation and repairs using patterns suited to the task. Install it through a compatible skill host from `https://github.com/SampleBias/skillz_library/tree/main/skills/patrs`, reload the host, and invoke `$patrs`. Use `$vers` separately for independent verification when available.

## AMARK: annotate Rust deliberately

[AMARK](skills/amark/SKILL.md) keeps Rust comments focused on safety, non-obvious decisions, necessary workarounds, and public API contracts. Install it through a compatible skill host from `https://github.com/SampleBias/skillz_library/tree/main/skills/amark` after the changes are published, reload the host, and invoke `$amark`. Use `$patrs` for development, `$amark` for annotation quality, and `$vers` for independent verification; repair findings and return the changed candidate for review.

## QC: govern Rust acceptance and checkpoints

[QC](skills/qc/SKILL.md) evaluates file and integrated project acceptance after PATRS, AMARK, and VERS. It records evidence-based scores and lifecycle decisions and holds when mandatory verification is incomplete. The [skill package](skills/qc/README.md) is a manual specification and starter configuration, not an executable checkpoint controller. After publication, install `skills/qc` through a compatible skill host, reload it, and invoke `$qc`.
