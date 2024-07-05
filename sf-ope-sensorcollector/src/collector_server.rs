use std::io::Cursor;
use std::net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket};

use chacha20poly1305::Key;

use sf_ope_commons::network::manager::{PacketManager, Packets};
use sf_ope_commons::network::packet::sensor_data_reply_packet::SensorDataReplyPacket;

use crate::sensor_collector::SensorCollector;

pub struct CollectorServer {
    pub(crate) port: u16,
}

impl CollectorServer {
    pub(crate) fn start_server(&self, collector: &SensorCollector, client_key: &Key) -> Result<(), String> {
        let bind_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(0, 0, 0, 0)), self.port);
        let sock = UdpSocket::bind(bind_addr)
            .expect(format!("Failed to bind on port {}", self.port).as_str());
        println!(
            "Listening on port {} as for the client",
            self.port,
        );

        let mut buf = [0; 128];
        loop {
            let (len, addr) = sock
                .recv_from(&mut buf)
                .map_err(|e| format!("Failed to recv buffer: {}", e))?;
            let rdr = Cursor::new(&buf[..len]);

            // Handle packet
            if let Err(value) = self._handle_packet(rdr, addr, &sock, &collector, &client_key) {
                println!(
                    "Failed to handle packet from {} with a size of {} ({})",
                    addr, len, value
                )
            }
        }
    }

    fn _handle_packet(
        &self,
        mut rdr: Cursor<&[u8]>,
        addr: SocketAddr,
        sock: &UdpSocket,
        collector: &SensorCollector,
        client_key: &Key,
    ) -> Result<(), String> {
        let mapped_packet = PacketManager::handle_packet(&mut rdr, addr, None)?;

        // Handle incoming packet
        match mapped_packet {
            Packets::RequestSensorData(packet) => {
                let result = collector.collect_sensors(packet.seed);
                match result {
                    None => {
                        println!("No solution found for Marzullo’s algorithm")
                    }
                    Some(optimal) => {
                        println!("Optimal solution found using Marzullo's algorithm with lower: {} and upper: {}", optimal.lower_bound, optimal.upper_bound);

                        let encoded = PacketManager::construct_packet(SensorDataReplyPacket { encrypted_value: optimal }, Some(client_key));
                        sock.send_to(&encoded, addr).map_err(|e| format!("Unable to send packets over the network ({})", e))?;
                    }
                }
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
