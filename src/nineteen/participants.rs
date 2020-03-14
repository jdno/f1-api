//! Packet with a list of all participants in a session

use crate::nineteen::PacketHeader;
use crate::packet::FromBytes;
use bytes::{Buf, BytesMut};
use std::convert::TryFrom;
use std::io::{Cursor, Error, ErrorKind};

/// Indicates whether a car is controlled by the AI or a human.
#[derive(Debug, PartialEq)]
pub enum Controller {
    Human = 0,
    AI = 1,
}

/// A list of all drivers in F1 2019.
#[derive(Debug, PartialEq)]
pub enum Driver {
    CarlosSainz = 0,
    DaniilKvyat = 1,
    DanielRicciardo = 2,
    KimiRaikkonen = 6,
    LewisHamilton = 7,
    MaxVerstappen = 9,
    NicoHulkenburg = 10,
    KevinMagnussen = 11,
    RomainGrosjean = 12,
    SebastianVettel = 13,
    SergioPerez = 14,
    ValtteriBottas = 15,
    LanceStroll = 19,
    ArronBarnes = 20,
    MartinGiles = 21,
    AlexMurray = 22,
    LucasRoth = 23,
    IgorCorreia = 24,
    SophieLevasseur = 25,
    JonasSchiffer = 26,
    AlainForest = 27,
    JayLetourneau = 28,
    EstoSaari = 29,
    YasarAtiyeh = 30,
    CallistoCalabresi = 31,
    NaotaIzum = 32,
    HowardClarke = 33,
    WilheimKaufmann = 34,
    MarieLaursen = 35,
    FlavioNieves = 36,
    PeterBelousov = 37,
    KlimekMichalski = 38,
    SantiagoMoreno = 39,
    BenjaminCoppens = 40,
    NoahVisser = 41,
    GertWaldmuller = 42,
    JulianQuesada = 43,
    DanielJones = 44,
    ArtemMarkelov = 45,
    TadasukeMakino = 46,
    SeanGelael = 47,
    NyckDeVries = 48,
    JackAitken = 49,
    GeorgeRussell = 50,
    MaximilianGunther = 51,
    NireiFukuzumi = 52,
    LucaGhiotto = 53,
    LandoNorris = 54,
    SergioSetteCamara = 55,
    LouisDeletraz = 56,
    AntonioFuoco = 57,
    CharlesLeclerc = 58,
    PierreGasly = 59,
    AlexanderAlbon = 62,
    NicholasLatifi = 63,
    DorianBoccolacci = 64,
    NikoKari = 65,
    RobertoMerhi = 66,
    ArjunMaini = 67,
    AlessioLorandi = 68,
    RubenMeijer = 69,
    RashidNair = 70,
    JackTremblay = 71,
    AntonioGiovinazzi = 74,
    RobertKubica = 75,
    NobuharuMatsushita = 78,
    NikitaMazepin = 79,
    GuanyaZhou = 80,
    MickSchumacher = 81,
    CallumIlott = 82,
    JuanManuelCorrea = 83,
    JordanKing = 84,
    MahaveerRaghunathan = 85,
    TatianaCalderon = 86,
    AnthoineHubert = 87,
    GuilianoAlesi = 88,
    RalphBoschung = 89,
}

/// A list of all teams in F1 2019.
#[derive(Debug, PartialEq)]
pub enum Team {
    Mercedes = 0,
    Ferrari = 1,
    RedBullRacing = 2,
    Williams = 3,
    RacingPoint = 4,
    Renault = 5,
    ToroRosso = 6,
    Haas = 7,
    McLaren = 8,
    AlfaRomeo = 9,
    McLaren1988 = 10,
    McLaren1991 = 11,
    Williams1992 = 12,
    Ferrari1995 = 13,
    Williams1996 = 14,
    McLaren1998 = 15,
    Ferrari2002 = 16,
    Ferrari2004 = 17,
    Renault2006 = 18,
    Ferrari2007 = 19,
    RedBull2010 = 21,
    Ferrari1976 = 22,
    ARTGrandPrix = 23,
    CamposVexatecRacing = 24,
    Carlin = 25,
    CharouzRacingSystem = 26,
    DAMS = 27,
    RussianTime = 28,
    MPMotorsport = 29,
    Pertamina = 30,
    McLaren1990 = 31,
    Trident = 32,
    BWTArden = 33,
    McLaren1976 = 34,
    Lotus1972 = 35,
    Ferrari1979 = 36,
    McLaren1982 = 37,
    Williams2003 = 38,
    Brawn2009 = 39,
    Lotus1978 = 40,
    ArtGP2019 = 42,
    Campos2019 = 43,
    Carlin2019 = 44,
    SauberJuniorCharouz2019 = 45,
    Dams2019 = 46,
    UniVirtuosi2019 = 47,
    MPMotorsport2019 = 48,
    Prema2019 = 49,
    Trident2019 = 50,
    Arden2019 = 51,
    Ferrari1990 = 63,
    McLaren2010 = 64,
    Ferrari2010 = 65,
}

