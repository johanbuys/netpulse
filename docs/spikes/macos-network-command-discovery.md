# macOS Network Command Discovery

**Spike document for NetPulse Phase 1**
*Status: compiled from a real sanitized macOS capture (macOS 25.3.0, ARM64)*
*This host is Linux; all command outputs are from the captured macOS session, not run locally.*

---

## Environment

| Property | Value |
|---|---|
| Host OS (this CI) | Linux |
| Target OS | macOS 25.3.0 (Darwin 25.3.0, ARM64 T6020) |
| Capture date | 2026-05-29 |
| Shell | zsh |
| Primary Wi-Fi | en0 |
| Default gateway | `<redacted-gateway-ip>` via en0 |

---

## Commands

### 1. `networksetup -listallhardwareports`

**Status:** `works`

Lists every hardware port with its BSD device name and MAC address. No sudo required.

```
Hardware Port: Ethernet Adapter (en4)  Device: en4  Ethernet Address: <redacted-mac>
Hardware Port: Ethernet Adapter (en5)  Device: en5  Ethernet Address: <redacted-mac>
...
Hardware Port: Wi-Fi                   Device: en0  Ethernet Address: <redacted-mac>
...
VLAN Configurations
===================
```

**Fields:** `Hardware Port` (friendly name), `Device` (BSD interface name), `Ethernet Address` (MAC; redact before committing).

---

### 2. `route -n get default`

**Status:** `works`

Shows the IPv4 default route. No sudo required.

```
   route to: default
destination: default
    gateway: <redacted-gateway-ip>
  interface: en0
      flags: <UP,GATEWAY,DONE,STATIC,PRCLONING,GLOBAL>
       mtu: 1500
```

**Fields:** `gateway` (LAN ping target), `interface` (primary iface), `mtu`, `flags`.

---

### 3. `networksetup -getairportnetwork en0`

**Status:** `works` (but reports "not associated" — do not rely for v1)

No sudo. Observed: `You are not associated with an AirPort network.`

**Fields:** expected current SSID/network name; observed no usable SSID because the command reported not associated.

Despite `system_profiler` showing en0 connected, this reports no association on macOS 25.3.0. Possibly WPA3/6 GHz or deprecation. **Do not rely for v1.** Use `system_profiler SPAirPortDataType -json` (command 5) instead.

---

### 4. `airport -I` (legacy path)

**Status:** `missing`

Binary absent at `/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport` on macOS 25.3.0.

```
zsh: no such file or directory: .../Resources/airport
```

**Expected fields (older macOS):** `RSSI`, `noise`, `BSSID`, `SSID`, `channel`, `txRate`, `MCS`, `state` — all unavailable. **Do not use for v1.**

---

### 5. `system_profiler SPAirPortDataType -json`

**Status:** `works`

No sudo. Returns structured JSON with current connection and nearby networks.

**Key fields (en0):**

| JSON key | Value (redacted) | Notes |
|---|---|---|
| `_name` | `<redacted>` | SSID — redact |
| `spairport_network_channel` | `5 (6GHz, 160MHz)` | Channel, band, width |
| `spairport_network_country_code` | `CA` | Regulatory domain |
| `spairport_network_mcs` | `7` | MCS index |
| `spairport_network_phymode` | `802.11ax` | PHY mode |
| `spairport_network_rate` | `1441` | Tx rate (Mbps) |
| `spairport_security_mode` | `wpa3_personal` | Security protocol |
| `spairport_signal_noise` | `-48 dBm / -92 dBm` | RSSI / Noise (dBm) — key metric |
| `spairport_status_information` | `spairport_status_connected` | Connection state |
| `spairport_wireless_mac_address` | `<redacted-mac>` | Interface MAC — redact |

Nearby networks expose `_name` (SSID, redacted), channel, phymode, security, and optionally signal_noise. The `awdl0` interface appears alongside en0 and can be filtered out.

---

### 6. `wdutil info`

**Status:** `requires sudo`

Without sudo, prints only usage. All expected fields (RSSI, noise, BSSID, SSID, channel, txRate, PHY mode, security) require root. **Exclude from v1 — no sudo-only commands required.** `system_profiler SPAirPortDataType -json` (command 5) provides sufficient Wi-Fi data without elevated privileges.

---

### 7. `ping -c 5 -i 0.2 1.1.1.1`

**Status:** `works`

No sudo required. 5 packets, 0% loss, min/avg/max/stddev = 10.858/11.499/12.683/0.667 ms.

**Fields:** `packets transmitted`, `packets received`, `packet loss` (%), `min/avg/max/stddev` RTT (ms).

---

### 8. `ping -c 5 -i 0.2 8.8.8.8`

**Status:** `works`

No sudo required. 5 packets, 0% loss, min/avg/max/stddev = 10.153/10.797/11.906/0.591 ms.

**Fields:** `packets transmitted`, `packets received`, `packet loss` (%), `min/avg/max/stddev` RTT (ms).

---

## Summary

