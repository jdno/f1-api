//! Packet with event details from F1 2019

use crate::nineteen::{PacketHeader, VehicleIndex};
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

/// The fastest lap time done by a particular driver.
#[derive(Debug, PartialEq)]
pub struct FastestLap {
    /// The index of the car achieving the fastest lap.
    pub vehicle_index: VehicleIndex,

    /// The lap time in seconds.
    pub lap_time: f32,
}

/// A driver that is retiring from the race, most often for technical reasons.
#[derive(Debug, PartialEq)]
pub struct Retirement {
    /// The index of the car that is retiring from the race.
    pub vehicle_index: VehicleIndex,
}

/// A teammate that has just entered the pits.
#[derive(Debug, PartialEq)]
pub struct TeammateInPits {
    /// The index of the teammate's car, who is currently pitting.
    pub vehicle_index: VehicleIndex,
}

/// The driver that has won the race.
#[derive(Debug, PartialEq)]
pub struct RaceWinner {
    /// The index of the car winning the race.
    pub vehicle_index: VehicleIndex,
}

/// Events that can occur during a session.
///
/// F1 2019 publishes event packets when certain events occur during a session. Each potential event
/// is represented as a variant of this enum. Some events carry a payload, which provides more
/// details about the event, e.g. which driver has retired in case of a retirement.
#[derive(Debug, PartialEq)]
pub enum Event {
    SessionStarted,
    SessionEnded,
    FastestLap(FastestLap),
    Retirement(Retirement),
    DrsEnabled,
    DrsDisabled,
    TeammatesInPits(TeammateInPits),
    ChequeredFlag,
    RaceWinner(RaceWinner),
}

/// A packet with details about an event that occurred in the session.
///
/// F1 2019 publishes event packets when certain events occur during a session. The packet consists
/// of the packet header, and an `Event`. The event can carry a payload, e.g. the driver who is
/// retiring for a retirement event.
pub struct EventPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The type of event.
    pub event: Event,
}

impl EventPacket {
    /// Peek into the packet to determine the event type.
    ///
    /// The event packet contains a string that identifies the type of the event. To be able to
    /// properly parse the packet, its type must be known beforehand. Using this method, the event
    /// code can be retrieved from the packet without modifying the cursor.
    fn peek_event_code(cursor: &mut Cursor<&mut BytesMut>) -> String {
        cursor.set_position(PacketHeader::buffer_size() as u64);

        let event_code = [
            cursor.get_u8() as char,
            cursor.get_u8() as char,
            cursor.get_u8() as char,
            cursor.get_u8() as char,
        ]
        .iter()
        .collect();

        cursor.set_position(0);
        event_code
    }

    /// Decode the "Session Started" event.
    fn decode_session_started(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::SessionStarted,
        })
    }

    /// Decode the "Session Ended" event.
    fn decode_session_ended(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::SessionEnded,
        })
    }

    /// Decode the "Fastest Lap" event.
    fn decode_fastest_lap(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::FastestLap(FastestLap {
                vehicle_index: cursor.get_u8(),
                lap_time: cursor.get_f32_le(),
            }),
        })
    }

    /// Decode the "Retirement" event.
    fn decode_retirement(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::Retirement(Retirement {
                vehicle_index: cursor.get_u8(),
            }),
        })
    }

    /// Decode the "DRS Enabled" event.
    fn decode_drs_enabled(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::DrsEnabled,
        })
    }

    /// Decode the "DRS Disabled" event.
    fn decode_drs_disabled(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::DrsDisabled,
        })
    }

    /// Decode the "Teammate in Pits" event.
    fn decode_teammate_pits(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::TeammatesInPits(TeammateInPits {
                vehicle_index: cursor.get_u8(),
            }),
        })
    }

    /// Decode the "Chequered Flag" event.
    fn decode_checkered_flag(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::ChequeredFlag,
        })
    }

    /// Decode the "Race Winner" event.
    fn decode_race_winner(cursor: &mut Cursor<&mut BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::RaceWinner(RaceWinner {
                vehicle_index: cursor.get_u8(),
            }),
        })
    }
}

impl FromBytes for EventPacket {
    fn buffer_size() -> usize {
        32
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let event_code = EventPacket::peek_event_code(cursor);

        match event_code.as_str() {
            "SSTA" => EventPacket::decode_session_started(cursor),
            "SEND" => EventPacket::decode_session_ended(cursor),
            "FTLP" => EventPacket::decode_fastest_lap(cursor),
            "RTMT" => EventPacket::decode_retirement(cursor),
            "DRSE" => EventPacket::decode_drs_enabled(cursor),
            "DRSD" => EventPacket::decode_drs_disabled(cursor),
            "TMPT" => EventPacket::decode_teammate_pits(cursor),
            "CHQF" => EventPacket::decode_checkered_flag(cursor),
            "RCWN" => EventPacket::decode_race_winner(cursor),
            event_code => Err(Error::new(
                ErrorKind::InvalidData,
                format!("Unexpected event code {}", event_code),
            )),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::nineteen::event::{Event, EventPacket};
    use crate::packet::FromBytes;
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
        let bytes = BytesMut::with_capacity(EventPacket::buffer_size());
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(b'F');
        bytes.put_u8(b'T');
        bytes.put_u8(b'L');
        bytes.put_u8(b'P');
        bytes.put_u8(1);
        bytes.put_f32(2.0);

        let mut cursor = Cursor::new(&mut bytes);

        let packet = EventPacket::from_bytes(&mut cursor).unwrap();
        match packet.event {
            Event::FastestLap(fastest_lap) => assert!(fastest_lap.lap_time - 2.0 < 0.0001),
            _ => panic!("Expected a fastest lap event"),
        }
    }

    #[test]
    fn from_bytes_with_ssta_event() {
        let bytes = BytesMut::with_capacity(EventPacket::buffer_size());
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(b'S');
        bytes.put_u8(b'S');
        bytes.put_u8(b'T');
        bytes.put_u8(b'A');
        let padding = vec![0u8; 5];
        bytes.put(padding.as_slice());

        let mut cursor = Cursor::new(&mut bytes);

        let packet = EventPacket::from_bytes(&mut cursor).unwrap();
        assert_eq!(Event::SessionStarted, packet.event)
    }
}
