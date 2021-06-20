//! Decoder for participants packet sent by F1 2019
//!
//! F1 2019 extends the participants packet from F1 2018 with the `telemetry_privacy` field. Other
//! than that both games use the same packet format.

use std::io::{Cursor, Error, ErrorKind};

use bytes::{Buf, BytesMut};

use crate::nineteen::header::decode_header;
use crate::packet::ensure_packet_size;
use crate::packet::participants::{
    Controller, Driver, Nationality, Participant, ParticipantsPacket, Team, TelemetryPrivacy,
};

/// Size of the participants packet.
pub const PACKET_SIZE: usize = 1104;

/// Decode a participants packet sent by F1 2019
///
/// F1 2019 extends the participants packet from F1 2018 with the `telemetry_privacy` field. Other
/// than that both games use the same packet format.
pub fn decode_participants(
    cursor: &mut Cursor<&mut BytesMut>,
) -> Result<ParticipantsPacket, Error> {
    ensure_packet_size(PACKET_SIZE, cursor)?;

    let header = decode_header(cursor)?;
    let active_participants_count = cursor.get_u8();

    let mut participants = Vec::with_capacity(20);

    for _ in 0..20 {
        participants.push(Participant::new(
            decode_controller(cursor)?,
            decode_driver(cursor)?,
            decode_team(cursor)?,
            cursor.get_u8(),
            decode_nationality(cursor)?,
            decode_name(cursor),
            decode_telemetry_privacy(cursor)?,
        ))
    }

    Ok(ParticipantsPacket::new(
        header,
        active_participants_count,
        participants,
    ))
}

fn decode_controller(cursor: &mut Cursor<&mut BytesMut>) -> Result<Controller, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Controller::Human),
        1 => Ok(Controller::AI),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode controller.",
        )),
    }
}

fn decode_driver(cursor: &mut Cursor<&mut BytesMut>) -> Result<Driver, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Driver::CarlosSainz),
        1 => Ok(Driver::DaniilKvyat),
        2 => Ok(Driver::DanielRicciardo),
        6 => Ok(Driver::KimiRaikkonen),
        7 => Ok(Driver::LewisHamilton),
        9 => Ok(Driver::MaxVerstappen),
        10 => Ok(Driver::NicoHulkenburg),
        11 => Ok(Driver::KevinMagnussen),
        12 => Ok(Driver::RomainGrosjean),
        13 => Ok(Driver::SebastianVettel),
        14 => Ok(Driver::SergioPerez),
        15 => Ok(Driver::ValtteriBottas),
        19 => Ok(Driver::LanceStroll),
        20 => Ok(Driver::ArronBarnes),
        21 => Ok(Driver::MartinGiles),
        22 => Ok(Driver::AlexMurray),
        23 => Ok(Driver::LucasRoth),
        24 => Ok(Driver::IgorCorreia),
        25 => Ok(Driver::SophieLevasseur),
        26 => Ok(Driver::JonasSchiffer),
        27 => Ok(Driver::AlainForest),
        28 => Ok(Driver::JayLetourneau),
        29 => Ok(Driver::EstoSaari),
        30 => Ok(Driver::YasarAtiyeh),
        31 => Ok(Driver::CallistoCalabresi),
        32 => Ok(Driver::NaotaIzum),
        33 => Ok(Driver::HowardClarke),
        34 => Ok(Driver::WilheimKaufmann),
        35 => Ok(Driver::MarieLaursen),
        36 => Ok(Driver::FlavioNieves),
        37 => Ok(Driver::PeterBelousov),
        38 => Ok(Driver::KlimekMichalski),
        39 => Ok(Driver::SantiagoMoreno),
        40 => Ok(Driver::BenjaminCoppens),
        41 => Ok(Driver::NoahVisser),
        42 => Ok(Driver::GertWaldmuller),
        43 => Ok(Driver::JulianQuesada),
        44 => Ok(Driver::DanielJones),
        45 => Ok(Driver::ArtemMarkelov),
        46 => Ok(Driver::TadasukeMakino),
        47 => Ok(Driver::SeanGelael),
        48 => Ok(Driver::NyckDeVries),
        49 => Ok(Driver::JackAitken),
        50 => Ok(Driver::GeorgeRussell),
        51 => Ok(Driver::MaximilianGunther),
        52 => Ok(Driver::NireiFukuzumi),
        53 => Ok(Driver::LucaGhiotto),
        54 => Ok(Driver::LandoNorris),
        55 => Ok(Driver::SergioSetteCamara),
        56 => Ok(Driver::LouisDeletraz),
        57 => Ok(Driver::AntonioFuoco),
        58 => Ok(Driver::CharlesLeclerc),
        59 => Ok(Driver::PierreGasly),
        62 => Ok(Driver::AlexanderAlbon),
        63 => Ok(Driver::NicholasLatifi),
        64 => Ok(Driver::DorianBoccolacci),
        65 => Ok(Driver::NikoKari),
        66 => Ok(Driver::RobertoMerhi),
        67 => Ok(Driver::ArjunMaini),
        68 => Ok(Driver::AlessioLorandi),
        69 => Ok(Driver::RubenMeijer),
        70 => Ok(Driver::RashidNair),
        71 => Ok(Driver::JackTremblay),
        74 => Ok(Driver::AntonioGiovinazzi),
        75 => Ok(Driver::RobertKubica),
        78 => Ok(Driver::NobuharuMatsushita),
        79 => Ok(Driver::NikitaMazepin),
        80 => Ok(Driver::GuanyaZhou),
        81 => Ok(Driver::MickSchumacher),
        82 => Ok(Driver::CallumIlott),
        83 => Ok(Driver::JuanManuelCorrea),
        84 => Ok(Driver::JordanKing),
        85 => Ok(Driver::MahaveerRaghunathan),
        86 => Ok(Driver::TatianaCalderon),
        87 => Ok(Driver::AnthoineHubert),
        88 => Ok(Driver::GuilianoAlesi),
        89 => Ok(Driver::RalphBoschung),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode driver.",
        )),
    }
}

