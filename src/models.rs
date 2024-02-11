use strum_macros::{EnumIter, VariantArray, EnumString};
use std::fmt;
use std::cmp::{Ord, Ordering};
use convert_case::{Case, Casing};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;

fn fmt_model(value: &impl fmt::Debug) -> String {
    format!("{:?}", value).to_case(Case::Title)
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "title_case")]
pub enum Species {
    All,
    AmericanAlligator,
    AntelopeJackrabbit,
    AxisDeer,
    Banteng,
    BeciteIbex,
    BighornSheep,
    BlackBear,
    BlackGrouse,
    Blackbuck,
    BlacktailDeer,
    BlueWildebeest,
    Bobcat,
    BobwhiteQuail,
    BrownBear,
    CanadaGoose,
    CapeBuffalo,
    Caribou,
    Chamois,
    CinnamonTeal,
    CollaredPeccary,
    CommonRaccoon,
    Coyote,
    EasternCottontailRabbit,
    EasternGrayKangaroo,
    EasternWildTurkey,
    EurasianLynx,
    EurasianTeal,
    EurasianWigeon,
    EuropeanBison,
    EuropeanHare,
    EuropeanRabbit,
    FallowDeer,
    FeralGoat,
    FeralPig,
    Gemsbok,
    Goldeneye,
    GrayFox,
    GrayWolf,
    GredosIbex,
    GreenWingedTeal,
    GreylagGoose,
    GrizzlyBear,
    HarlequinDuck,
    HazelGrouse,
    HogDeer,
    IberianMouflon,
    IberianWolf,
    JavanRusa,
    LesserKudu,
    Lion,
    MagpieGoose,
    Mallard,
    MerriamTurkey,
    MexicanBobcat,
    Moose,
    MountainGoat,
    MountainHare,
    MuleDeer,
    PlainsBison,
    Pronghorn,
    Puma,    
    RaccoonDog,
    RedDeer,
    RedFox,
    Reindeer,
    RingNeckedPheasant,
    RioGrandeTurkey,
    RockPtarmigan,
    RockymountainElk,
    RoeDeer,
    RondaIbex,
    RooseveltElk,
    SaltwaterCrocodile,
    Sambar,
    ScrubHare,
    SiberianMuskDeer,
    SideStripedJackal,
    SikaDeer,
    SoutheasternSpanishIbex,
    Springbok,
    StubbedQuail,
    TuftedDuck,
    TundraBeanGoose,
    Warthog,
    WaterBuffalo,
    WesternCapercaillie,
    WhitetailDeer,
    WhiteTailedJackrabbit,
    WildBoar,
    WillowPtarmigan,
    Unknown,
}
impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Serialize, Deserialize, Hash, Eq)]
#[strum(serialize_all = "title_case")]
pub enum Reserve {
    All,
    HirschfeldenHuntingReserve,
    LaytonLakeDistrict,
    MedvedTaigaNationalPark,
    VurhongaSavanna,
    ParqueFernando,
    YukonValleyNatureReserve,
    CuatroColinasGameReserve,
    SilverRidgePeaks,
    TeAwaroaNationalPark,   
    RanchoDelArroyo,
    MississippiAcresPreserve,
    RevontuliCoast,
    NewEnglandMountains,
    EmeraldCoastAustralia,
    Unknown,
}

