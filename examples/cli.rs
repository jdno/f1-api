use clap::{crate_version, App, Arg};
use f1_api::nineteen::Packet;
use f1_api::packet::Packet::{Event, Lap, Motion, Nineteen};
use f1_api::F1;
use std::net::{IpAddr, SocketAddr};
use std::str::FromStr;
use tokio::stream::StreamExt;

#[tokio::main]
async fn main() {
    let matches = App::new("F1 API")
        .version(crate_version!())
        .arg(
            Arg::with_name("address")
                .short("a")
                .long("address")
                .value_name("IP ADDRESS")
                .help("IP address to bind the local socket to")
                .default_value("0.0.0.0")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("port")
                .short("p")
                .long("port")
                .value_name("PORT")
                .help("Port to bind the local socket to")
                .default_value("20777")
                .takes_value(true),
        )
        .get_matches();

    let ip_address = matches.value_of("address").unwrap();
    let port: u16 = matches.value_of("port").unwrap().parse().unwrap();

    let socket = SocketAddr::new(IpAddr::from_str(ip_address).unwrap(), port);
    let mut stream = F1::stream(socket).unwrap();

    while let Some(packet) = stream.next().await {
        match packet {
            Event(_) => println!("Received Event packet"),
            Lap(_) => println!("Received Lap packet"),
            Motion(_) => println!("Received Motion packet"),
            Nineteen(packet) => match packet {
                Packet::Participants(_) => println!("Received Participants packet"),
                Packet::Session(_) => println!("Received Session packet"),
                Packet::Setup(_) => println!("Received Setup packet"),
                Packet::Telemetry(_) => println!("Received Telemetry packet"),
                Packet::Status(_) => println!("Received Status packet"),
            },
        }
    }
}
