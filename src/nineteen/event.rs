use crate::nineteen::{PacketHeader, VehicleIndex};
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::io::{Cursor, Error, ErrorKind};

#[derive(Debug, PartialEq)]
pub struct FastestLap {
    /// The index of the car achieving the fastest lap.
    pub vehicle_index: VehicleIndex,

    /// The lap time in seconds.
    pub lap_time: f32,
}

#[derive(Debug, PartialEq)]
pub struct Retirement {
    /// The index of the car that is retiring from the race.
    pub vehicle_index: VehicleIndex,
}

#[derive(Debug, PartialEq)]
pub struct TeammateInPits {
    /// The index of the teammate's car, who is currently pitting.
    pub vehicle_index: VehicleIndex,
}

#[derive(Debug, PartialEq)]
pub struct RaceWinner {
    /// The index of the car winning the race.
    pub vehicle_index: VehicleIndex,
}

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

pub struct EventPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// The type of event.
    pub event: Event,
}

impl EventPacket {
    fn peek_event_code(cursor: &mut Cursor<BytesMut>) -> String {
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

    fn decode_session_started(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::SessionStarted,
        })
    }

    fn decode_session_ended(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::SessionEnded,
        })
    }

    fn decode_fastest_lap(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::FastestLap(FastestLap {
                vehicle_index: cursor.get_u8(),
                lap_time: cursor.get_f32_le(),
            }),
        })
    }

    fn decode_retirement(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::Retirement(Retirement {
                vehicle_index: cursor.get_u8(),
            }),
        })
    }

    fn decode_drs_enabled(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::DrsEnabled,
        })
    }

    fn decode_drs_disabled(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::DrsDisabled,
        })
    }

    fn decode_teammate_pits(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::TeammatesInPits(TeammateInPits {
                vehicle_index: cursor.get_u8(),
            }),
        })
    }

    fn decode_checkered_flag(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
        Ok(EventPacket {
            header: PacketHeader::decode(cursor)?,
            event: Event::ChequeredFlag,
        })
    }

    fn decode_race_winner(cursor: &mut Cursor<BytesMut>) -> Result<EventPacket, Error> {
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

    fn decode(cursor: &mut Cursor<BytesMut>) -> Result<Self, Error>
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

        let mut cursor = Cursor::new(bytes);

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

        let mut cursor = Cursor::new(bytes);

        let packet = EventPacket::from_bytes(&mut cursor).unwrap();
        assert_eq!(Event::SessionStarted, packet.event)
    }
}
