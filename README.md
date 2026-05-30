# NetPulse

Standalone macOS menu bar network monitor built with Tauri, React/TypeScript, and Rust.

## Goal

NetPulse measures the Mac's own network experience and shows Internet + WLAN health from the menu bar. It is intentionally independent from Hermes: no backend service, no cloud dependency, no LLM calls.

## Current phase: Phase 1 — data spike

Before building UI, prove that we can reliably collect useful Mac-local network data:

- Internet probes: gateway ping, WAN ping, DNS latency, HTTPS latency, packet loss/jitter.
- Wi-Fi stats: interface, SSID, BSSID, RSSI, noise, channel, tx rate, PHY mode.
- Local persistence: JSON samples suitable for a future dashboard.

## Product and handoff docs

- `docs/product-requirements.md` — full PRD: users, goals, non-goals, requirements, milestones, open decisions.
- `docs/technical-specification.md` — architecture, data collection, model, UI/backend boundaries, testing strategy.
- `docs/development-plan.md` — phased roadmap and task-by-task handoff plan.
- `docs/agentic-development-operating-model.md` — how AI worker agents should be used safely.

## Planned stack

- Tauri 2
- React
- TypeScript
- Rust backend commands
- macOS system probes and shell-command fallbacks

## Non-goals

- No Hermes backend coupling.
- No Telegram dependency.
- No cloud service requirement.
- No speed tests in the default background loop.
