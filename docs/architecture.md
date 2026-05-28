# Architecture Notes

## Product stance

NetPulse is a standalone local macOS app. Hermes is only used as a development collaborator; the shipped app must not depend on Hermes or any Hermes-managed files.

## Proposed architecture

```text
NetPulse.app
  ├─ menu bar status item
  ├─ popover dashboard
  ├─ React/TypeScript frontend
  ├─ Rust/Tauri backend commands
  │   ├─ collect_current_sample()
  │   ├─ get_recent_samples()
  │   ├─ copy_diagnostics()
  │   └─ open_settings()
  └─ local storage
      └─ ~/Library/Application Support/NetPulse/samples.jsonl
```

## Data flow

1. Backend collector runs probes on demand and on a timer.
2. Collector normalizes command/API output into `NetworkSample`.
3. Sample is appended to local JSONL storage.
4. Frontend asks Tauri backend for latest and recent samples.
5. Tray status reflects latest health state.

## Rust backend responsibilities

- Discover active default interface and gateway.
- Collect Wi-Fi stats from macOS APIs/commands.
- Run ping/DNS/HTTPS probes with timeouts.
- Classify health.
- Store/retrieve samples.
- Avoid blocking the UI thread.

## React frontend responsibilities

- Render dashboard cards.
- Show current health and reasons.
- Display recent history/sparklines.
- Provide manual refresh and copy diagnostics actions.
- Keep the UI simple enough for v1.

## Open questions

- Which macOS version is Johan running?
- Is the active Wi-Fi interface always `en0`, or should discovery always infer it?
- Which Wi-Fi command still provides RSSI/noise/BSSID without sudo on that Mac?
- What app name should we commit to: NetPulse, SignalBar, LagWatch, or another?
- Should the first remote GitHub repo be public or private?
