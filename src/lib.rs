use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct NetworkSnapshot {
    pub timestamp: &'static str,
    pub internet: InternetSnapshot,
    pub wifi: WifiSnapshot,
    pub health: HealthSnapshot,
}

#[derive(Debug, Serialize)]
pub struct InternetSnapshot {
    pub gateway: ProbeResult,
    pub cloudflare: ProbeResult,
    pub dns_ms: Option<u32>,
    pub https_ms: Option<u32>,
}

#[derive(Debug, Serialize)]
pub struct ProbeResult {
    pub host: &'static str,
    pub avg_ms: Option<f32>,
    pub loss_pct: Option<f32>,
    pub jitter_ms: Option<f32>,
}

#[derive(Debug, Serialize)]
pub struct WifiSnapshot {
    pub interface_name: &'static str,
    pub ssid: Option<&'static str>,
    pub bssid: Option<&'static str>,
    pub rssi_dbm: Option<i32>,
    pub noise_dbm: Option<i32>,
    pub channel: Option<&'static str>,
    pub tx_rate_mbps: Option<u32>,
    pub quality: &'static str,
}

#[derive(Debug, Serialize)]
pub struct HealthSnapshot {
    pub status: &'static str,
    pub reasons: Vec<&'static str>,
}

/// Return deterministic placeholder data for the phase 1 CLI skeleton.
///
/// This does not inspect the host network. Real macOS collection will be added
/// after command output is captured and normalized in a follow-up task.
pub fn sample_snapshot() -> NetworkSnapshot {
    NetworkSnapshot {
        timestamp: "2026-05-28T00:00:00Z",
        internet: InternetSnapshot {
            gateway: ProbeResult {
                host: "192.168.1.1",
                avg_ms: Some(2.1),
                loss_pct: Some(0.0),
                jitter_ms: Some(0.4),
            },
            cloudflare: ProbeResult {
                host: "1.1.1.1",
                avg_ms: Some(8.5),
                loss_pct: Some(0.0),
                jitter_ms: Some(1.2),
            },
            dns_ms: Some(14),
            https_ms: Some(52),
        },
        wifi: WifiSnapshot {
            interface_name: "en0",
            ssid: Some("ExampleWiFi"),
            bssid: Some("aa:bb:cc:dd:ee:ff"),
            rssi_dbm: Some(-51),
            noise_dbm: Some(-92),
            channel: Some("6"),
            tx_rate_mbps: Some(866),
            quality: "placeholder",
        },
        health: HealthSnapshot {
            status: "unknown",
            reasons: vec!["placeholder sample data; real network probes not implemented yet"],
        },
    }
}
