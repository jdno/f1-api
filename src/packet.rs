//! Utilities to decode UDP packets into high-level data structures.

use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

/// A trait to decode UDP packets.
///
/// The F1 games publish their session and telemetry data to the network via a
/// UDP socket. The UDP packets are decoded, and converted to their respective
/// representations in this library. The `FromBytes` trait is implemented for
/// each packet, and maps the raw bytes in the UDP packet to Rust types.
///
/// The trait defines three public functions. `from_bytes` is the high-level
/// interface that should be used by callers to convert a buffer of bytes to a
/// packet. Besides calling `decode`, the second function of the trait, to do
/// the actual conversion, it executes sanity checks to prevent panics at
/// runtime. One sanity check is confirming the packet has the right size, which
/// requires the implementation of `packet_size`.
pub trait FromBytes {
    /// Convert a buffer of bytes to an F1 packet.
    ///
    /// Calling this function is the preferred method for converting a UDP
    /// packet to an F1 packet of this library. Before attempting the
    /// conversion, this function checks that the buffer has the right size. The
    /// actual decoding is done through the `decode` function in the same trait.
    fn from_bytes(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let expected_size = Self::buffer_size();
        let packet_size = cursor.remaining();

        if packet_size != expected_size {
            return Err(Error::new(
                ErrorKind::UnexpectedEof,
                format!(
                    "Packet is expected to have a size of {} bytes, but was {}.",
                    expected_size, packet_size
                ),
            ));
        }

        Self::decode(cursor)
    }

    /// Define the expected buffer size in bytes.
    ///
    /// Each UDP packet's specification contains the total size of the packet.
    /// It is used to check whether all UDP data frames have already been
    /// received. If this is not the case, an error is thrown that signals the
    /// decoder to wait for more data before attempting the conversion again.
    ///
    /// __Attention__: The size of each packet is measured in bytes.
    ///
    /// # Examples
    ///
    /// ```
    /// use f1_api::packet::FromBytes;
    /// # use bytes::BytesMut;
    /// # use std::io::{Error, Cursor};
    ///
    /// struct Packet {
    ///     id: u8
    /// }
    ///
    /// impl FromBytes for Packet {
    ///     fn buffer_size() -> usize {
    ///         1
    ///     }
    ///
    ///     // Other trait functions...
    ///     #
    ///     # fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error> where
    ///     #    Self: Sized{
    ///     #    unimplemented!()
    ///     # }
    /// }
    /// ```
    fn buffer_size() -> usize;

    /// Decode a buffer of bytes to a packet.
    ///
    /// Following the UDP packet specification, this function decodes the
    /// content of the UDP packet to its representation in Rust. This is the
    /// low-level counterpart to `from_bytes`, and the caller must guarantee
    /// that the buffer has the right size.
    ///
    /// # Panics
    ///
    /// This function panics if the buffer is smaller than the expected size of
    /// the packet.
    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
    where
        Self: Sized;
}

#[cfg(test)]
mod tests {
    use crate::packet::FromBytes;
    use bytes::{Buf, BufMut, BytesMut};
    use std::io::{Cursor, Error, ErrorKind};

    struct Packet {
        id: u8,
    }

    impl FromBytes for Packet {
        fn buffer_size() -> usize {
            1
        }

        fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
        where
            Self: Sized,
        {
            Ok(Packet {
                id: cursor.get_u8(),
            })
        }
    }

    #[test]
    fn from_bytes() {
        let mut bytes = BytesMut::new();
        bytes.put_u8(1);

        let mut cursor = Cursor::new(bytes);

        let packet = Packet::from_bytes(&mut cursor).unwrap();
        assert_eq!(1, packet.id);
    }

    #[test]
    fn from_bytes_with_zero_length() {
        let bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(bytes);

        match Packet::from_bytes(&mut cursor) {
            Ok(_) => panic!("Expected decoding header from zero length byte buffer to fail"),
            Err(error) => assert_eq!(ErrorKind::UnexpectedEof, error.kind()),
        }
    }

    #[test]
    fn buffer_size() {
        assert_eq!(1, Packet::buffer_size())
    }

    #[test]
    fn decode() {
        let mut bytes = BytesMut::new();
        bytes.put_u8(1);

        let mut cursor = Cursor::new(bytes);

        let packet = Packet::decode(&mut cursor).unwrap();
        assert_eq!(1, packet.id)
    }

    #[test]
    #[should_panic(expected = "self.remaining() >= 1")]
    fn decode_with_zero_length() {
        let bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(bytes);

        Packet::decode(&mut cursor).unwrap();
    }
}
