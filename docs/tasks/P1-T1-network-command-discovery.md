# NetPulse Task Card

## Task ID

`P1-T1`

## Title

Document macOS network command outputs for the phase 1 data spike.

## Worker model

`q/manual-or-agent`

## Objective

Create a small, factual command-discovery document that records which macOS network commands are useful for NetPulse's first read-only data probe.

## Allowed files

The worker may only create or modify these files:

- `docs/spikes/macos-network-command-discovery.md`

If another file seems necessary, stop and report why.

## Context

NetPulse Phase 1 is a data spike before committing to UI details. The target end state is a small probe that emits one JSON object with current Internet and Wi-Fi health.

Relevant phase document:

- `docs/phase-1-data-spike.md`

Commands to investigate on Johan's Mac, if available:

```bash
networksetup -listallhardwareports
route -n get default
networksetup -getairportnetwork en0
/System/Library/PrivateFrameworks/Apple80211.framework/Versions/Current/Resources/airport -I
system_profiler SPAirPortDataType -json
wdutil info
ping -c 5 -i 0.2 1.1.1.1
ping -c 5 -i 0.2 8.8.8.8
```

This repository may be edited from a Linux host, so the worker must not fake macOS output. If commands cannot be run because the environment is not macOS, create the document with a clear `Not yet captured` status and exact instructions for capturing the output on Johan's Mac.

## Acceptance criteria

- [ ] The document lists every command above.
- [ ] For each command, the document records one of: `works`, `missing`, `requires sudo`, `redacted`, or `not yet captured`.
- [ ] The document has a section for fields expected from each command.
- [ ] The document explicitly notes that no sudo-only command should be required for v1.
- [ ] The document includes a copy/paste script Johan can run on macOS to capture sanitized output.
- [ ] The worker does not invent command output.

## Required verification

Run these commands and report the result:

```bash
uname -s
```

Expected:

```text
If Darwin: command discovery may be captured directly.
If Linux or anything else: mark macOS command outputs as not yet captured and include capture instructions.
```

Also run:

```bash
git diff -- docs/spikes/macos-network-command-discovery.md
```

Expected:

```text
Only the allowed file is changed.
```

## Max diff budget

Target: `220` changed lines.

If the task requires more, stop and explain before continuing.

## Stop conditions

Stop and report instead of guessing if:

- The solution requires changing files outside the allowed list.
- You are tempted to fabricate macOS command output.
- Sensitive local network data would be committed without sanitization.

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
