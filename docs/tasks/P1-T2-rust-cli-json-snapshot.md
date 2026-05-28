# NetPulse Task Card

## Task ID

`P1-T2`

## Title

Create a minimal Rust CLI that emits a network snapshot JSON fixture.

## Worker model

`q/manual-or-agent`

## Objective

Create the first Rust CLI skeleton for `netpulse-probe --json`, using a deterministic placeholder snapshot until real macOS command parsing is captured.

## Allowed files

The worker may only create or modify these files:

- `Cargo.toml`
- `Cargo.lock`
- `src/main.rs`
- `src/lib.rs`
- `tests/netpulse_probe_cli.rs`
- `docs/spikes/sample-netpulse-snapshot.json`

If another file seems necessary, stop and report why.

## Context

This is a deliberately small CLI-first step, not the full Tauri app. It creates a stable data contract and test harness before adding macOS-specific collection.

The target command from `docs/phase-1-data-spike.md` is:

```bash
netpulse-probe --json
```

The first implementation may return deterministic sample data rather than querying the host. Real macOS command parsing should come after `P1-T1` captures command outputs.

## Acceptance criteria

- [ ] `cargo test` passes.
- [ ] `cargo run -- --json` prints valid JSON to stdout.
- [ ] The JSON includes top-level keys: `timestamp`, `internet`, `wifi`, and `health`.
- [ ] `health.status` is one of `healthy`, `degraded`, `bad`, or `unknown`.
- [ ] The implementation is clear that sample data is placeholder data.
- [ ] No shell commands are executed in this task.
- [ ] No privileged operations are required.

## Required verification

Follow TDD:

1. Write a failing integration test that runs the CLI with `--json` and asserts the output is valid JSON with the required top-level keys.
2. Run the test and verify it fails for the expected reason.
3. Implement the minimal Rust CLI to make the test pass.
4. Run the full test suite.

Commands:

```bash
cargo test
cargo run -- --json
```

Expected:

```text
cargo test exits 0.
cargo run -- --json emits valid JSON with timestamp, internet, wifi, and health keys.
```

## Suggested implementation shape

Use a tiny Rust crate:

- `src/lib.rs`: data structs and `sample_snapshot()` function.
- `src/main.rs`: parse `--json`; print pretty JSON; otherwise print usage.
- `tests/netpulse_probe_cli.rs`: integration test for command output.

Suggested dependencies:

- `serde`
- `serde_json`
- `chrono` with serde support, or a static timestamp for the first deterministic test.

Keep this small. Do not add Tauri yet.

## Max diff budget

Target: `260` changed lines.

If the task requires more, stop and explain before continuing.

## Stop conditions

Stop and report instead of guessing if:

- Rust toolchain is missing.
- Tests cannot be made meaningful within scope.
- The solution requires shelling out to macOS commands.
- The solution requires adding Tauri or frontend files.

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