| # | Command | Status | Sudo? | Recommended for v1 |
|---|---|---|---|---|
| 1 | `networksetup -listallhardwareports` | ✅ works | No | Yes — interface enumeration |
| 2 | `route -n get default` | ✅ works | No | Yes — gateway + interface |
| 3 | `networksetup -getairportnetwork en0` | ✅ works (unreliable) | No | No — reports "not associated" |
| 4 | `airport -I` (legacy) | ❌ missing | N/A | No — binary absent on 25.3.0 |
| 5 | `system_profiler SPAirPortDataType -json` | ✅ works | No | Yes — primary Wi-Fi data source |
| 6 | `wdutil info` | ⚠️ requires sudo | Yes | No — avoid sudo requirement |
| 7 | `ping 1.1.1.1` | ✅ works | No | Yes — Internet health probe |
| 8 | `ping 8.8.8.8` | ✅ works | No | Yes — Internet health probe (redundant target) |

---

## Sudo-Only Exclusion

**No command requiring `sudo` is required for v1.** `wdutil info` (command 6) requires root and is excluded. `system_profiler SPAirPortDataType -json` (command 5) provides RSSI, noise, channel, rate, PHY mode, and security without elevated privileges. Future phases may add sudo-gated diagnostics behind an `--elevated` flag.

---

## Key Findings for v1

1. **Default gateway** — obtained via `route -n get default` (`gateway` field). Use as LAN ping target.
2. **Internet probes** — `ping` to `1.1.1.1` (Cloudflare) and `8.8.8.8` (Google DNS). Both work without sudo; ~11ms average RTT, 0% loss.
3. **Wi-Fi details** — from `system_profiler SPAirPortDataType -json`:
   - RSSI / Noise → `spairport_signal_noise` (parse `-X dBm / -Y dBm`)
   - Channel → `spairport_network_channel`
   - Tx rate → `spairport_network_rate`
   - PHY mode → `spairport_network_phymode`
   - Security → `spairport_security_mode`
   - SSID → `_name` (redact before logging)
4. **Interface enumeration** — `networksetup -listallhardwareports` identifies Wi-Fi as `en0`.
5. **Legacy `airport` binary absent** on macOS 25.3.0. Do not depend on it.
6. **`networksetup -getairportnetwork` unreliable** on this macOS version despite being connected. Do not use.

---

## Capture Script

```bash
#!/bin/bash
# netpulse-macos-capture.sh — captures macOS network commands for NetPulse dev
RAW=$(mktemp /tmp/netpulse-raw-XXXXXX)
SANITIZED="macos-capture-sanitized.txt"
# Collect all output (continues through failures)
{
  echo "=== uname ==="; uname -a
  echo ""; echo "=== networksetup -listallhardwareports ==="; networksetup -listallhardwareports 2>&1 || true
  echo ""; echo "=== route -n get default ==="; route -n get default 2>&1 || true
  echo ""; echo "=== networksetup -getairportnetwork en0 ==="; networksetup -getairportnetwork en0 2>&1 || true
  echo ""; echo "=== airport -I ==="; /System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I 2>&1 || true
  echo ""; echo "=== system_profiler SPAirPortDataType -json ==="; system_profiler SPAirPortDataType -json 2>&1 || true
  echo ""; echo "=== wdutil info ==="; wdutil info 2>&1 || true
  echo ""; echo "=== ping -c 5 -i 0.2 1.1.1.1 ==="; ping -c 5 -i 0.2 1.1.1.1 2>&1 || true
  echo ""; echo "=== ping -c 5 -i 0.2 8.8.8.8 ==="; ping -c 5 -i 0.2 8.8.8.8 2>&1 || true
} > "$RAW"
# Automated redactions (MACs, emails)
sed -E -e 's/([0-9A-Fa-f]{2}[:.-]){5}[0-9A-Fa-f]{2}/<redacted-mac>/g' \
       -e 's/[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}/<redacted-email>/g' \
       -e 's/\b(10\.[0-9]{1,3}\.[0-9]{1,3}\.[0-9]{1,3}|172\.(1[6-9]|2[0-9]|3[0-1])\.[0-9]{1,3}\.[0-9]{1,3}|192\.168\.[0-9]{1,3}\.[0-9]{1,3}|169\.254\.[0-9]{1,3}\.[0-9]{1,3})\b/<redacted-private-ip>/g' \
       "$RAW" > "$SANITIZED"
rm -f "$RAW"
echo "Raw temp file deleted."
echo "Sanitized output: $SANITIZED"
echo "=== Manual review required ==="
echo "Check for SSIDs (JSON _name strings) and hostnames; replace with <redacted>."
```

Run: `bash netpulse-macos-capture.sh`. Raw temp file deleted after sanitization. **Manually review** sanitized output for SSIDs and hostnames before sharing.

---

## Process Trace

- **Worker backend:** pi / OpenCode draft
- **Spec review (delegate):** Verify all 8 commands are listed, statuses accurate per captured output, summary table matches.
- **Privacy/quality review (delegate):** Confirm no MACs, real SSIDs, gateway IPs, or sensitive identifiers appear. Capture script includes redaction instructions.
- **Q final verification:** Confirm `git diff` shows only the allowed file changed (`docs/spikes/macos-network-command-discovery.md`); no sudo required for v1; no fabricated command output.
