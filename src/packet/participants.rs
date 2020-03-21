//! Data about all participants in a session
//!
//! The F1 games provide information about each participant in a session, for example their name,
//! team, and nationality. The data is updated every 5 seconds.

use crate::packet::header::Header;
use derive_new::new;
use getset::{CopyGetters, Getters};

/// Controller of a car
///
/// Cars can either be controlled by a human player or the AI.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Controller {
    AI,
    Human,
}

impl Default for Controller {
    fn default() -> Self {
        Controller::AI
    }
}

/// Drivers that appear in the F1 games
///
/// The F1 games feature a long list of drivers that appear in the games. Not every driver is
/// available in every game, and some drivers might be in a F2 championship in one game, and in F1
/// in the next.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Driver {
    AlainForest,
    AlessioLorandi,
    AlexMurray,
    AlexanderAlbon,
    AnthoineHubert,
    AntonioFuoco,
    AntonioGiovinazzi,
    ArjunMaini,
    ArronBarnes,
    ArtemMarkelov,
    BenjaminCoppens,
    CallistoCalabresi,
    CallumIlott,
    CarlosSainz,
    CharlesLeclerc,
    DanielJones,
    DanielRicciardo,
    DaniilKvyat,
    DorianBoccolacci,
    EstoSaari,
    FlavioNieves,
    GeorgeRussell,
    GertWaldmuller,
    GuanyaZhou,
    GuilianoAlesi,
    HowardClarke,
    IgorCorreia,
    JackAitken,
    JackTremblay,
    JayLetourneau,
    JonasSchiffer,
    JordanKing,
    JuanManuelCorrea,
    JulianQuesada,
    KevinMagnussen,
    KimiRaikkonen,
    KlimekMichalski,
    LanceStroll,
    LandoNorris,
    LewisHamilton,
    LouisDeletraz,
    LucaGhiotto,
    LucasRoth,
    MahaveerRaghunathan,
    MarieLaursen,
    MartinGiles,
    MaxVerstappen,
    MaximilianGunther,
    MickSchumacher,
    NaotaIzum,
    NicholasLatifi,
    NicoHulkenburg,
    NikitaMazepin,
    NikoKari,
    NireiFukuzumi,
    NoahVisser,
    NobuharuMatsushita,
    NyckDeVries,
    PeterBelousov,
    PierreGasly,
    RalphBoschung,
    RashidNair,
    RobertKubica,
    RobertoMerhi,
    RomainGrosjean,
    RubenMeijer,
    SantiagoMoreno,
    SeanGelael,
    SebastianVettel,
    SergioPerez,
    SergioSetteCamara,
    SophieLevasseur,
    TadasukeMakino,
    TatianaCalderon,
    ValtteriBottas,
    WilheimKaufmann,
    YasarAtiyeh,
}

impl Default for Driver {
    fn default() -> Self {
        // Open a PR to change this and I will block you!
        Driver::NicoHulkenburg
    }
}

/// Teams that appear in the F1 games
///
/// The F1 games feature a long list of teams that appear in the games, with some teams only being
/// available in certain games.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Team {
    ARTGrandPrix,
    AlfaRomeo,
    Arden2019,
    ArtGP2019,
    BWTArden,
    Brawn2009,
    Campos2019,
    CamposVexatecRacing,
    Carlin,
    Carlin2019,
    CharouzRacingSystem,
    DAMS,
    Dams2019,
    Ferrari,
    Ferrari1976,
    Ferrari1979,
    Ferrari1990,
    Ferrari1995,
    Ferrari2002,
    Ferrari2004,
    Ferrari2007,
    Ferrari2010,
    Haas,
    Lotus1972,
    Lotus1978,
    MPMotorsport,
    MPMotorsport2019,
    McLaren,
    McLaren1976,
    McLaren1982,
    McLaren1988,
    McLaren1990,
    McLaren1991,
    McLaren1998,
    McLaren2010,
    Mercedes,
    Pertamina,
    Prema2019,
    RacingPoint,
    RedBull2010,
    RedBullRacing,
    Renault,
    Renault2006,
    RussianTime,
    SauberJuniorCharouz2019,
    ToroRosso,
    Trident,
    Trident2019,
    UniVirtuosi2019,
    Williams,
    Williams1992,
    Williams1996,
    Williams2003,
}

impl Default for Team {
    fn default() -> Self {
        // We don't really have a choice in the hybrid area.
        Team::Mercedes
    }
}

