use netpulse_probe::{parse_default_route, parse_system_profiler_airport};

#[test]
fn parses_default_route_gateway_interface_and_mtu() {
    let output = r#"
   route to: default
destination: default
    gateway: 203.0.113.1
  interface: en0
      flags: <UP,GATEWAY,DONE,STATIC,PRCLONING,GLOBAL>
       mtu: 1500
"#;

    let route = parse_default_route(output).expect("default route parses");

    assert_eq!(route.gateway, "203.0.113.1");
    assert_eq!(route.interface_name, "en0");
    assert_eq!(route.mtu, Some(1500));
}

#[test]
fn parses_connected_wifi_details_from_system_profiler_json() {
    let json = r#"
{
  "SPAirPortDataType": [
    {
      "_name": "Wi-Fi",
      "spairport_airport_interfaces": [
        {
          "_name": "en0",
          "spairport_current_network_information": {
            "_name": "<redacted-ssid>",
            "spairport_network_channel": "5 (6GHz, 160MHz)",
            "spairport_network_phymode": "802.11ax",
            "spairport_network_rate": "1441",
            "spairport_security_mode": "wpa3_personal",
            "spairport_signal_noise": "-48 dBm / -92 dBm",
            "spairport_status_information": "spairport_status_connected"
          }
        },
        {
          "_name": "awdl0"
        }
      ]
    }
  ]
}
"#;

    let wifi = parse_system_profiler_airport(json, "en0").expect("connected wifi parses");

    assert_eq!(wifi.interface_name, "en0");
    assert_eq!(wifi.ssid.as_deref(), Some("<redacted-ssid>"));
    assert_eq!(wifi.bssid, None);
    assert_eq!(wifi.rssi_dbm, Some(-48));
    assert_eq!(wifi.noise_dbm, Some(-92));
    assert_eq!(wifi.channel.as_deref(), Some("5 (6GHz, 160MHz)"));
    assert_eq!(wifi.tx_rate_mbps, Some(1441));
    assert_eq!(wifi.quality, "connected");
}

#[test]
fn does_not_treat_not_connected_status_as_connected() {
    let json = r#"{"SPAirPortDataType":[{"spairport_airport_interfaces":[{"_name":"en0","spairport_current_network_information":{"spairport_status_information":"spairport_status_not_connected","spairport_network_rate":1441}}]}]}"#;

    let wifi = parse_system_profiler_airport(json, "en0").expect("wifi parses");

    assert_eq!(wifi.quality, "unknown");
    assert_eq!(wifi.tx_rate_mbps, Some(1441));
}
