---
name: packet
description: Create, improve, or review prompts and tasks using the PACKET framework. Use when the user asks to apply PACKET.
---

# PACKET

Help the user make a request clear and actionable using the six PACKET elements. Preserve their intent, stated constraints, and preferred level of detail. Apply PACKET to the user's subject; do not treat the framework itself as the subject unless requested.

## Elements

- **P — Persona:** The role or perspective the agent should adopt, when useful.
- **A — Action:** The specific work to perform, expressed as an action.
- **C — Context:** The background, audience, purpose, available materials, and relevant constraints.
- **K — Keywords:** Important terms, concepts, terminology, or phrases to include or consider.
- **E — Examples:** Relevant examples supplied by the user, found on the web when appropriate, or clearly labeled as manufactured. Explain what each example illustrates.
- **T — Target output or Tone:** The desired deliverable, format, length, audience, tone, or other success criteria.

## Workflow

1. Identify the user's goal and extract any PACKET elements already provided. Retain their original requirements.
2. Fill in reasonable, helpful details when the request supports them; mark material assumptions. Do not invent facts, sources, or requirements.
3. Ask concise questions only when missing information would materially change the outcome and cannot reasonably be inferred. Do not insist on every element when it adds no value.
4. Provide a usable PACKET-structured prompt or review, as requested. For a review, point out consequential gaps and suggest concrete improvements while retaining the user's intent.
5. Include examples when useful. If web examples are requested, research them when browsing is available, use reliable sources, and cite direct links. If browsing is unavailable, say so; do not present manufactured examples as web-sourced. Clearly label invented examples, which are illustrations rather than evidence.
6. If the user also asks you to carry out the task, use the resulting framework to perform the authorized work. If they ask only for a prompt or framework, provide that deliverable for review. Never infer authorization for external actions from a request to draft a prompt.

## Suggested response format

Adapt this format to the request; omit or mark elements that are not yet known.

**P — Persona:** [Role or perspective]

**A — Action:** [Specific task]

**C — Context:** [Background, audience, and constraints]

**K — Keywords:** [Relevant terms or phrases]

**E — Examples:** [Provided, sourced, or clearly labeled manufactured examples]

**T — Target output or Tone:** [Format, length, tone, and success criteria]

## Example

For “Use PACKET to make a prompt that teaches first-time budgeters,” a concise result could be:

**P — Persona:** A patient financial educator.

**A — Action:** Explain how to make a first monthly budget.

**C — Context:** The audience is adults with little budgeting experience. Use plain language and avoid assuming their income or location.

**K — Keywords:** Income, fixed expenses, variable expenses, savings, emergency fund.

**E — Examples:** Include a clearly labeled manufactured budget using an illustrative monthly income of $2,000.

**T — Target output or Tone:** A friendly explanation with a simple table, in fewer than 500 words.
