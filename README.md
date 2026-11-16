# Skillz Library

A collection of reusable agent skills. Each skill lives in its own folder under `skills/` and contains a `SKILL.md` file.

## Skills

| Skill | Purpose |
| --- | --- |
| [PACKET](skills/packet/SKILL.md) | Create, improve, or review prompts using Persona, Action, Context, Keywords, Examples, and Target output or Tone. |
| [PATRS](skills/patrs/SKILL.md) | Develop idiomatic Rust with practical patterns and minimal abstractions. |

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
