# Phase 1 Data Spike

## Objective

Collect real Mac-local network data before committing to UI details. The output of this phase should be a small probe that emits a single JSON object representing current Internet and Wi-Fi health.

## Target command

```bash
netpulse-probe --json
```

Example shape:

```json
{
  "timestamp": "2026-05-28T12:00:00Z",
  "internet": {
    "gateway": { "host": "192.168.1.1", "avg_ms": 2.1, "loss_pct": 0, "jitter_ms": 0.4 },
    "cloudflare": { "host": "1.1.1.1", "avg_ms": 8.5, "loss_pct": 0, "jitter_ms": 1.2 },
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

## Mac commands to test first

### Interface discovery

```bash
networksetup -listallhardwareports
route -n get default
```

### Wi-Fi network name

```bash
networksetup -getairportnetwork en0
```

### Wi-Fi detailed stats fallbacks

Try in this order and record what works on Johan's macOS version:

```bash
/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I
```

```bash
system_profiler SPAirPortDataType -json
```

```bash
wdutil info
```

Note: some commands may require elevated permissions or may redact details on newer macOS versions. The spike should capture which fields are available without sudo.

### Internet probes

```bash
ping -c 5 -i 0.2 1.1.1.1
ping -c 5 -i 0.2 8.8.8.8
```

Gateway target should come from `route -n get default`, not a hard-coded IP.

DNS and HTTPS checks can be implemented directly in Rust after the shell-command spike.

## Phase 1 deliverables

1. A document listing which macOS commands work and which fields they return.
2. A Rust CLI prototype that emits one JSON sample.
3. A small set of sample JSON outputs from good and degraded network moments.
4. Health classification thresholds for v1.

## Initial health thresholds

- Healthy:
  - WAN packet loss: 0%
  - Cloudflare avg latency: < 50 ms
  - gateway avg latency: < 10 ms
  - RSSI: >= -65 dBm
- Degraded:
  - WAN packet loss: > 0% and <= 5%
  - Cloudflare avg latency: 50-150 ms
  - gateway avg latency: 10-30 ms
  - RSSI: -66 to -75 dBm
- Bad:
  - WAN packet loss: > 5%
  - Cloudflare avg latency: > 150 ms
  - gateway avg latency: > 30 ms
  - RSSI: < -75 dBm

These are starting points and should be tuned from real samples.
