//! Decoder for event packets sent by F1 2019
//!
//! F1 2019 extended the event packet with seven new events compared to its predecessor, four of
//! which can carry a payload.

use crate::nineteen::header::decode_header;
use crate::packet::event::{
    Event, EventPacket, FastestLap, RaceWinner, Retirement, TeammateInPits,
};
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};
use std::time::Duration;

/// Size of the event packet in bytes
///
/// The event packet can have a maximum size of 32 bytes, but since not all events carry a payload,
/// it might very well be smaller.
pub const PACKET_SIZE: usize = 32;

/// Decode an event packet sent by F1 2019
///
/// F1 2019 extended the event packet with seven new events compared to its predecessor, four of
/// which can carry a payload. A four character event code is provided after the packet header to
/// identify the event. Based on this code the right decoding function is called, and a variant of
/// the `EventPacket` is returned.
pub fn decode_event(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
    let header = decode_header(cursor)?;
    let event_code = decode_event_code(cursor);

    let payload = match event_code.as_str() {
        "SSTA" => Event::SessionStarted,
        "SEND" => Event::SessionEnded,
        "FTLP" => decode_fastest_lap(cursor),
        "RTMT" => decode_retirement(cursor),
        "DRSE" => Event::DrsEnabled,
        "DRSD" => Event::DrsDisabled,
        "TMPT" => decode_teammate_pits(cursor),
        "CHQF" => Event::ChequeredFlag,
        "RCWN" => decode_race_winner(cursor),
        event_code => {
            return Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unexpected event code {}", event_code),
            ))
        }
    };

    Ok(EventPacket::new(header, payload))
}

/// Decode the event code at the beginning of the event packet
///
/// The event packet contains a string that identifies the type of the event. Based on the event
/// code different logic can be used to decode the remainder of the packet.
fn decode_event_code(cursor: &mut Cursor<&mut BytesMut>) -> String {
    [
        cursor.get_u8() as char,
        cursor.get_u8() as char,
        cursor.get_u8() as char,
        cursor.get_u8() as char,
    ]
    .iter()
    .collect()
}

/// Decode the "Fastest Lap" event.
fn decode_fastest_lap(cursor: &mut Cursor<&mut BytesMut>) -> Event {
    Event::FastestLap(FastestLap::new(
        cursor.get_u8(),
        Duration::from_secs_f32(cursor.get_f32_le()),
    ))
}

/// Decode the "Retirement" event.
fn decode_retirement(cursor: &mut Cursor<&mut BytesMut>) -> Event {
    Event::Retirement(Retirement::new(cursor.get_u8()))
}

/// Decode the "Teammate in Pits" event.
fn decode_teammate_pits(cursor: &mut Cursor<&mut BytesMut>) -> Event {
    Event::TeammatesInPits(TeammateInPits::new(cursor.get_u8()))
}

/// Decode the "Race Winner" event.
fn decode_race_winner(cursor: &mut Cursor<&mut BytesMut>) -> Event {
    Event::RaceWinner(RaceWinner::new(cursor.get_u8()))
}

#[cfg(test)]
mod tests {
    use crate::nineteen::event::{decode_event, PACKET_SIZE};
    use crate::packet::event::Event;
    use bytes::{BufMut, BytesMut};
    use std::io::Cursor;

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(0);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn from_bytes_with_ftlp_event() {
        let bytes = BytesMut::with_capacity(PACKET_SIZE);
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(b'F');
        bytes.put_u8(b'T');
        bytes.put_u8(b'L');
        bytes.put_u8(b'P');
        bytes.put_u8(1);
        bytes.put_f32_le(2.0);

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_event(&mut cursor).unwrap();
        match packet.event() {
            Event::FastestLap(fastest_lap) => assert_eq!(2, fastest_lap.time().as_secs()),
            _ => panic!("Expected a fastest lap event"),
        }
    }

    #[test]
    fn from_bytes_with_ssta_event() {
        let bytes = BytesMut::with_capacity(PACKET_SIZE);
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(b'S');
        bytes.put_u8(b'S');
        bytes.put_u8(b'T');
        bytes.put_u8(b'A');
        let padding = vec![0u8; 5];
        bytes.put(padding.as_slice());

        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_event(&mut cursor).unwrap();
        assert_eq!(Event::SessionStarted, *packet.event())
    }
}
