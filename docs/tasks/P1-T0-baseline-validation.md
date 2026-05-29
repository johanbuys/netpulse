# NetPulse Task Card

## Task ID

`P1-T0`

## Title

Add baseline Rust CI and local build-artifact hygiene.

## Worker model

`q/manual`

## Objective

Add the smallest validation safety net before more implementation work lands: a GitHub Actions workflow that runs Rust formatting, linting, tests, and build checks on every PR and push to `main`.

## Allowed files

The worker may only create or modify these files:

- `.github/workflows/ci.yml`
- `.gitignore`
- `docs/tasks/P1-T0-baseline-validation.md`

If another file seems necessary, stop and report why.

## Context

PRs `P1-T1` and `P1-T2` established the first macOS command discovery doc and Rust CLI JSON skeleton. Before adding real parsers, NetPulse needs an automated validation gate so code does not drift from local-only checks.

## Acceptance criteria

- [ ] GitHub Actions runs on pull requests.
- [ ] GitHub Actions runs on pushes to `main`.
- [ ] CI installs stable Rust with `rustfmt` and `clippy`.
- [ ] CI runs `cargo fmt --check`.
- [ ] CI runs `cargo clippy --all-targets --all-features -- -D warnings`.
- [ ] CI runs `cargo test --all --locked`.
- [ ] CI runs `cargo build --locked`.
- [ ] Local `target/` build outputs are ignored.
- [ ] No application behavior changes are included.

## Required verification

Commands:

```bash
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo test --all --locked
cargo build --locked
git diff --check
```

Expected:

```text
All commands exit 0.
```

## Max diff budget

Target: `120` changed lines.

If the task requires more, stop and explain before continuing.

## Stop conditions

Stop and report instead of guessing if:

- The workflow needs secrets.
- The workflow needs deployment permissions.
- The workflow requires macOS runners.
- The solution requires changing Rust source code.

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
