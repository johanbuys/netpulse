# NetPulse Development Plan and Handoff

## 1. Purpose

This document is the development handoff plan for NetPulse. It defines the work still needed, the order to do it in, and the constraints for dedicated implementation workflows.

Do not hand a worker the whole product. Hand workers one task card at a time.

## 2. Current baseline

As of this plan, NetPulse has:

- Rust crate and CLI skeleton.
- Deterministic `netpulse-probe --json` placeholder output.
- macOS command discovery document.
- Pure parsers for default route and `system_profiler SPAirPortDataType -json`.
- Injected command-runner collector boundary.
- Baseline CI.
- Agentic development operating model.
- Task-card template and first task cards.

Merged PRs already covered:

- P1-T0 baseline validation.
- P1-T1 macOS command discovery.
- P1-T2 Rust CLI JSON snapshot.
- P1-T3 macOS parser fixtures.
- P1-T4 collector boundary.

## 3. Development strategy

Build in this order:

1. Finish trustworthy data collection.
2. Normalize/classify the data.
3. Capture real samples from Johan's Mac.
4. Build the Tauri shell.
5. Render a useful dashboard.
6. Add persistence and polish.

The project should stay real-data-first. UI work should not outrun confidence in the measurements.

## 4. Phase 1: Data spike / CLI probe

Goal: a CLI that can produce a useful real network snapshot on macOS.

### P1-T5: Live macOS command runner

Objective:

Add a live `std::process::Command` implementation of `CommandRunner` without changing CLI behavior yet.

Allowed files suggestion:

- `src/lib.rs` or new `src/macos.rs`
- `tests/macos_command_runner.rs` if meaningful
- `docs/tasks/P1-T5-live-command-runner.md`

Acceptance:

- A live runner can execute fixed commands.
- Tests do not require live macOS network output.
- Errors include enough detail for debugging.
- No shell interpolation.
- Existing tests still pass.

Verification:

```bash
cargo test --all --locked
cargo fmt --check
cargo clippy --all-targets --all-features -- -D warnings
cargo build --locked
git diff --check
```

### P1-T6: Wire CLI to live collection on macOS

Objective:

Make `netpulse-probe --json` use live macOS collection when running on macOS, while preserving deterministic testability and graceful non-macOS behavior.

Acceptance:

- On macOS, CLI collects route and Wi-Fi output via live runner.
- On Linux CI/dev hosts, tests still pass and CLI behavior is documented.
- JSON schema remains stable enough for downstream UI work.
- Errors are serialized or reported consistently.

### P1-T7: Gateway and WAN probe metrics

Objective:

Add active latency/loss/jitter probes for gateway and WAN target.

Acceptance:

- Gateway host comes from default route.
- WAN target defaults to `1.1.1.1`.
- Probe returns average latency, loss percentage, jitter, and failure reason.
- Tests cover parser/math logic without requiring external network.
- Runtime has short timeouts.

### P1-T8: DNS and HTTPS timing

Objective:

Add DNS and HTTPS timing checks to the sample model.

Acceptance:

- DNS timing is measured using fixed/safe behavior.
- HTTPS timing uses a lightweight endpoint.
- Both have timeouts.
- Failure is represented distinctly from slow success.

### P1-T9: Health classification

Objective:

Implement transparent health classification and reasons.

Acceptance:

- Pure classifier function.
- Unit tests cover healthy, degraded, bad, and unknown.
- Reasons include measured values and thresholds.
- Missing critical data results in `unknown` rather than false confidence.

### P1-T10: Real Mac sample capture

Objective:

Run the CLI on Johan's Mac and commit only sanitized example outputs / findings.

Acceptance:

- At least one healthy sample captured.
- If possible, one degraded sample captured.
- All sensitive values redacted before commit.
- Findings document threshold or parser adjustments needed.

### P1 completion criteria

Phase 1 is complete when:

- `netpulse-probe --json` produces useful live output on macOS.
- Tests pass on the dev/CI environment.
- A human can inspect sanitized sample JSON and understand it.
- Health status is generated from actual measurements, not placeholders.

