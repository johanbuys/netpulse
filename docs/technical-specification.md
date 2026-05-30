# NetPulse Technical Specification

## 1. Architecture overview

NetPulse has two layers:

1. **Probe/core layer**: Rust library and CLI that collects, parses, normalizes, classifies, and stores network samples.
2. **Desktop app layer**: Tauri shell with React/TypeScript UI that calls Rust backend commands and renders status/history.

```text
NetPulse.app
  ├─ macOS menu bar item
  ├─ React/TypeScript popover dashboard
  ├─ Tauri Rust backend
  │   ├─ command: collect_current_sample()
  │   ├─ command: get_recent_samples()
  │   ├─ command: copy_diagnostics()
  │   └─ command: open_settings()
  └─ Rust core crate
      ├─ command execution boundary
      ├─ macOS collectors
      ├─ pure parsers
      ├─ active probes
      ├─ health classification
      └─ local JSONL store
```

The Rust core should remain independently testable without a Tauri runtime.

## 2. Current implementation state

Current crate:

- Package: `netpulse-probe`
- Existing binary: `netpulse-probe`
- Current command: `netpulse-probe --json`
- Current behavior: deterministic placeholder JSON.

Current implemented concepts:

- `NetworkSnapshot`
- `InternetSnapshot`
- `ProbeResult`
- `DefaultRoute`
- `WifiSnapshot`
- `HealthSnapshot`
- `CommandRunner` trait
- `parse_default_route(...)`
- `parse_system_profiler_airport(...)`
- `collect_macos_snapshot(...)` using injected command runner
- Tests for CLI shape, macOS parsers, and collector boundary

## 3. Core module boundaries

The code should evolve toward these modules as complexity increases:

```text
src/
  lib.rs                       # temporary facade; split when needed
  model.rs                     # NetworkSnapshot and related data types
  macos/
    mod.rs
    commands.rs                # live std::process::Command runner
    route.rs                   # route parser
    wifi_system_profiler.rs    # system_profiler parser
    collector.rs               # combines command outputs into sample
  probes/
    mod.rs
    ping.rs
    dns.rs
    https.rs
  health.rs                    # classification and reason generation
  diagnostics.rs               # sanitized text/JSON diagnostic output
  storage.rs                   # JSONL append/read/retention
```

Do not split files prematurely. Split when a task needs it and tests make the boundary useful.

## 4. Command execution boundary

All process execution must stay behind an injectable abstraction.

Current shape:

```rust
pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String, String>;
}
```

Expected live runner behavior:

- Uses `std::process::Command`.
- Applies timeout or is wrapped by timeout-capable caller.
- Captures stdout/stderr.
- Returns structured-enough errors for the UI to explain failures.
- Does not invoke shell interpolation for fixed commands.
- Does not accept untrusted user input as program/args in v1.

Potential future improvement:

```rust
pub struct CommandOutput {
    pub stdout: String,
    pub stderr: String,
    pub status_code: Option<i32>,
    pub duration_ms: u64,
}
```

Use the simple trait until richer error reporting is needed.

## 5. macOS collection

### 5.1 Default route

Command:

```bash
route -n get default
```

Fields:

- `gateway`
- `interface`
- optional `mtu`

Parser requirements:

- Ignore irrelevant lines.
- Require gateway and interface.
- Treat missing gateway/interface as errors.
- Parse MTU if present and numeric.

### 5.2 Wi-Fi details

Command:

```bash
system_profiler SPAirPortDataType -json
```

Fields to extract when present:

- interface name
- current network name / SSID
- BSSID
- RSSI
- noise
- channel
- tx rate
- connected status

Parser requirements:

- Accept sanitized fixtures.
- Avoid hard-coding private/local values.
- Return `unknown` quality when connection status is missing or not connected.
- Surface parser errors clearly enough for diagnostics.

Known alternatives:

- Legacy `airport -I`: missing on tested current macOS; do not rely on for v1.
- `networksetup -getairportnetwork en0`: unreliable for current needs; not primary.
- `wdutil info`: sudo-only; exclude from v1 default path.

## 6. Active probes

Active probes should be implemented after live macOS command collection is wired.

### 6.1 Gateway probe

Input:

- Gateway host from default route.

Metrics:

- average latency
- packet loss percentage
- jitter
- timeout/failure reason

### 6.2 WAN probe

Initial target:

- `1.1.1.1`

Possible later targets:

- `8.8.8.8`
- configurable target

Do not add many background targets in v1; keep checks lightweight.

### 6.3 DNS timing

Options:

- Use Rust resolver timing for a fixed harmless hostname.
- Use system resolver behavior so result resembles real user experience.

Requirements:

- Do not log arbitrary user domains.
- Apply timeout.
- Report failure distinctly from slow success.

### 6.4 HTTPS timing

