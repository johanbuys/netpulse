# Agentic Development Operating Model

> NetPulse is both a product and an experiment in disciplined AI-assisted software development.

## Goal

Use lower-cost/open-weight coding agents for implementation work while Q acts as architect, orchestrator, reviewer, and quality gate. The objective is to reduce premium-model token use without accepting AI slop.

## Roles

### Q: Orchestrator / SOTA Reviewer

Q owns:

- Product direction and scope control.
- Architecture decisions.
- Task decomposition.
- Acceptance criteria.
- Final code review.
- Test and validation strategy.
- GitHub App/API operations.
- Merge readiness decisions.

Q should spend premium tokens on judgment, not rote implementation.

### Worker agents: Implementation candidates

Worker agents, initially via `pi` using OpenCode Go models, own:

- Narrow implementation tasks.
- Draft code changes.
- Test additions.
- Local refactors within explicit boundaries.
- Producing concise implementation notes.

Workers do not own architecture, scope, final acceptance, or merge decisions.

## Available worker backend

This Hermes host currently has `pi` available. `opencode` CLI itself is not installed, but `pi` exposes OpenCode Go models through the `opencode-go` provider.

Verified models include:

- `opencode-go/deepseek-v4-flash`
- `opencode-go/deepseek-v4-pro`
- `opencode-go/glm-5.1`
- `opencode-go/kimi-k2.6`
- `opencode-go/minimax-m2.7`
- `opencode-go/qwen3.7-max`

## Core workflow

1. Q writes a precise implementation plan before coding.
2. Q creates a branch/worktree for each worker task.
3. Worker receives a small task with:
   - explicit files allowed to change
   - acceptance criteria
   - required tests
   - forbidden scope creep
4. Worker implements and reports changed files + test results.
5. Q reviews diff for spec compliance.
6. Q reviews diff for code quality.
7. Q runs verification locally.
8. Only Q commits/pushes/opens PRs unless explicitly delegated.

## Anti-slop gates

### Gate 1: Scope gate

Before worker execution:

- Is the task small enough?
- Are file boundaries clear?
- Are acceptance criteria testable?
- Is there a rollback path?

### Gate 2: Spec compliance gate

After worker execution, check:

- Did it implement exactly the requested behavior?
- Did it skip any requirement?
- Did it add unrelated features?
- Did it silently change architecture?

### Gate 3: Quality gate

After spec compliance passes, check:

- Is code simple and idiomatic?
- Are types useful and accurate?
- Are errors handled deliberately?
- Are tests meaningful, not superficial?
- Are names clear?
- Is there duplication or dead code?

### Gate 4: Verification gate

Before merge/push:

- Formatting passes.
- Lint/typecheck passes.
- Tests pass.
- Manual smoke path is documented.
- Diff is small enough to review.

## Worker prompt template

```text
You are an implementation worker for NetPulse.

Task: <one narrow task>

You may modify only:
- <file/path>
- <file/path>

Acceptance criteria:
- <criterion>
- <criterion>

Required verification:
- <command>
- <expected result>

Rules:
- Do not change architecture.
- Do not add dependencies unless explicitly allowed.
- Do not broaden scope.
- Prefer small, boring, testable code.
- If blocked, stop and report the blocker.
- At the end, report changed files and test commands run.
```

## Initial model allocation hypothesis

- Planning/architecture: Q only.
- Simple file scaffolding: `opencode-go/deepseek-v4-flash` or `opencode-go/qwen3.5-plus`.
- Moderate implementation: `opencode-go/kimi-k2.6`, `opencode-go/qwen3.7-max`, or `opencode-go/minimax-m2.7`.
- Review/merge decisions: Q only.

This is experimental and should be revised based on observed output quality.

## Metrics to track

For each worker task:

- Model used.
- Prompt size/context provided.
- Files changed.
- Tests added.
- Tests passed/failed.
- Q review findings.
- Number of correction loops.
- Whether output was accepted, partially accepted, or discarded.

## Principle

AI-generated code is a draft until it passes human-grade engineering gates. The process should make bad output cheap to reject and good output easy to integrate.