fn decode_team(cursor: &mut Cursor<&mut BytesMut>) -> Result<Team, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Team::Mercedes),
        1 => Ok(Team::Ferrari),
        2 => Ok(Team::RedBullRacing),
        3 => Ok(Team::Williams),
        4 => Ok(Team::RacingPoint),
        5 => Ok(Team::Renault),
        6 => Ok(Team::ToroRosso),
        7 => Ok(Team::Haas),
        8 => Ok(Team::McLaren),
        9 => Ok(Team::AlfaRomeo),
        10 => Ok(Team::McLaren1988),
        11 => Ok(Team::McLaren1991),
        12 => Ok(Team::Williams1992),
        13 => Ok(Team::Ferrari1995),
        14 => Ok(Team::Williams1996),
        15 => Ok(Team::McLaren1998),
        16 => Ok(Team::Ferrari2002),
        17 => Ok(Team::Ferrari2004),
        18 => Ok(Team::Renault2006),
        19 => Ok(Team::Ferrari2007),
        21 => Ok(Team::RedBull2010),
        22 => Ok(Team::Ferrari1976),
        23 => Ok(Team::ARTGrandPrix),
        24 => Ok(Team::CamposVexatecRacing),
        25 => Ok(Team::Carlin),
        26 => Ok(Team::CharouzRacingSystem),
        27 => Ok(Team::DAMS),
        28 => Ok(Team::RussianTime),
        29 => Ok(Team::MPMotorsport),
        30 => Ok(Team::Pertamina),
        31 => Ok(Team::McLaren1990),
        32 => Ok(Team::Trident),
        33 => Ok(Team::BWTArden),
        34 => Ok(Team::McLaren1976),
        35 => Ok(Team::Lotus1972),
        36 => Ok(Team::Ferrari1979),
        37 => Ok(Team::McLaren1982),
        38 => Ok(Team::Williams2003),
        39 => Ok(Team::Brawn2009),
        40 => Ok(Team::Lotus1978),
        42 => Ok(Team::ArtGP2019),
        43 => Ok(Team::Campos2019),
        44 => Ok(Team::Carlin2019),
        45 => Ok(Team::SauberJuniorCharouz2019),
        46 => Ok(Team::Dams2019),
        47 => Ok(Team::UniVirtuosi2019),
        48 => Ok(Team::MPMotorsport2019),
        49 => Ok(Team::Prema2019),
        50 => Ok(Team::Trident2019),
        51 => Ok(Team::Arden2019),
        63 => Ok(Team::Ferrari1990),
        64 => Ok(Team::McLaren2010),
        65 => Ok(Team::Ferrari2010),
        _ => Err(Error::new(ErrorKind::InvalidData, "Failed to decode team.")),
    }
}

