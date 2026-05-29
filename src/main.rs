use std::process::ExitCode;

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);

    match (args.next().as_deref(), args.next()) {
        (Some("--json"), None) => {
            match serde_json::to_string_pretty(&netpulse_probe::sample_snapshot()) {
                Ok(json) => {
                    println!("{json}");
                    ExitCode::SUCCESS
                }
                Err(error) => {
                    eprintln!("failed to serialize placeholder snapshot: {error}");
                    ExitCode::FAILURE
                }
            }
        }
        _ => {
            eprintln!("Usage: netpulse-probe --json");
            ExitCode::FAILURE
        }
    }
}
