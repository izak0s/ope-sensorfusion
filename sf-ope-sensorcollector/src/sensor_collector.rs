use std::io::Cursor;
use std::net::{SocketAddr, UdpSocket};
use std::time::Duration;

use chacha20poly1305::Key;

use sf_ope_commons::data::encrypted_sensor_value::EncryptedSensorValue;
use sf_ope_commons::network::manager::{PacketManager, Packets};
use sf_ope_commons::network::packet::request_sensor_data_packet::RequestSensorDataPacket;

use crate::fault_tolerant_handler::FaultTolerantHandler;

pub struct SensorCollector<'a> {
    sensors: Vec<&'a str>,
    keys: Vec<Key>,
}

impl<'a> SensorCollector<'a> {
    pub(crate) fn new(address_list: Vec<&'a str>, key_list: Vec<Key>) -> Self {
        SensorCollector {
            sensors: address_list,
            keys: key_list,
        }
    }

    // Function to request the sensor readings from all the individual sensors and run Marzullo's algorithm
    pub(crate) fn collect_sensors(&self, key: [u8; 4]) -> Option<EncryptedSensorValue> {
        let mut results = Vec::new();

        // Collect all sensors
        for (idx, addr) in self.sensors.iter().enumerate() {
            match self.collect_individual_sensor(addr, self.keys.get(idx).unwrap(), key) {
                Ok(value) => {
                    println!(
                        "Received sensor values from {} ({}-{})",
                        addr, value.lower_bound, value.upper_bound
                    );
                    results.push(value);
                }
                Err(err) => {
                    println!(
                        "Failed to receive sensor values from {}. Reason: {}",
                        addr, err
                    );
                }
            }
        }

        println!(
            "Retrieved {}/{} sensor values ({:.2}%)",
            results.len(),
            self.sensors.len(),
            (results.len() as f64 / self.sensors.len() as f64) * 100f64
        );

        // Run Marzullo's algorithm
        FaultTolerantHandler { results }.calculate_marzullo()
    }

    // Collect individual sensor readings
    fn collect_individual_sensor(&self, sensor_addr: &str, key: &Key, seed: [u8; 4]) -> Result<EncryptedSensorValue, String> {
        // Setup socket
        let socket = UdpSocket::bind("0.0.0.0:0").map_err(|e| format!("Failed to bind: {}", e))?;
        socket
            .set_read_timeout(Some(Duration::from_millis(50)))
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
            .send(&PacketManager::construct_packet(RequestSensorDataPacket { seed }, Some(key)))
            .map_err(|e| format!("Failed to send packet: {}", e))?;

        // Attempt read from sensors
        let mut buf = [0; 256];
        let (len, _addr) = socket
            .recv_from(&mut buf)
            .map_err(|e| format!("Failed to receive: {}", e))?;

        // Decode packet ID
        let mut rdr = Cursor::new(&buf[..len]);

        let wrapped_packet = PacketManager::handle_packet(&mut rdr, _addr, Some(key))?;

        return match wrapped_packet {
            Packets::SensorDataReply(packet) => {
                // Return decoded packet
                Ok(packet.encrypted_value)
            }
            _ => {
                return Err(
                    "Unhandeled packet received".to_string(),
                );
            }
        };
    }
}