fn decode_nationality(cursor: &mut Cursor<&mut BytesMut>) -> Result<Nationality, Error> {
    let value = cursor.get_u8();

    match value {
        1 => Ok(Nationality::American),
        2 => Ok(Nationality::Argentinean),
        3 => Ok(Nationality::Australian),
        4 => Ok(Nationality::Austrian),
        5 => Ok(Nationality::Azerbaijani),
        6 => Ok(Nationality::Bahraini),
        7 => Ok(Nationality::Belgian),
        8 => Ok(Nationality::Bolivian),
        9 => Ok(Nationality::Brazilian),
        10 => Ok(Nationality::British),
        11 => Ok(Nationality::Bulgarian),
        12 => Ok(Nationality::Cameroonian),
        13 => Ok(Nationality::Canadian),
        14 => Ok(Nationality::Chilean),
        15 => Ok(Nationality::Chinese),
        16 => Ok(Nationality::Colombian),
        17 => Ok(Nationality::CostaRican),
        18 => Ok(Nationality::Croatian),
        19 => Ok(Nationality::Cypriot),
        20 => Ok(Nationality::Czech),
        21 => Ok(Nationality::Danish),
        22 => Ok(Nationality::Dutch),
        23 => Ok(Nationality::Ecuadorian),
        24 => Ok(Nationality::English),
        25 => Ok(Nationality::Emirian),
        26 => Ok(Nationality::Estonian),
        27 => Ok(Nationality::Finnish),
        28 => Ok(Nationality::French),
        29 => Ok(Nationality::German),
        30 => Ok(Nationality::Ghanaian),
        31 => Ok(Nationality::Greek),
        32 => Ok(Nationality::Guatemalan),
        33 => Ok(Nationality::Honduran),
        34 => Ok(Nationality::HongKonger),
        35 => Ok(Nationality::Hungarian),
        36 => Ok(Nationality::Icelander),
        37 => Ok(Nationality::Indian),
        38 => Ok(Nationality::Indonesian),
        39 => Ok(Nationality::Irish),
        40 => Ok(Nationality::Israeli),
        41 => Ok(Nationality::Italian),
        42 => Ok(Nationality::Jamaican),
        43 => Ok(Nationality::Japanese),
        44 => Ok(Nationality::Jordanian),
        45 => Ok(Nationality::Kuwaiti),
        46 => Ok(Nationality::Latvian),
        47 => Ok(Nationality::Lebanese),
        48 => Ok(Nationality::Lithuanian),
        49 => Ok(Nationality::Luxembourger),
        50 => Ok(Nationality::Malaysian),
        51 => Ok(Nationality::Maltese),
        52 => Ok(Nationality::Mexican),
        53 => Ok(Nationality::Monegasque),
        54 => Ok(Nationality::NewZealander),
        55 => Ok(Nationality::Nicaraguan),
        56 => Ok(Nationality::NorthKorean),
        57 => Ok(Nationality::NorthernIrish),
        58 => Ok(Nationality::Norwegian),
        59 => Ok(Nationality::Omani),
        60 => Ok(Nationality::Pakistani),
        61 => Ok(Nationality::Panamanian),
        62 => Ok(Nationality::Paraguayan),
        63 => Ok(Nationality::Peruvian),
        64 => Ok(Nationality::Polish),
        65 => Ok(Nationality::Portuguese),
        66 => Ok(Nationality::Qatari),
        67 => Ok(Nationality::Romanian),
        68 => Ok(Nationality::Russian),
        69 => Ok(Nationality::Salvadoran),
        70 => Ok(Nationality::Saudi),
        71 => Ok(Nationality::Scottish),
        72 => Ok(Nationality::Serbian),
        73 => Ok(Nationality::Singaporean),
        74 => Ok(Nationality::Slovakian),
        75 => Ok(Nationality::Slovenian),
        76 => Ok(Nationality::SouthKorean),
        77 => Ok(Nationality::SouthAfrican),
        78 => Ok(Nationality::Spanish),
        79 => Ok(Nationality::Swedish),
        80 => Ok(Nationality::Swiss),
        81 => Ok(Nationality::Thai),
        82 => Ok(Nationality::Turkish),
        83 => Ok(Nationality::Uruguayan),
        84 => Ok(Nationality::Ukrainian),
        85 => Ok(Nationality::Venezuelan),
        86 => Ok(Nationality::Welsh),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode nationality.",
        )),
    }
}

