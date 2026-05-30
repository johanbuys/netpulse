# NetPulse Task Card: P1-T4

Add a testable macOS collector boundary that combines existing pure parsers through an injected command runner.

Allowed files: `src/lib.rs`, `tests/macos_collector.rs`, this task card.

## Acceptance

- Define a command-runner abstraction for command output collection.
- Add a collector function that requests route and Wi-Fi profiler outputs through that abstraction.
- Combine parsed route + Wi-Fi output into `NetworkSnapshot`.
- Test with a fake runner only; do not add live `std::process::Command` execution yet.
- Preserve existing CLI placeholder behavior.
- Add no real SSIDs, MACs, private IPs, hostnames, usernames, or secrets.
- Keep diff at or below `240` changed lines.

## Verification

`cargo test --test macos_collector`; `cargo fmt --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all --locked`; `cargo build --locked`; `git diff --check`.