/// A list of all nationalities in F1 2019.
#[derive(Debug, PartialEq)]
pub enum Nationality {
    American = 1,
    Argentinean = 2,
    Australian = 3,
    Austrian = 4,
    Azerbaijani = 5,
    Bahraini = 6,
    Belgian = 7,
    Bolivian = 8,
    Brazilian = 9,
    British = 10,
    Bulgarian = 11,
    Cameroonian = 12,
    Canadian = 13,
    Chilean = 14,
    Chinese = 15,
    Colombian = 16,
    CostaRican = 17,
    Croatian = 18,
    Cypriot = 19,
    Czech = 20,
    Danish = 21,
    Dutch = 22,
    Ecuadorian = 23,
    English = 24,
    Emirian = 25,
    Estonian = 26,
    Finnish = 27,
    French = 28,
    German = 29,
    Ghanaian = 30,
    Greek = 31,
    Guatemalan = 32,
    Honduran = 33,
    HongKonger = 34,
    Hungarian = 35,
    Icelander = 36,
    Indian = 37,
    Indonesian = 38,
    Irish = 39,
    Israeli = 40,
    Italian = 41,
    Jamaican = 42,
    Japanese = 43,
    Jordanian = 44,
    Kuwaiti = 45,
    Latvian = 46,
    Lebanese = 47,
    Lithuanian = 48,
    Luxembourger = 49,
    Malaysian = 50,
    Maltese = 51,
    Mexican = 52,
    Monegasque = 53,
    NewZealander = 54,
    Nicaraguan = 55,
    NorthKorean = 56,
    NorthernIrish = 57,
    Norwegian = 58,
    Omani = 59,
    Pakistani = 60,
    Panamanian = 61,
    Paraguayan = 62,
    Peruvian = 63,
    Polish = 64,
    Portuguese = 65,
    Qatari = 66,
    Romanian = 67,
    Russian = 68,
    Salvadoran = 69,
    Saudi = 70,
    Scottish = 71,
    Serbian = 72,
    Singaporean = 73,
    Slovakian = 74,
    Slovenian = 75,
    SouthKorean = 76,
    SouthAfrican = 77,
    Spanish = 78,
    Swedish = 79,
    Swiss = 80,
    Thai = 81,
    Turkish = 82,
    Uruguayan = 83,
    Ukrainian = 84,
    Venezuelan = 85,
    Welsh = 86,
}

/// Only the player's telemetry data is broadcast over UDP. Telemetry data of
/// other cars is restricted to prevent players gaining an unfair advantage.
#[derive(Debug, PartialEq)]
pub enum UdpSetting {
    Restricted = 0,
    Public = 1,
}

/// A participant in an F1 2019 session.
///
/// F1 2019 publishes information about each participant in a session, including their name, team,
/// and whether they are a human or AI driver.
pub struct Participant {
    /// Each car can be controlled by an AI or a human.
    pub controller: Controller,

    /// The driver of the car.
    pub driver: Driver,

    /// The team for the car.
    pub team: Team,

    /// The race number of the car.
    pub race_number: u8,

    /// The driver's nationality.
    pub nationality: Nationality,

    /// The driver's name. In single player sessions, the name matches the driver. In multi player
    /// sessions, the player's Steam ID or LAN name is used.
    pub name: String,

    /// The participant's UDP setting.
    pub telemetry: UdpSetting,
}

pub struct ParticipantsPacket {
    /// Each packet starts with a packet header.
    pub header: PacketHeader,