impl fmt::Display for Reserve {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

pub fn reserve_species() -> HashMap<Reserve, Vec<Species>> {
    let mut map = HashMap::new();
    map.insert(Reserve::HirschfeldenHuntingReserve, vec![
        Species::WildBoar,
        Species::EuropeanRabbit,
        Species::FallowDeer,
        Species::RedFox,
        Species::RingNeckedPheasant,
        Species::RedDeer,
    ]);
    map.insert(Reserve::LaytonLakeDistrict, vec![
        Species::Moose,
        Species::WhiteTailedJackrabbit,
        Species::Mallard,
        Species::EasternWildTurkey,
        Species::BlackBear,
        Species::RooseveltElk,
        Species::Coyote,
        Species::BlacktailDeer,
        Species::WhitetailDeer,
    ]);
    map.insert(Reserve::MedvedTaigaNationalPark, vec![
        Species::SiberianMuskDeer,
        Species::Moose,
        Species::WildBoar,
        Species::Reindeer,
        Species::EurasianLynx,
        Species::BrownBear,
        Species::WesternCapercaillie,
        Species::GrayWolf,
    ]);
    map.insert(Reserve::VurhongaSavanna, vec![
        Species::EurasianWigeon,
        Species::BlueWildebeest,
        Species::SideStripedJackal,
        Species::Gemsbok,
        Species::LesserKudu,
        Species::ScrubHare,
        Species::Lion,
        Species::Warthog,
        Species::CapeBuffalo,
        Species::Springbok,
    ]);
    map.insert(Reserve::ParqueFernando, vec![
        Species::RedDeer,
        Species::WaterBuffalo,
        Species::Puma,
        Species::Blackbuck,
        Species::CinnamonTeal,
        Species::CollaredPeccary,
        Species::MuleDeer,
        Species::AxisDeer,
    ]);
    map.insert(Reserve::YukonValleyNatureReserve, vec![
        Species::HarlequinDuck,
        Species::Moose,
        Species::RedFox,
        Species::Caribou,
        Species::CanadaGoose,
        Species::GrizzlyBear,
        Species::GrayWolf,
        Species::PlainsBison,
    ]);
    map.insert(Reserve::CuatroColinasGameReserve, vec![
        Species::SoutheasternSpanishIbex,
        Species::IberianWolf,
        Species::RedDeer,
        Species::IberianMouflon,
        Species::WildBoar,
        Species::BeciteIbex,
        Species::EuropeanHare,
        Species::RoeDeer,
        Species::RondaIbex,
        Species::RingNeckedPheasant,
        Species::GredosIbex,
    ]);
    map.insert(Reserve::SilverRidgePeaks, vec![
        Species::Pronghorn,
        Species::Puma,
        Species::MountainGoat,
        Species::BighornSheep,
        Species::EasternWildTurkey,
        Species::BlackBear,
        Species::MuleDeer,
        Species::RockymountainElk,
        Species::PlainsBison,
    ]);
    map.insert(Reserve::TeAwaroaNationalPark, vec![
        Species::RedDeer,
        Species::EuropeanRabbit,
        Species::FeralPig,
        Species::FallowDeer,
        Species::Chamois,
        Species::Mallard,
        Species::EasternWildTurkey,
        Species::SikaDeer,
        Species::FeralGoat,
    ]);
    map.insert(Reserve::RanchoDelArroyo, vec![
        Species::MexicanBobcat,
        Species::RioGrandeTurkey,
        Species::Pronghorn,
        Species::BighornSheep,
        Species::CollaredPeccary,
        Species::AntelopeJackrabbit,
        Species::MuleDeer,
        Species::Coyote,
        Species::RingNeckedPheasant,
        Species::WhitetailDeer,
    ]);
    map.insert(Reserve::MississippiAcresPreserve, vec![
        Species::FeralPig,
        Species::CommonRaccoon,
        Species::EasternCottontailRabbit,
        Species::BobwhiteQuail,
        Species::EasternWildTurkey,
        Species::GrayFox,
        Species::BlackBear,
        Species::AmericanAlligator,
        Species::GreenWingedTeal,
        Species::WhitetailDeer,
    ]);
    map.insert(Reserve::RevontuliCoast, vec![
        Species::Mallard,
        Species::RockPtarmigan,
        Species::EurasianWigeon,
        Species::Moose,
        Species::Goldeneye,
        Species::MountainHare,
        Species::TuftedDuck,
        Species::BlackGrouse,
        Species::TundraBeanGoose,
        Species::WillowPtarmigan,
        Species::EurasianLynx,
        Species::HazelGrouse,
        Species::BrownBear,
        Species::EurasianTeal,
        Species::WesternCapercaillie,
        Species::CanadaGoose,
        Species::GreylagGoose,
        Species::WhitetailDeer,
        Species::RaccoonDog,
    ]);
    map.insert(Reserve::NewEnglandMountains, vec![
        Species::Mallard,
        Species::Moose,
        Species::Goldeneye,
        Species::CommonRaccoon,
        Species::EasternCottontailRabbit,
        Species::BobwhiteQuail,
        Species::EasternWildTurkey,
        Species::GrayFox,
        Species::RedFox,
        Species::BlackBear,
        Species::Bobcat,
        Species::Coyote,
        Species::RingNeckedPheasant,
        Species::GreenWingedTeal,
        Species::WhitetailDeer,
    ]);
    map.insert(Reserve::EmeraldCoastAustralia, vec![
        Species::HogDeer,
        Species::FallowDeer,
        Species::RedDeer,
        Species::FeralPig,
        Species::MagpieGoose,
        Species::EasternGrayKangaroo,
        Species::RedFox,
        Species::Sambar,
        Species::Banteng,
        Species::SaltwaterCrocodile,
        Species::FeralGoat,
        Species::StubbedQuail,
        Species::AxisDeer,
        Species::JavanRusa,
    ]);
    map
}

#[derive(PartialOrd, PartialEq, Eq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Deserialize, Serialize)]
#[strum(serialize_all = "title_case")]
pub enum Rating {
    All,
    GreatOne,
    Diamond,
    Gold,
    Silver,
    Bronze,
    None,
    Unknown,
}
impl fmt::Display for Rating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}
fn ratings_to_i32(rating: &Rating) -> i32 {
    match rating {
        Rating::GreatOne => 6,
        Rating::Diamond => 5,
        Rating::Gold => 4,
        Rating::Silver => 3,
        Rating::Bronze => 2,
        Rating::None => 1,
        Rating::Unknown => 0,
        _ => 0,
    }
}
impl Ord for Rating {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = ratings_to_i32(self);
        let y = &ratings_to_i32(other);
        x.cmp(y)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Serialize, Deserialize)]
