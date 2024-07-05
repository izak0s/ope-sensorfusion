use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use chacha20poly1305::Key;

use sf_ope_commons::data::sensor_value::SensorValue;
use sf_ope_commons::network::manager::{PacketManager, Packets};
use sf_ope_commons::network::packet::sensor_data_reply_packet::SensorDataReplyPacket;
use sf_ope_commons::utils::crypt_utils::combine_keys;

pub struct SensorServer {
    pub(crate) port: u16,
    pub(crate) faulty: bool,
    pub(crate) collector_key: Key,
    pub(crate) e2e_key: Key,
}

impl SensorServer {
    pub(crate) fn start_server(&self) -> Result<(), String> {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), self.port);
        let sock = UdpSocket::bind(bind_addr)
            .expect(format!("Failed to bind on port {}", self.port).as_str());
        println!(
            "Listening on port {} as a {} sensor",
            self.port,
            if self.faulty { "malicious" } else { "honest" }
        );

        let mut buf = [0; 16];
        loop {
            let (len, addr) = sock
                .recv_from(&mut buf)
                .map_err(|e| format!("Failed to recv buffer: {}", e))?;
            let rdr = Cursor::new(&buf[..len]);

            // Handle packet
            if let Err(value) = self._handle_packet(rdr, addr, &sock) {
                println!(
                    "Failed to handle packet from {} with a size of {} ({})",
                    addr, len, value
                )
            }
        }
    }

    fn _handle_packet(&self, mut rdr: Cursor<&[u8]>, addr: SocketAddr, sock: &UdpSocket,) -> Result<(), String> {
        let mapped_packet = PacketManager::handle_packet(&mut rdr, addr, None)?;

        // Handle mapped packet
        match mapped_packet {
            Packets::RequestSensorData(packet) => {
                let combined: [u8; 16] = combine_keys(*b"secret_senso", packet.seed);
                let encrypted_value = SensorValue::generate(self.faulty).encrypt(combined, &self.e2e_key);

                let encoded = PacketManager::construct_packet(SensorDataReplyPacket { encrypted_value }, Some(&self.collector_key));
                sock.send_to(&encoded, addr)
                    .map_err(|e| format!("Unable to send packets over the network ({})", e))?;
            }
            _ => {
                return Err(format!(
                    "Received an unhandled packet from {}", addr
                ));
            }
        }

        Ok(())
    }
}
