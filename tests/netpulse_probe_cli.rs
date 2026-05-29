use std::process::Command;

#[test]
fn json_flag_prints_valid_snapshot_with_required_top_level_keys() {
    let exe = std::env::var("CARGO_BIN_EXE_netpulse-probe").expect("binary path is available");
    let output = Command::new(exe)
        .arg("--json")
        .output()
        .expect("run netpulse-probe");

    assert!(
        output.status.success(),
        "expected command to succeed, stderr: {}",
        String::from_utf8_lossy(&output.stderr)
    );

    let stdout = String::from_utf8(output.stdout).expect("stdout is utf8");
    let value: serde_json::Value = serde_json::from_str(&stdout).expect("stdout is valid json");

    for key in ["timestamp", "internet", "wifi", "health"] {
        assert!(value.get(key).is_some(), "missing top-level key: {key}");
    }

    let status = value
        .pointer("/health/status")
        .and_then(serde_json::Value::as_str)
        .expect("health.status is a string");

    assert!(
        matches!(status, "healthy" | "degraded" | "bad" | "unknown"),
        "unexpected health.status: {status}"
    );
}