Options:

- Lightweight HEAD/GET to a stable endpoint.
- Consider `https://www.cloudflare.com/cdn-cgi/trace` or another stable low-payload endpoint.

Requirements:

- Apply timeout.
- Measure wall-clock duration.
- Avoid downloading large payloads.

## 7. Health classification

Health classification should be pure:

```rust
pub fn classify_health(sample: &NetworkSnapshot) -> HealthSnapshot
```

Rules:

- Return status and reasons.
- Reasons must be human-readable and include measurements.
- Missing data should produce `unknown` if it prevents confident classification.
- Multiple bad signals should all be reported, not hidden behind the first failure.

Suggested reason examples:

- `Gateway latency 42 ms exceeds bad threshold 30 ms`
- `WAN packet loss 2.0% exceeds healthy threshold 0%`
- `Wi-Fi RSSI -78 dBm is below bad threshold -75 dBm`
- `DNS probe timed out after 1000 ms`

## 8. Local storage

Initial storage format:

- JSON Lines file.
- Location: `~/Library/Application Support/NetPulse/samples.jsonl`.

Requirements:

- Append one JSON object per sample.
- Read most recent N samples.
- Retain bounded history.
- Handle corrupt lines gracefully by skipping/reporting.
- Never store unredacted copied diagnostics outside the user's local app data.

Potential retention defaults:

- Keep last 24 hours at default polling interval.
- Or keep last 1,000 samples.

Exact policy remains open.

## 9. Tauri backend commands

### `collect_current_sample()`

Returns latest collected sample.

Behavior:

- Runs one sample collection.
- Updates in-memory latest state.
- Appends to storage when storage exists.
- Returns structured errors/partial sample.

### `get_recent_samples(limit)`

Returns latest stored samples.

Behavior:

- Bounds `limit`.
- Reads from local storage.
- Returns newest-first or oldest-first consistently; document choice.

### `copy_diagnostics()`

Returns sanitized text suitable for clipboard.

Behavior:

- Includes latest sample summary.
- Redacts sensitive identifiers.
- Includes app version and macOS version if available.

### `open_settings()`

May be deferred. Settings can initially be static/defaulted.

## 10. React UI specification

### Menu bar states

- Healthy: green/normal icon.
- Degraded: yellow/warning icon.
- Bad: red/error icon.
- Unknown: gray/question icon.
- Sampling: transient spinner or subtle progress state.

Exact visual design can be decided during UI implementation.

### Popover layout v1

Recommended sections:

1. Header
   - Current status.
   - Last updated timestamp.
   - Refresh button.
2. Reasons
   - List of health reasons.
   - Empty state when healthy.
3. Wi-Fi
   - Interface, SSID display, RSSI, noise, channel, tx rate, quality.
4. Internet
   - Gateway probe.
   - WAN probe.
   - DNS.
   - HTTPS.
5. Recent trend
   - Small timeline/sparkline or list.
6. Actions
   - Copy diagnostics.
   - Open settings/about.

## 11. Privacy/redaction specification

Copied diagnostics and committed fixtures must redact:

- MAC addresses and BSSIDs.
- Private IPv4 addresses.
- Link-local addresses.
- Hostnames.
- Usernames.
- Email-like strings.
- Real SSIDs unless explicitly allowed by the user for local-only display.

In-app display may show local details because the app runs locally, but exported/copyable diagnostics should default to redacted.

## 12. Testing strategy

### Unit tests

- Pure parsers.
- Health classification.
- Redaction.
- Storage line parsing.

### Integration tests

- CLI JSON output shape.
- Collector with fake command runner.
- Storage append/read on temp directory.

### Platform tests

- Live macOS command runner should be manually tested on Johan's Mac before relying on it.
- Linux CI must not pretend to validate live macOS collection.

### Required checks before PR merge

```bash
cargo test --all --locked
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --locked
git diff --check
```

## 13. CI expectations

CI should run deterministic checks that do not require macOS live network access unless a macOS runner is explicitly added.

Minimum CI:

- format check
- clippy
- test
- build

Later CI:

- TypeScript checks
- frontend lint/test
- Tauri build smoke if feasible

## 14. Error handling principles

- Prefer typed errors as code grows.
- Avoid panics in collection path.
- Preserve partial measurements when one probe fails.
- UI should show useful error messages without overwhelming the user.

## 15. Handoff constraints for development agents

Workers must:

- Work from task cards.
- Modify only allowed files.
- Keep diffs small.
- Add tests for new pure behavior.
- Preserve privacy rules.
- Avoid broad architecture changes unless the task explicitly asks for them.
- Report verification commands and results.

Q/human reviewer owns:

- Product direction.
- Architecture changes.
- Merge readiness.
- Threshold tuning decisions.
- Whether UX is understandable.