fn decode_name(cursor: &mut Cursor<&mut BytesMut>) -> String {
    let cursor_position = cursor.position();
    let mut letters = Vec::with_capacity(48);

    for _ in 0..48 {
        let letter = cursor.get_u8() as char;

        if letter == '\0' {
            break;
        } else {
            letters.push(letter);
        }
    }

    cursor.set_position(cursor_position + 48);
    letters.iter().collect()
}

fn decode_telemetry_privacy(
    cursor: &mut Cursor<&mut BytesMut>,
) -> Result<Option<TelemetryPrivacy>, Error> {
    let value = cursor.get_u8();

    match value {
        0 => Ok(Some(TelemetryPrivacy::Restricted)),
        1 => Ok(Some(TelemetryPrivacy::Public)),
        _ => Err(Error::new(
            ErrorKind::InvalidData,
            "Failed to decode telemetry privacy setting.",
        )),
    }
}

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use bytes::{BufMut, BytesMut};

    use crate::nineteen::participants::{decode_name, decode_participants, PACKET_SIZE};
    use crate::packet::participants::{Controller, Driver, Nationality, Team, TelemetryPrivacy};

    fn put_packet_header(mut bytes: BytesMut) -> BytesMut {
        bytes.put_u16_le(2019);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(4);
        bytes.put_u64_le(u64::max_value());
        bytes.put_f32_le(1.0);
        bytes.put_u32_le(u32::max_value());
        bytes.put_u8(0);

        bytes
    }

    #[test]
    fn decode_participants_with_error() {
        let mut bytes = BytesMut::with_capacity(0);
        let mut cursor = Cursor::new(&mut bytes);

        let packet = decode_participants(&mut cursor);
        assert!(packet.is_err());
    }

    #[test]
    fn decode_participants_with_success() {
        let bytes = BytesMut::with_capacity(PACKET_SIZE);
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(20);

        for _ in 0..20 {
            bytes.put_u8(1);
            bytes.put_u8(2);
            bytes.put_u8(3);
            bytes.put_u8(4);
            bytes.put_u8(5);
            bytes.put_u8(b'P');
            bytes.put_u8(b'l');
            bytes.put_u8(b'a');
            bytes.put_u8(b'y');
            bytes.put_u8(b'e');
            bytes.put_u8(b'r');
            bytes.put_u8(0);

            let padding = vec![0u8; 41];
            bytes.put(padding.as_slice());

            bytes.put_u8(0);
        }

        let mut cursor = Cursor::new(&mut bytes);
        let packet = decode_participants(&mut cursor).unwrap();

        assert_eq!(20, packet.active_participants_count());

        let participant = &packet.participants()[0];

        assert_eq!(Controller::AI, participant.controller());
        assert_eq!(Driver::DanielRicciardo, participant.driver());
        assert_eq!(Team::Williams, participant.team());
        assert_eq!(4, participant.race_number());
        assert_eq!(Nationality::Azerbaijani, participant.nationality());
        assert_eq!(String::from("Player"), *participant.name());
        assert_eq!(
            TelemetryPrivacy::Restricted,
            participant.telemetry_privacy().unwrap()
        );
    }

    #[test]
    fn decode_short_name() {
        let mut bytes = BytesMut::with_capacity(48);

        bytes.put_u8(b'N');
        bytes.put_u8(b'a');
        bytes.put_u8(b'm');
        bytes.put_u8(b'e');
        bytes.put_u8(0);

        let mut cursor = Cursor::new(&mut bytes);

        let name = decode_name(&mut cursor);

        assert_eq!(String::from("Name"), name);
        assert_eq!(48, cursor.position());
    }
}