pub enum Weapon {
    Unknown,
}
impl fmt::Display for Weapon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum SortBy {
    Date,
    Score,
    Weight,
    Rating,
    ShotDistance,
}
impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(Debug, VariantArray, Clone, Copy, EnumString, Serialize, Deserialize, PartialEq, EnumIter)]
pub enum Gender {
    All,
    Male,
    Female,
    Unknown,
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, EnumIter, EnumString, PartialEq)]
pub enum Boolean {
    True,
    False,
}
impl fmt::Display for Boolean {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}
impl std::convert::From<bool> for Boolean {
    fn from(b: bool) -> Self {
        if b {
            Boolean::True
        } else {
            Boolean::False
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trophy {
    pub id: f32,
    pub species: Species,
    pub reserve: Reserve,
    pub rating: Rating,
    pub score: f32,
    pub weight: f32,
    pub fur: String,
    pub date: String,
    pub gender: Gender,
    pub cash: i32,
    pub xp: i32,
    pub session_score: i32,
    pub integrity: Boolean,
    pub tracking: f32,
    pub weapon_score: f32,
    pub shot_distance: f32,
    pub shot_damage: f32,
    pub mods: Boolean,
    pub grind: Option<String>,
}
impl Trophy {
    pub fn valid(&self) -> bool {
        self.species != Species::Unknown && self.reserve != Reserve::Unknown
    }
}

pub struct TrophyFilter {
    pub species: Species,
    pub reserve: Reserve,
    pub rating: Rating,
    pub gender: Gender,
    pub grind: String,
    pub sort_by: SortBy,
}
impl Default for TrophyFilter {
    fn default() -> Self {
        TrophyFilter {
            species: Species::All,
            reserve: Reserve::All,
            rating: Rating::All,
            gender: Gender::All,
            grind: "".to_string(),
            sort_by: SortBy::Date,
        }
    }
}

#[derive(PartialOrd, PartialEq, Eq, VariantArray, Debug, EnumIter, EnumString)]
#[strum(serialize_all = "title_case")]
pub enum TrophyCols {
    Species,
    Reserve,
    Rating,
    Score,
    Weight,
    Fur,
    Gender,
    Date,
    Cash,
    XP,
    SessionScore,
    Integrity,
    Tracking,
    WeaponScore,
    ShotDistance,
    ShotDamage,
    Mods,
    Grind,
}
impl fmt::Display for TrophyCols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}
fn trophy_col_order(col: &TrophyCols) -> i32 {
    match col {
        TrophyCols::Species => 0,
        TrophyCols::Reserve => 1,
        TrophyCols::Rating => 2,
        TrophyCols::Score => 3,
        TrophyCols::Weight => 4,
        TrophyCols::Fur => 5,
        TrophyCols::Gender => 6,
        TrophyCols::Date => 7,
        TrophyCols::Cash => 8,
        TrophyCols::XP => 9,
        TrophyCols::SessionScore => 10,
        TrophyCols::Integrity => 11,
        TrophyCols::Tracking => 12,
        TrophyCols::WeaponScore => 13,
        TrophyCols::ShotDistance => 14,
        TrophyCols::ShotDamage => 15,
        TrophyCols::Mods => 16,
        TrophyCols::Grind => 17,
    }
}
impl Ord for TrophyCols {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = trophy_col_order(self);
        let y = trophy_col_order(other);
        x.cmp(&y)
    }
}

#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct Grind {
    pub name: String,
    pub species: Species,
    pub reserve: Reserve,
    pub active: bool,
    pub start: String,
    pub kills: i64,
    pub is_deleted: bool,
}
impl Default for Grind {
    fn default() -> Self {
        Grind {
            name: rand::random::<u32>().to_string(),
            species: Species::RedDeer,
            reserve: Reserve::HirschfeldenHuntingReserve,
            active: false,
            start: chrono::Local::now().to_rfc3339(),
            kills: 69,
            is_deleted: false,
        }
    }
}
impl Grind {
    fn grind_exists(name: String, grinds: &Vec<Grind>) -> bool {
        for g in grinds {
            if g.name == name {
                return true;
            }
        }
        false
    }