/// Nationalities that appear in the F1 games
///
/// The F1 games feature a long list of drivers and teams, all of which have different
/// nationalities.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum Nationality {
    American,
    Argentinean,
    Australian,
    Austrian,
    Azerbaijani,
    Bahraini,
    Belgian,
    Bolivian,
    Brazilian,
    British,
    Bulgarian,
    Cameroonian,
    Canadian,
    Chilean,
    Chinese,
    Colombian,
    CostaRican,
    Croatian,
    Cypriot,
    Czech,
    Danish,
    Dutch,
    Ecuadorian,
    Emirian,
    English,
    Estonian,
    Finnish,
    French,
    German,
    Ghanaian,
    Greek,
    Guatemalan,
    Honduran,
    HongKonger,
    Hungarian,
    Icelander,
    Indian,
    Indonesian,
    Irish,
    Israeli,
    Italian,
    Jamaican,
    Japanese,
    Jordanian,
    Kuwaiti,
    Latvian,
    Lebanese,
    Lithuanian,
    Luxembourger,
    Malaysian,
    Maltese,
    Mexican,
    Monegasque,
    NewZealander,
    Nicaraguan,
    NorthKorean,
    NorthernIrish,
    Norwegian,
    Omani,
    Pakistani,
    Panamanian,
    Paraguayan,
    Peruvian,
    Polish,
    Portuguese,
    Qatari,
    Romanian,
    Russian,
    Salvadoran,
    Saudi,
    Scottish,
    Serbian,
    Singaporean,
    Slovakian,
    Slovenian,
    SouthAfrican,
    SouthKorean,
    Spanish,
    Swedish,
    Swiss,
    Thai,
    Turkish,
    Ukrainian,
    Uruguayan,
    Venezuelan,
    Welsh,
}

impl Default for Nationality {
    fn default() -> Self {
        // Greetings from the home of Hello Bits!
        Nationality::Dutch
    }
}

/// Privacy setting for telemetry data
///
/// In multiplayer sessions, only the player's telemetry data is broadcast over UDP. Telemetry data
/// of other cars is restricted to prevent players gaining an unfair advantage.
#[derive(Debug, PartialEq, Copy, Clone, Eq, Ord, PartialOrd, Hash)]
pub enum TelemetryPrivacy {
    Public,
    Restricted,
}

impl Default for TelemetryPrivacy {
    fn default() -> Self {
        TelemetryPrivacy::Public
    }
}

/// Data about a participant in the session
///
/// The F1 games publish data for each participant in a session that identifies them. This data
/// includes the participant's name, team, and nationality among others.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Clone, Eq, Ord, PartialOrd, Hash, Default)]
pub struct Participant {
    /// Returns the type of controller.
    #[getset(get_copy = "pub")]
    controller: Controller,

    /// Returns the driver.
    #[getset(get_copy = "pub")]
    driver: Driver,

    /// Returns the participant's team.
    #[getset(get_copy = "pub")]
    team: Team,

    /// Returns the number of the participant's car.
    #[getset(get_copy = "pub")]
    race_number: u8,

    /// Returns the participant's nationality.
    #[getset(get_copy = "pub")]
    nationality: Nationality,

    /// Returns the participant's name.
    ///
    /// In single player sessions, the AI is always named after the driver. In multiplayer sessions
    /// on PC, a player's SteamID or LAN name is used. On PlayStation, the LAN name is used. On
    /// Xbox, the driver name is always used.
    #[getset(get = "pub")]
    name: String,

    /// Returns the privacy setting for the participant's telemetry data.
    #[getset(get_copy = "pub")]
    telemetry_privacy: Option<TelemetryPrivacy>,
}

/// Packet containing information about each participant in the session
///
/// The F1 games provide information about each participant in a session, for example their name,
/// team, and nationality. The data is updated every 5 seconds.
#[derive(new, Debug, CopyGetters, Getters, PartialEq, Clone, Eq, Ord, PartialOrd, Hash)]
pub struct ParticipantsPacket {
    /// Returns the packet header prefixing the participants packet.
    #[getset(get = "pub")]
    header: Header,

    /// Returns the number of active participant in the session.
    ///
    /// The number of active participants in the packet should match the number of cars on the HUD
    /// in-game.
    #[getset(get_copy = "pub")]
    active_participants_count: u8,

    /// Returns the participants in the session.
    ///
    /// As is the case in other packets, the participants packet always contain 20 entries. This is
    /// also the case when there are less then 20 active participants in the session.
    #[getset(get = "pub")]
    participants: Vec<Participant>,
}
