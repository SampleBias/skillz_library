# Host installation

Clone this repository from a trusted source and run the examples below from the checkout. They copy complete directories, including references and scripts. They refuse to replace an existing destination; review installed versions before updating them. Install all five Rust skills (`ut-mode`, `patrs`, `amark`, `vers`, `qc`) for the coordinated workflow; `packet` is optional. To install one instead, replace the list in the loop with its name.

## Codex

Ask Codex to use its installer:

```text
$skill-installer Install skills/ut-mode, skills/patrs, skills/amark, skills/vers, skills/qc, and skills/packet from https://github.com/SampleBias/skillz_library
```

Start a new Codex session so its skill catalog is rebuilt, then invoke `$ut-mode` or another installed skill. See the [UT_MODE handoff guide](../skills/ut-mode/references/integration.md).

## Claude Code

```bash
mkdir -p "$HOME/.claude/skills"
for skill in ut-mode patrs amark vers qc packet; do
  test ! -e "$HOME/.claude/skills/$skill" || { echo "Already exists: $skill" >&2; continue; }
  cp -R "skills/$skill" "$HOME/.claude/skills/$skill"
done
```

Start a new Claude Code session if the skill is not discovered; type `/ut-mode` in Claude Code. See [Claude Code skills](https://code.claude.com/docs/en/skills).

## Cursor

```bash
mkdir -p "$HOME/.cursor/skills"
for skill in ut-mode patrs amark vers qc packet; do
  test ! -e "$HOME/.cursor/skills/$skill" || { echo "Already exists: $skill" >&2; continue; }
  cp -R "skills/$skill" "$HOME/.cursor/skills/$skill"
done
```

Restart Cursor if needed, then look in Customize → Skills or type `/` in Agent chat and select the skill. For Cloud Agents, consult Cursor's skill sync instructions; local personal skills are not automatically available there. See [Cursor skills](https://cursor.com/docs/skills).

## Grok Build

```bash
mkdir -p "$HOME/.grok/skills"
for skill in ut-mode patrs amark vers qc packet; do
  test ! -e "$HOME/.grok/skills/$skill" || { echo "Already exists: $skill" >&2; continue; }
  cp -R "skills/$skill" "$HOME/.grok/skills/$skill"
done
```

Start a new Grok Build session if needed; use `/skills` to inspect the list or run `grok inspect` from the project. User-invocable skills appear as slash commands. These instructions apply to Grok Build, not the Grok chat website. See [Grok Build skills](https://docs.x.ai/build/features/skills-plugins-marketplaces).

Host discovery and permission rules can change. A skill is guidance, not permission to execute code or use independent reviewers. VERS needs a Rust toolchain, cached dependencies, a Git-root workspace and an appropriate isolation decision; see its [integration guide](../skills/vers/references/integration.md).