    /// A list of participants in the race.
    pub participants: Vec<Participant>,
}

impl TryFrom<u8> for Controller {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Controller::Human),
            1 => Ok(Controller::AI),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode controller.",
            )),
        }
    }
}

impl TryFrom<u8> for Driver {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
}

impl TryFrom<u8> for Team {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
}

impl TryFrom<u8> for Nationality {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
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
}

impl TryFrom<u8> for UdpSetting {
    type Error = Error;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(UdpSetting::Restricted),
            1 => Ok(UdpSetting::Public),
            _ => Err(Error::new(
                ErrorKind::InvalidData,
                "Failed to decode UDP setting.",
            )),
        }
    }
}

impl ParticipantsPacket {
    /// Decode a driver's name.
    ///
    /// F1 2019 publishes the name of each driver as a null-terminated string with a maximum length
    /// of 40 bytes. This method reads each character until the null byte, and returns them as a
    /// string.
    fn decode_name(cursor: &mut Cursor<&mut BytesMut>) -> String {
        let mut letters = Vec::with_capacity(40);

        for _ in 0..40 {
            let letter = cursor.get_u8() as char;

            if letter == '\0' {
                break;
            } else {
                letters.push(letter);
            }
        }

        letters.iter().collect()
    }
}

impl FromBytes for ParticipantsPacket {
    fn buffer_size() -> usize {
        1104
    }

    fn decode(cursor: &mut Cursor<&mut BytesMut>) -> Result<Self, Error>
    where
        Self: Sized,
    {
        let header = PacketHeader::decode(cursor)?;
        let participants_count = cursor.get_u8();

        let mut participants = Vec::with_capacity(participants_count as usize);

        for _ in 0..participants_count {
            participants.push(Participant {
                controller: Controller::try_from(cursor.get_u8())?,
                driver: Driver::try_from(cursor.get_u8())?,
                team: Team::try_from(cursor.get_u8())?,
                race_number: cursor.get_u8(),
                nationality: Nationality::try_from(cursor.get_u8())?,
                name: Self::decode_name(cursor),
                telemetry: UdpSetting::try_from(cursor.get_u8())?,
            })
        }

        Ok(ParticipantsPacket {
            header,
            participants,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::nineteen::participants::{
        Controller, Driver, Nationality, ParticipantsPacket, Team, UdpSetting,
    };
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
    fn decode_name() {
        let mut bytes = BytesMut::with_capacity(5);

        bytes.put_u8(b'N');
        bytes.put_u8(b'a');
        bytes.put_u8(b'm');
        bytes.put_u8(b'e');
        bytes.put_u8(0);

        let mut cursor = Cursor::new(&mut bytes);

        let name = ParticipantsPacket::decode_name(&mut cursor);
        assert_eq!(String::from("Name"), name);
    }

    #[test]
    fn from_bytes_with_one_participant() {
        let bytes = BytesMut::with_capacity(ParticipantsPacket::buffer_size());
        let mut bytes = put_packet_header(bytes);

        bytes.put_u8(1);
        bytes.put_u8(0);
        bytes.put_u8(1);
        bytes.put_u8(2);
        bytes.put_u8(3);
        bytes.put_u8(4);
        bytes.put_u8(b'P');
        bytes.put_u8(b'l');
        bytes.put_u8(b'a');
        bytes.put_u8(b'y');
        bytes.put_u8(b'e');
        bytes.put_u8(b'r');
        bytes.put_u8(0);
        let padding = vec![0u8; 33];
        bytes.put(padding.as_slice());
        bytes.put_u8(0);
        let padding = vec![0u8; 1034];
        bytes.put(padding.as_slice());

        let mut cursor = Cursor::new(&mut bytes);
        let packet = ParticipantsPacket::from_bytes(&mut cursor).unwrap();

        assert_eq!(1, packet.participants.len());
        let participant = &packet.participants[0];

        assert_eq!(Controller::Human, participant.controller);
        assert_eq!(Driver::DaniilKvyat, participant.driver);
        assert_eq!(Team::RedBullRacing, participant.team);
        assert_eq!(3, participant.race_number);
        assert_eq!(Nationality::Austrian, participant.nationality);
        assert_eq!(String::from("Player"), participant.name);
        assert_eq!(UdpSetting::Restricted, participant.telemetry);
    }
}
