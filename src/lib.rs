use serde::Serialize;
use serde_json::Value;

#[derive(Debug, Serialize)]
pub struct NetworkSnapshot {
    pub timestamp: String,
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
    pub host: String,
    pub avg_ms: Option<f32>,
    pub loss_pct: Option<f32>,
    pub jitter_ms: Option<f32>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct DefaultRoute {
    pub gateway: String,
    pub interface_name: String,
    pub mtu: Option<u32>,
}

#[derive(Debug, PartialEq, Serialize)]
pub struct WifiSnapshot {
    pub interface_name: String,
    pub ssid: Option<String>,
    pub bssid: Option<String>,
    pub rssi_dbm: Option<i32>,
    pub noise_dbm: Option<i32>,
    pub channel: Option<String>,
    pub tx_rate_mbps: Option<u32>,
    pub quality: String,
}

#[derive(Debug, Serialize)]
pub struct HealthSnapshot {
    pub status: String,
    pub reasons: Vec<String>,
}

pub trait CommandRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String, String>;
}

pub fn collect_macos_snapshot(
    runner: &impl CommandRunner,
    timestamp: &str,
) -> Result<NetworkSnapshot, String> {
    let route_output = runner.run("route", &["-n", "get", "default"])?;
    let route = parse_default_route(&route_output)?;
    let airport_output = runner.run("system_profiler", &["SPAirPortDataType", "-json"])?;
    let wifi = parse_system_profiler_airport(&airport_output, &route.interface_name)?;

    Ok(NetworkSnapshot {
        timestamp: timestamp.to_string(),
        internet: InternetSnapshot {
            gateway: ProbeResult {
                host: route.gateway,
                avg_ms: None,
                loss_pct: None,
                jitter_ms: None,
            },
            cloudflare: ProbeResult {
                host: "1.1.1.1".to_string(),
                avg_ms: None,
                loss_pct: None,
                jitter_ms: None,
            },
            dns_ms: None,
            https_ms: None,
        },
        wifi,
        health: HealthSnapshot {
            status: "unknown".to_string(),
            reasons: vec!["command output parsed; active probes not implemented yet".to_string()],
        },
    })
}

/// Return deterministic placeholder data for the phase 1 CLI skeleton.
///
/// This does not inspect the host network. Real macOS collection will be added
/// after command output is captured and normalized in a follow-up task.
pub fn sample_snapshot() -> NetworkSnapshot {
    NetworkSnapshot {
        timestamp: "2026-05-28T00:00:00Z".to_string(),
        internet: InternetSnapshot {
            gateway: ProbeResult {
                host: "example-gateway".to_string(),
                avg_ms: Some(2.1),
                loss_pct: Some(0.0),
                jitter_ms: Some(0.4),
            },
            cloudflare: ProbeResult {
                host: "1.1.1.1".to_string(),
                avg_ms: Some(8.5),
                loss_pct: Some(0.0),
                jitter_ms: Some(1.2),
            },
            dns_ms: Some(14),
            https_ms: Some(52),
        },
        wifi: WifiSnapshot {
            interface_name: "en0".to_string(),
            ssid: Some("ExampleWiFi".to_string()),
            bssid: None,
            rssi_dbm: Some(-51),
            noise_dbm: Some(-92),
            channel: Some("6".to_string()),
            tx_rate_mbps: Some(866),
            quality: "placeholder".to_string(),
        },
        health: HealthSnapshot {
            status: "unknown".to_string(),
            reasons: vec![
                "placeholder sample data; real network probes not implemented yet".to_string(),
            ],
        },
    }
}

/// Parse `route -n get default` output captured on macOS.
pub fn parse_default_route(output: &str) -> Result<DefaultRoute, String> {
    let mut gateway = None;
    let mut interface_name = None;
    let mut mtu = None;

    for line in output.lines() {
        let Some((key, value)) = line.trim().split_once(':') else {
            continue;
        };
        let value = value.trim();
        match key.trim() {
            "gateway" => gateway = Some(value.to_string()),
            "interface" => interface_name = Some(value.to_string()),
            "mtu" => mtu = value.parse::<u32>().ok(),
            _ => {}
        }
    }

    Ok(DefaultRoute {
        gateway: gateway.ok_or_else(|| "missing gateway in default route output".to_string())?,
        interface_name: interface_name
            .ok_or_else(|| "missing interface in default route output".to_string())?,
        mtu,
    })
}

/// Parse `system_profiler SPAirPortDataType -json` output for one interface.
pub fn parse_system_profiler_airport(
    json: &str,
    interface_name: &str,
) -> Result<WifiSnapshot, String> {
    let value: Value = serde_json::from_str(json)
        .map_err(|error| format!("invalid system_profiler airport JSON: {error}"))?;
    let interface = find_airport_interface(&value, interface_name)
        .ok_or_else(|| format!("missing airport interface {interface_name}"))?;

    let current_network = interface
        .get("spairport_current_network_information")
        .and_then(Value::as_object)
        .ok_or_else(|| format!("missing current network information for {interface_name}"))?;

    let signal_noise = current_network
        .get("spairport_signal_noise")
        .and_then(Value::as_str);
    let (rssi_dbm, noise_dbm) = signal_noise.map(parse_signal_noise).unwrap_or((None, None));

    let quality = match current_network
        .get("spairport_status_information")
        .and_then(Value::as_str)
    {
        Some("spairport_status_connected") => "connected",
        _ => "unknown",
    };

    Ok(WifiSnapshot {
        interface_name: interface_name.to_string(),
        ssid: string_field(&Value::Object(current_network.clone()), "_name"),
        bssid: string_field(
            &Value::Object(current_network.clone()),
            "spairport_network_bssid",
        ),
        rssi_dbm,
        noise_dbm,
        channel: string_field(
            &Value::Object(current_network.clone()),
            "spairport_network_channel",
        ),
        tx_rate_mbps: current_network
            .get("spairport_network_rate")
            .and_then(parse_u32_value),
        quality: quality.to_string(),
    })
}

fn find_airport_interface<'a>(value: &'a Value, interface_name: &str) -> Option<&'a Value> {
    match value {
        Value::Object(object) => {
            if object.get("_name").and_then(Value::as_str) == Some(interface_name)
                && object.contains_key("spairport_current_network_information")
            {
                return Some(value);
            }

            object
                .values()
                .find_map(|child| find_airport_interface(child, interface_name))
        }
        Value::Array(values) => values
            .iter()
            .find_map(|child| find_airport_interface(child, interface_name)),
        _ => None,
    }
}

fn string_field(value: &Value, key: &str) -> Option<String> {
    value.get(key).and_then(Value::as_str).map(str::to_string)
}

fn parse_signal_noise(signal_noise: &str) -> (Option<i32>, Option<i32>) {
    let mut parts = signal_noise
        .split('/')
        .map(|part| part.trim().trim_end_matches("dBm").trim());

    let rssi = parts.next().and_then(|part| part.parse::<i32>().ok());
    let noise = parts.next().and_then(|part| part.parse::<i32>().ok());

    (rssi, noise)
}

fn parse_u32_value(value: &Value) -> Option<u32> {
    match value {
        Value::Number(number) => number.as_u64().and_then(|n| u32::try_from(n).ok()),
        Value::String(text) => text
            .split_whitespace()
            .next()
            .and_then(|digits| digits.parse::<u32>().ok()),
        _ => None,
    }
}
