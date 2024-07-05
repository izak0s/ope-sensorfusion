use std::env;

use sf_ope_commons::utils::env_parser::parse_env_key;

use crate::sensor_server::SensorServer;

pub mod sensor_server;

fn main() {
    // Parse port
    let port: u16 = match env::var("PORT") {
        Ok(port) => port.parse::<u16>().expect("Unable to parse port"),
        Err(_) => 13300u16,
    };

    // Parse faulty
    let faulty: bool = match env::var("FAULTY") {
        Ok(port) => port
            .parse::<bool>()
            .expect("Unable to parse FAULTY env variable"),
        Err(_) => false,
    };

    let collector_key = parse_env_key("COLLECTOR_KEY");
    let e2e_key = parse_env_key("E2E_KEY");

    SensorServer { port, faulty, collector_key, e2e_key }
        .start_server()
        .expect("Failed to start server");
}
