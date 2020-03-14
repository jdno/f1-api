//! Codec for modern F1 games

use crate::nineteen;
use crate::packet::{FromBytes, Packet};
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};
use tokio_util::codec::Decoder;

/// Codec to decode UDP packets published by modern F1 games.
///
/// This struct implements the `Decoder` trait for tokio-utils. It can be used to decode incoming
/// UDP packets, and convert them into internal data representations. The F1 codec can decode the
/// packets of all F1 games that are supported by this library.
pub struct F1Codec;

impl Decoder for F1Codec {
    type Item = Packet;
    type Error = Error;

    /// Decode a UDP packet and return its data.
    ///
    /// The `decode` method is called whenever a new data frame is received on a UDP socket, and the
    /// data frame is passed as an argument. This method has to make a few decisions then:
    ///
    /// 1. Does the data form a complete packet so that it can be decoded?
    /// 2. Is the packet a valid packet sent by an F1 game?
    /// 3. Can the packet be parsed?
    ///
    /// To answer these questions, the following process is used. First, the packet header is read
    /// to determine the game that sent the packet. With the game and the packet type from the
    /// header, the expected size of the packet can be determined by calling `buffer_size` from the
    /// `FromBytes` trait. If the packet is too small, `Ok(None)` is returned to signal that more
    /// data needs to be retrieved from the UDP socket.
    ///
    /// If the packet is complete, it is decoded using the `from_bytes` method in the `FromBytes`
    /// trait. If the packet can be decoded successfully, it is returned. Otherwise, the error from
    /// the decoding is returned, signaling that the UDP stream is corrupted and should be shut
    /// down.
    fn decode(&mut self, src: &mut BytesMut) -> Result<Option<Self::Item>, Error> {
        let mut cursor = Cursor::new(src);

        // Not enough data yet to decode the packet format.
        if cursor.remaining() < 2 {
            return Ok(None);
        }

        let packet_format = cursor.get_u16_le();

        let packet = match packet_format {
            2019 => nineteen::Packet::from_bytes(&mut cursor),
            format => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unknown packet format {}.", format),
            )),
        };

        match packet {
            Ok(packet) => {
                cursor.into_inner().clear();
                Ok(Some(Packet::Nineteen(packet)))
            }
            Err(error) => match error.kind() {
                // Signal more bytes are expected
                ErrorKind::UnexpectedEof => Ok(None),
                _ => Err(error),
            },
        }
    }
}
