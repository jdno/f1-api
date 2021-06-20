//! A Rust implementation of the telemetry API provided by modern F1 video games

use std::io::Error;
use std::net::SocketAddr;

use socket2::{Domain, Protocol, Socket, Type};
use tokio::net::UdpSocket;
use tokio_stream::{Stream, StreamExt};
use tokio_util::udp::UdpFramed;

use crate::codec::F1Codec;
use crate::packet::Packet;

pub mod codec;
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
    /// use std::net::{IpAddr, SocketAddr};
    ///
    /// use f1_api::F1;
    /// use f1_api::packet::Packet::{Event, Lap, Motion, Participants, Session, Setup, Status, Telemetry};
    /// use tokio_stream::StreamExt;
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
    ///             Participants(_) => println!("Received Participants packet"),
    ///             Session(_) => println!("Received Session packet"),
    ///             Setup(_) => println!("Received Setup packet"),
    ///             Status(_) => println!("Received Status packet"),
    ///             Telemetry(_) => println!("Received Telemetry packet"),
    ///         }
    ///     }
    /// }
    /// ```
    pub fn stream(socket_address: SocketAddr) -> Result<impl Stream<Item = Packet>, Error> {
        let socket = match socket_address {
            SocketAddr::V4(_) => Socket::new(Domain::IPV4, Type::DGRAM, Some(Protocol::UDP)),
            SocketAddr::V6(_) => Socket::new(Domain::IPV6, Type::DGRAM, Some(Protocol::UDP)),
        }?;

        socket.bind(&socket_address.into())?;

        Ok(UdpFramed::new(UdpSocket::from_std(socket.into())?, F1Codec)
            .map(|result| result.unwrap())
            .map(|(packet, _address)| packet))
    }
}
