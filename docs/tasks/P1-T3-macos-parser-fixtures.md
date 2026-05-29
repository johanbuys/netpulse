# NetPulse Task Card: P1-T3

Pure Rust parsers for sanitized macOS outputs:
`route -n get default` and `system_profiler SPAirPortDataType -json`.

Allowed files: `src/lib.rs`, `tests/macos_parsers.rs`, this task card.

## Acceptance

- Parse route gateway, interface, optional MTU.
- Parse requested Wi-Fi interface: SSID, RSSI, noise, channel, tx rate, connected status.
- Use sanitized fixtures only; add no real SSIDs, MACs, private IPs, hostnames, usernames, or secrets.
- Execute no shell commands; preserve existing CLI placeholder behavior.
- Keep diff at or below `260` changed lines.

## Verification

`cargo test --test macos_parsers`; `cargo fmt --check`; `cargo clippy --all-targets --all-features -- -D warnings`; `cargo test --all --locked`; `cargo build --locked`; `git diff --check`.
