# NetPulse Product Requirements Document

## 1. Product summary

NetPulse is a standalone macOS menu bar app that helps a user understand whether poor connectivity is caused by the local Wi-Fi link, the router/gateway, DNS, HTTPS reachability, or wider Internet latency/loss.

The product is intentionally local-first. It runs on the user's Mac, stores data locally, and does not depend on Hermes, Telegram, a cloud service, or an LLM backend.

## 2. Problem statement

When home or office Internet feels slow, the user typically has to manually check several disconnected signals:

- Is Wi-Fi signal weak?
- Is the Mac connected to the expected access point?
- Is the router/gateway reachable?
- Is WAN latency or packet loss high?
- Is DNS slow?
- Is HTTPS reachability degraded?
- Is the issue intermittent and hard to reproduce?

Existing tooling is either too low-level, too manual, too ISP/router-specific, or not optimized for quick menu-bar diagnosis.

## 3. Target users

### Primary user

A technically capable Mac user who wants quick local-network diagnosis without opening Terminal.

### Secondary users

- Engineering managers and developers working from home.
- People who need evidence before blaming Wi-Fi, ISP, VPN, or a remote service.
- Users who want lightweight network history without full observability infrastructure.

## 4. Goals

NetPulse should:

1. Show current network health from the macOS menu bar.
2. Distinguish local Wi-Fi quality from gateway reachability and WAN reachability.
3. Explain *why* health is degraded using concrete measurements.
4. Keep background checks lightweight and privacy-preserving.
5. Store recent samples locally for short-term troubleshooting.
6. Allow the user to copy a sanitized diagnostic summary.
7. Be simple enough to trust and maintain.

## 5. Non-goals

NetPulse should not:

- Depend on Hermes or any assistant backend.
- Require a cloud account.
- Run Internet speed tests by default.
- Monitor every device on the LAN.
- Replace router/AP observability.
- Capture packet contents.
- Store browsing history, DNS query names, or remote domains beyond fixed probe targets.
- Require root/sudo for the default v1 experience.
- Become a general-purpose network scanner.

## 6. User stories

### Menu-bar health

As a user, I want to see a simple menu-bar health indicator so that I know whether my network is currently healthy without opening a dashboard.

Acceptance:

- The menu-bar icon/state reflects the latest sample.
- States include at least `healthy`, `degraded`, `bad`, and `unknown`.
- Unknown is used when data is incomplete rather than pretending confidence.

### Explain degradation

As a user, I want NetPulse to tell me why it thinks the network is degraded so that I can decide what to do next.

Acceptance:

- The dashboard lists concrete reasons, e.g. high gateway latency, WAN packet loss, weak RSSI, DNS timeout.
- Reasons include measured values and thresholds where possible.

### Wi-Fi diagnosis

As a user, I want to see Wi-Fi signal and connection details so that I can identify local wireless problems.

Acceptance:

- Shows interface name, SSID when available, RSSI, noise, channel, tx rate, and connection status when macOS exposes them without sudo.
- Privacy-sensitive values are hidden or redacted in copied diagnostics by default.

### Internet reachability

As a user, I want NetPulse to probe gateway and WAN targets so that I can distinguish router issues from Internet issues.

Acceptance:

- Probes gateway derived from the default route.
- Probes at least one public stable target such as `1.1.1.1`.
- Calculates latency, loss percentage, and jitter for ping-like checks.
- Uses short timeouts so the app does not hang.

### DNS and HTTPS checks

As a user, I want DNS and HTTPS timing so that I can detect when ping works but real Internet usage is degraded.

Acceptance:

- Measures DNS lookup latency against a fixed harmless lookup target or resolver strategy.
- Measures HTTPS reachability/latency against a fixed lightweight endpoint.
- Reports timeouts and failures as reasons.

### History

As a user, I want recent samples so that intermittent problems are visible after they happen.

Acceptance:

- Stores timestamped samples locally.
- Shows recent health trend in the UI.
- Bounds storage size and avoids unbounded growth.

### Copy diagnostics

As a user, I want to copy a sanitized diagnostic summary so that I can share it with an assistant, ISP, colleague, or future bug report.

Acceptance:

- Redacts MAC addresses/BSSIDs, private IPs, hostnames, usernames, and secrets.
- Includes app version, macOS version, timestamp, health reasons, probe metrics, and relevant Wi-Fi fields.
- Makes clear what was redacted.

## 7. Functional requirements

### FR1: Sampling

- Collect one network sample on demand.
- Eventually collect samples periodically in the background.
- Avoid overlapping sample runs.
- Apply per-probe timeouts.

### FR2: macOS data collection

- Discover default route gateway and interface.
- Collect Wi-Fi details via macOS APIs or safe command fallbacks.
- Current Phase 1 command choices:
  - `route -n get default` for gateway/interface/MTU.
  - `system_profiler SPAirPortDataType -json` for Wi-Fi details.
- Do not rely on `wdutil info` for v1 because it requires sudo.
- Do not rely on the legacy `airport` binary because it may be missing on current macOS.

### FR3: Active probes

- Gateway probe.
- WAN probe to fixed public endpoint(s).
- DNS timing.
- HTTPS timing.
- All probes should produce structured successes/failures, not only strings.

### FR4: Health classification

- Classify health from measurements.
- Include reasons with measurements and thresholds.
- Use conservative `unknown` when required inputs are missing.