    pub fn valid(&self, grinds: &Vec<Grind>) -> bool {
        self.name != "" && 
        self.species != Species::All && 
        self.reserve != Reserve::All && 
        self.species != Species::Unknown && 
        self.reserve != Reserve::Unknown &&
        !Grind::grind_exists(self.name.to_string(), grinds)
    }
}

pub struct GrindKill {
    pub name: String,
}

#[derive(Debug, Clone, Deserialize, Serialize, PartialEq)]
pub struct Challenge {
    pub name: String,
    pub species: Species,
    pub reserve: Reserve,
    pub rating: Rating,
    pub gender: Gender,
    pub weapon: Weapon,
    pub mods: Boolean,
    pub public: Boolean,
    pub shot_damage: u32,
    pub shot_distance: u32,
    pub tracking: u32,
    pub kills: u32,
    pub total_shots: u32,
    pub weight: f32,
    pub score: f32,
}
impl Default for Challenge {
    fn default() -> Self {
        Challenge {
            name: "".to_string(),
            species: Species::Unknown,
            reserve: Reserve::Unknown,
            rating: Rating::Unknown,
            gender: Gender::Unknown,
            weapon: Weapon::Unknown,
            mods: Boolean::False,
            public: Boolean::False,
            shot_damage: 0,
            shot_distance: 0,
            tracking: 0,
            kills: 0,
            total_shots: 0,
            weight: 0.0,
            score: 0.0,
        }
    }
}
impl PartialOrd for Challenge {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.reserve.to_string().cmp(&other.reserve.to_string()))
    }
}
impl Ord for Challenge {
    fn cmp(&self, other: &Self) -> Ordering {
        self.reserve.to_string().cmp(&other.reserve.to_string())
    }
}
impl Eq for Challenge {}