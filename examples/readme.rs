use f1_api::packet::Packet::{Event, Lap, Motion, Participants, Session, Setup, Status, Telemetry};
use f1_api::F1;
use std::net::{IpAddr, SocketAddr};
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let ip_address = IpAddr::from([0, 0, 0, 0]);
    let port = 20777;
    let socket = SocketAddr::new(ip_address, port);

    let mut stream = F1::stream(socket).unwrap();

    while let Some(packet) = stream.next().await {
        match packet {
            Event(_) => println!("Received an Event packet"),
            Lap(_) => println!("Received a Lap packet"),
            Motion(_) => println!("Received a Motion packet"),
            Participants(_) => println!("Received a Participants packet"),
            Session(_) => println!("Received a Session packet"),
            Setup(_) => println!("Received aaSetup packet"),
            Status(_) => println!("Received a Status packet"),
            Telemetry(_) => println!("Received a Telemetry packet"),
        }
    }
}
