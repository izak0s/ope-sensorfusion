use std::env;
use std::io::Cursor;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use chacha20poly1305::Key;
use rand::Rng;

use sf_ope_commons::network::manager::{PacketManager, Packets};
use sf_ope_commons::network::packet::request_sensor_data_packet::RequestSensorDataPacket;
use sf_ope_commons::utils::env_parser::parse_env_key;

fn main() -> Result<(), String> {
    let client_key = parse_env_key("CLIENT_KEY");
    let e2e_key = parse_env_key("E2E_KEY");

    let collector_address = env::var("COLLECTOR_ADDRESS").expect("COLLECTOR_ADDRESS not set");
    retrieve_sensor_value(collector_address, &client_key, &e2e_key)?;

    Ok(())
}

fn retrieve_sensor_value(sensor_addr: String, collector_key: &Key, e2e_key: &Key) -> Result<(), String> {
    let mut rnd = rand::thread_rng();
    let seed: [u8; 4] = rnd.gen();

    let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Failed to bind: {}", e))?;
    socket
        .set_read_timeout(Some(Duration::from_millis(1000)))
        .map_err(|e| format!("Failed to set read timeout: {}", e))?;

    // Attempt parsing address
    let sensor_socket_addr: SocketAddr = sensor_addr
        .parse()
        .map_err(|e| format!("Failed to parse sensor address: {}", e))?;
    socket
        .connect(sensor_socket_addr)
        .map_err(|e| format!("Failed to connect to sensor: {}", e))?;

    // Request sensor to send data
    socket
        .send(&PacketManager::construct_packet(RequestSensorDataPacket { seed }, Some(collector_key)))
        .map_err(|e| format!("Failed to send packet: {}", e))?;

    // Attempt read from sensors
    let mut buf = [0; 256];
    let (len, addr) = socket.recv_from(&mut buf).map_err(|e| format!("Failed to receive: {}", e))?;

    // Decode packet ID
    let mut rdr = Cursor::new(&buf[..len]);

    let wrapped_packet = PacketManager::handle_packet(&mut rdr, addr, Some(collector_key))?;

    return match wrapped_packet {
        Packets::SensorDataReply(packet) => {
            let value = packet.encrypted_value.decrypt(&e2e_key);

            println!("Received optimal sensor value with value {}-{}", value.lower_bound, value.upper_bound);

            Ok(())
        }
        _ => {
            return Err(String::from("Received an invalid packet, malicious sensor collector?"));
        }
    };
}