## 5. Phase 2: Tauri app shell

Goal: a minimal standalone macOS app that can call the probe backend.

### P2-T1: Initialize Tauri 2 app structure

Acceptance:

- Tauri app builds locally.
- React/TypeScript frontend exists.
- Rust core remains testable outside Tauri.
- Existing CLI checks still pass.

### P2-T2: Backend command for current sample

Acceptance:

- Tauri command calls Rust collector.
- Frontend can request a sample.
- Errors/partial results are represented in JSON.

### P2-T3: Menu-bar app shell

Acceptance:

- App runs as menu-bar utility.
- Popover opens from menu-bar item.
- Manual refresh triggers sample collection.

### P2 completion criteria

Phase 2 is complete when:

- A local app can be launched.
- The popover can display the latest sample JSON or structured placeholder view.
- The app remains independent from Hermes/cloud services.

## 6. Phase 3: Dashboard UX

Goal: make the UI useful for diagnosis.

### P3-T1: Health summary and reasons

Acceptance:

- Header shows status and last updated time.
- Reasons are visible and understandable.
- Unknown/partial data states are explicit.

### P3-T2: Wi-Fi card

Acceptance:

- Displays available Wi-Fi fields.
- Handles missing fields gracefully.
- Does not overstate quality.

### P3-T3: Internet probes card

Acceptance:

- Shows gateway, WAN, DNS, and HTTPS results.
- Failures/timeouts are distinguishable.

### P3-T4: Manual refresh and sampling state

Acceptance:

- Refresh button disabled or shows progress while sampling.
- Overlapping samples prevented.
- Errors shown without crashing UI.

## 7. Phase 4: Persistence and diagnostics

Goal: make NetPulse useful for intermittent issues and support sharing a sanitized snapshot.

### P4-T1: JSONL sample store

Acceptance:

- Append samples to local JSONL.
- Read recent samples.
- Bound retention.
- Tests use temporary directories.

### P4-T2: Recent history UI

Acceptance:

- Shows recent status trend.
- Makes intermittent degradation visible.

### P4-T3: Copy diagnostics

Acceptance:

- Produces sanitized text or JSON.
- Redacts private identifiers.
- Includes health reasons and latest measurements.

## 8. Phase 5: Packaging and beta

Goal: daily-use installable app.

Tasks:

- App icon and status icons.
- Build/release process.
- macOS permissions review.
- Signing/notarization investigation.
- Beta install instructions.
- Known limitations document.

## 9. Review expectations

Every implementation PR should include:

- Task card.
- Small diff.
- Test output.
- Privacy check if fixtures/diagnostics are touched.
- Review Copilot evidence pack once that workflow is automated enough.

The reviewer should be able to answer:

- What changed?
- Why did it change?
- Which files matter most?
- What behavior is now expected?
- What could break?
- Which tests prove it?
- What still needs human judgment?

## 10. Handoff checklist for a dedicated dev workflow

Before starting a task, the orchestrator should:

- Ensure `main` is clean and current.
- Create a branch named for one task.
- Write/update the task card.
- Define allowed files.
- Define acceptance criteria.
- Define exact verification commands.
- State privacy constraints.

Before merging a task, the orchestrator should:

- Review diff against task card.
- Run verification commands.
- Confirm no sensitive values were committed.
- Confirm no scope creep.
- Produce or update Review Copilot evidence pack if relevant.

## 11. Decision log

- Rust/Tauri chosen for standalone macOS app.
- NetPulse must not depend on Hermes backend.
- Phase 1 is real-data-first before UI.
- `system_profiler SPAirPortDataType -json` is the v1 Wi-Fi source until proven too slow or insufficient.
- `wdutil` excluded from v1 default path because it requires sudo.
- Worker agents receive task cards, not broad product instructions.

## 12. Immediate next move

Create task card `P1-T5-live-command-runner.md`, then implement it as a narrow PR.

Do not start the Tauri UI until the live CLI path is credible on macOS.