Initial threshold candidates:

- Healthy:
  - WAN packet loss: `0%`
  - Cloudflare avg latency: `< 50 ms`
  - Gateway avg latency: `< 10 ms`
  - RSSI: `>= -65 dBm`
- Degraded:
  - WAN packet loss: `> 0% and <= 5%`
  - Cloudflare avg latency: `50-150 ms`
  - Gateway avg latency: `10-30 ms`
  - RSSI: `-66 to -75 dBm`
- Bad:
  - WAN packet loss: `> 5%`
  - Cloudflare avg latency: `> 150 ms`
  - Gateway avg latency: `> 30 ms`
  - RSSI: `< -75 dBm`

These thresholds are provisional and must be tuned using real samples.

### FR5: UI

- Menu-bar status item.
- Popover dashboard.
- Manual refresh.
- Current health card.
- Wi-Fi card.
- Internet probes card.
- Recent history/trend card.
- Copy diagnostics action.
- Settings later if needed.

### FR6: Persistence

- Store recent samples locally under `~/Library/Application Support/NetPulse/`.
- Prefer JSONL for simple append/read/debug workflows.
- Bound retention by sample count and/or age.

### FR7: Privacy and safety

- No secrets or raw private network identifiers in committed fixtures.
- No packet capture.
- No command requiring sudo in the default v1 path.
- Copied diagnostics must redact sensitive local identifiers.

## 8. Non-functional requirements

### Performance

- Manual sample should normally complete within a few seconds.
- Background sampling must not noticeably affect battery or CPU.
- `system_profiler` runtime must be measured; if too slow for polling, use it sparingly or replace with a faster API.

### Reliability

- Partial samples are acceptable and should produce `unknown` or degraded reasons, not crashes.
- Missing commands or changed macOS output should produce structured errors.

### Maintainability

- Core parsing and health classification must be pure and well-tested.
- Shell/process execution must remain behind an injectable boundary.
- UI should consume typed backend commands rather than scraping strings.

### Portability

- Product target is macOS.
- Development and tests should still run on Linux where practical.
- Linux should not fake macOS live command output.

## 9. Data model v1 draft

```json
{
  "timestamp": "2026-05-28T12:00:00Z",
  "internet": {
    "gateway": {
      "host": "192.168.1.1",
      "avg_ms": 2.1,
      "loss_pct": 0,
      "jitter_ms": 0.4
    },
    "cloudflare": {
      "host": "1.1.1.1",
      "avg_ms": 8.5,
      "loss_pct": 0,
      "jitter_ms": 1.2
    },
    "dns_ms": 14,
    "https_ms": 52
  },
  "wifi": {
    "interface_name": "en0",
    "ssid": "ExampleWiFi",
    "bssid": "aa:bb:cc:dd:ee:ff",
    "rssi_dbm": -51,
    "noise_dbm": -92,
    "channel": "6",
    "tx_rate_mbps": 866,
    "quality": "excellent"
  },
  "health": {
    "status": "healthy",
    "reasons": []
  }
}
```

Implementation may evolve this shape, but any changes should be deliberate and documented.

## 10. Milestones

### M0: Repository and process foundation

Status: mostly complete.

- Repo initialized.
- Rust crate exists.
- Baseline CI exists.
- Agentic development operating model exists.
- Task-card workflow exists.

### M1: Data spike / CLI probe

Status: in progress.

Goal: prove reliable local data collection before UI work.

Deliverables:

- macOS command discovery.
- Pure parsers for route and Wi-Fi output.
- Injected command runner collector boundary.
- Real command runner.
- CLI wired to live collection on macOS.
- Active probes for gateway/WAN/DNS/HTTPS.
- Health classification.
- Sanitized sample outputs.

### M2: Tauri menu-bar shell

Goal: establish app shell and UI/backend boundary.

Deliverables:

- Tauri 2 project structure.
- Menu-bar status item.
- Popover dashboard shell.
- Backend command for current sample.
- Manual refresh.

### M3: Useful v1 dashboard

Goal: make the app useful for day-to-day diagnosis.

Deliverables:

- Health summary.
- Wi-Fi details.
- Internet probe details.
- Reasons/explanations.
- Recent sample display.
- Copy diagnostics.

### M4: Persistence and polish

Goal: stable local utility.

Deliverables:

- JSONL local sample store.
- Retention policy.
- Settings for polling interval and privacy display.
- Error states.
- Packaging/signing investigation.

### M5: Beta

Goal: daily use on Johan's Mac.

Deliverables:

- Installable app bundle.
- Stable background sampling.
- Known limitations documented.
- Feedback loop for threshold tuning.

## 11. Open decisions

- How often should background sampling run by default?
- Is `system_profiler` fast enough for background polling, or only refresh/manual sampling?
- Should SSID be visible in the UI by default or hidden behind a privacy toggle?
- Which HTTPS endpoint should be used for timing?
- Should WAN probe use only `1.1.1.1`, or multiple targets?
- How much history should v1 retain?
- What is the exact menu-bar icon language for each status?

## 12. Development principle

NetPulse should be built from real data outward:

1. Capture and parse real macOS output.
2. Normalize it into a stable sample model.
3. Classify health with transparent reasons.
4. Only then build UI around trustworthy data.

Do not build a beautiful dashboard over fake or unverified signals.
