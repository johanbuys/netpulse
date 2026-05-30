use std::cell::RefCell;

use netpulse_probe::{CommandRunner, collect_macos_snapshot};

struct FakeRunner {
    calls: RefCell<Vec<(String, Vec<String>)>>,
}

impl FakeRunner {
    fn new() -> Self {
        Self {
            calls: RefCell::new(Vec::new()),
        }
    }
}

impl CommandRunner for FakeRunner {
    fn run(&self, program: &str, args: &[&str]) -> Result<String, String> {
        self.calls.borrow_mut().push((
            program.to_string(),
            args.iter().map(|arg| arg.to_string()).collect(),
        ));

        match (program, args) {
            ("route", ["-n", "get", "default"]) => Ok(r#"
   route to: default
destination: default
    gateway: 203.0.113.1
  interface: en0
       mtu: 1500
"#
            .to_string()),
            ("system_profiler", ["SPAirPortDataType", "-json"]) => Ok(r#"
{
  "SPAirPortDataType": [{
    "spairport_airport_interfaces": [{
      "_name": "en0",
      "spairport_current_network_information": {
        "_name": "<redacted-ssid>",
        "spairport_network_channel": "5 (6GHz, 160MHz)",
        "spairport_network_rate": "1441 Mb/s",
        "spairport_signal_noise": "-48 dBm / -92 dBm",
        "spairport_status_information": "spairport_status_connected"
      }
    }]
  }]
}
"#
            .to_string()),
            _ => Err(format!("unexpected command: {program} {args:?}")),
        }
    }
}

#[test]
fn collector_uses_runner_and_combines_route_and_wifi_parsers() {
    let runner = FakeRunner::new();

    let snapshot = collect_macos_snapshot(&runner, "2026-05-29T00:00:00Z")
        .expect("snapshot collection from fake command output succeeds");

    assert_eq!(snapshot.timestamp, "2026-05-29T00:00:00Z");
    assert_eq!(snapshot.internet.gateway.host, "203.0.113.1");
    assert_eq!(snapshot.wifi.interface_name, "en0");
    assert_eq!(snapshot.wifi.ssid.as_deref(), Some("<redacted-ssid>"));
    assert_eq!(snapshot.wifi.rssi_dbm, Some(-48));
    assert_eq!(snapshot.wifi.noise_dbm, Some(-92));
    assert_eq!(snapshot.wifi.tx_rate_mbps, Some(1441));
    assert_eq!(snapshot.wifi.quality, "connected");

    assert_eq!(
        runner.calls.into_inner(),
        vec![
            (
                "route".to_string(),
                vec!["-n".to_string(), "get".to_string(), "default".to_string()]
            ),
            (
                "system_profiler".to_string(),
                vec!["SPAirPortDataType".to_string(), "-json".to_string()]
            ),
        ]
    );
}
