//! A Rust implementation of the telemetry API provided by modern F1 video games

use crate::codec::F1Codec;
use crate::packet::Packet;
use net2::UdpBuilder;
use std::io::Error;
use std::net::SocketAddr;
use tokio::net::UdpSocket;
use tokio::stream::{Stream, StreamExt};
use tokio_util::udp::UdpFramed;

pub mod codec;
pub mod from_bytes;
pub mod nineteen;
pub mod packet;
pub mod types;

/// A high-level interface to the telemetry data of modern F1 video games.
///
/// The F1 struct implements a high-level interface to the telemetry data of the
/// modern F1 video games. It is the recommended way to use the library, as it
/// provides a simple interface to consumers that hides the low-level internals
/// of the library.
pub struct F1 {}

impl F1 {
    /// Create a stream that yields decoded UDP packets.
    ///
    /// Modern F1 games publish their telemetry and session data through a UDP-based protocol. With
    /// this function, a stream can be created that listens at the given socket for incoming
    /// packets, decodes them using the `F1Codec`, and returns their Rust representations.
    ///
    /// # Examples
    ///
    /// ```
    /// use f1_api::F1;
    /// use f1_api::packet::Packet::{Event, Lap, Motion, Nineteen};
    /// use std::net::{IpAddr, SocketAddr};
    /// use tokio::stream::StreamExt;
    ///
    /// async fn example() {
    ///     let ip_address = IpAddr::from([0, 0, 0, 0]);
    ///     let port = 20777;
    ///     let socket = SocketAddr::new(ip_address, port);
    ///
    ///     let mut stream = F1::stream(socket).unwrap();
    ///
    ///     while let Some(packet) = stream.next().await {
    ///         match packet {
    ///             Event(_) => println!("Received Event packet"),
    ///             Lap(_) => println!("Received Lap packet"),
    ///             Motion(_) => println!("Received Motion packet"),
    ///             Nineteen(packet) => println!("Received a packet from F1 2019")
    ///         }
    ///     }
    /// }
    /// ```
    pub fn stream(socket_address: SocketAddr) -> Result<impl Stream<Item = Packet>, Error> {
        let socket = match socket_address {
            SocketAddr::V4(address) => UdpBuilder::new_v4()?.bind(address),
            SocketAddr::V6(address) => UdpBuilder::new_v6()?.only_v6(true)?.bind(address),
        }?;

        Ok(UdpFramed::new(UdpSocket::from_std(socket)?, F1Codec)
            .map(|result| result.unwrap())
            .map(|(packet, _address)| packet))
    }
}
