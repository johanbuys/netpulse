# NetPulse Task Card Template

Use one task card per worker-agent execution. Workers receive task cards, not whole phase specs.

## Task ID

`P<phase>-T<sequence>`

## Title

Short imperative title.

## Worker model

`opencode-go/<model>`

## Objective

One sentence describing the single outcome.

## Allowed files

The worker may only create or modify these files:

- `path/to/file`

If another file seems necessary, stop and report why.

## Context

Only the minimal context needed for this task:

- Relevant architecture notes.
- Existing types/functions to preserve.
- Fixture text or command output if needed.
- Prior task outputs if needed.

## Acceptance criteria

- [ ] Criterion 1.
- [ ] Criterion 2.
- [ ] Criterion 3.

## Required verification

Run these commands and report the result:

```bash
<command>
```

Expected:

```text
<expected output or condition>
```

## Max diff budget

Target: `<N>` changed lines.

If the task requires more, stop and explain before continuing.

## Stop conditions

Stop and report instead of guessing if:

- Required command/tool is missing.
- The current codebase differs from this task card.
- The solution requires changing files outside the allowed list.
- The task appears to require new dependencies.
- Tests cannot be made meaningful within scope.

## Worker output format

```text
Summary:
- <what changed>

Changed files:
- <file>

Verification:
- <command>: <pass/fail + notes>

Risks / follow-ups:
- <anything Q should inspect>
```
