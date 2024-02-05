use strum::VariantArray;
use strum_macros::{EnumIter, VariantArray, EnumString};
use std::fmt;
use std::cmp::{Ord, Ordering};
use convert_case::{Case, Casing};
use rand::prelude::*;
use serde::{Serialize, Deserialize};

fn fmt_model(value: &impl fmt::Debug) -> String {
    format!("{:?}", value).to_case(Case::Title)
}

fn random_enum<I, T>(items: I) -> T 
where I: Iterator<Item = T>,
      T: fmt::Display + Clone {
    let mut rng = rand::thread_rng();
    items.filter(|x| x.to_string() != "All").choose(&mut rng).unwrap()
}

fn random_f32() -> f32 {
    let mut rng = rand::thread_rng();
    let value = rng.gen_range(0.0..100.0) as f32;
    let scale = 100.0;
    let rounded = (value * scale).round() / scale;
    rounded
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "title_case")]
pub enum Species {
    All,
    AmericanAlligator,
    AntelopeJackrabbit,
    AxisDeer,
    BeciteIbex,
    BighornSheep,
    BlackBear,
    BlackGrouse,
    Blackbuck,
    BlacktailDeer,
    BlueWildebeest,
    Bobcat,
    CanadaGoose,
    CapeBuffalo,
    Caribou,
    Chamois,
    CinnamonTeal,
    CollaredPeccary,
    Coyote,
    EasternCottontailRabbit,
    EasternWildTurkey,
    EuropeanBison,
    EuropeanHare,
    EuropeanRabbit,
    EurasianBrownBear,
    EurasianTeal,
    EurasianWigeon,
    FallowDeer,
    FeralGoat,
    FeralPig,
    Gemsbok,
    Goldeneye,
    GrayFox,
    GrayWolf,
    GredosIbex,
    GreenWingTeal,
    GreylagGoose,
    GrizzlyBear,
    HarlequinDuck,
    HazelGrouse,
    IberianMouflon,
    IberianWolf,
    Jackrabbit,
    LesserKudu,
    Lion,
    Mallard,
    MexicanBobcat,
    Moose,
    MountainGoat,
    MountainHare,
    MountainLion,
    MuleDeer,
    NorthernBobwhiteQuail,
    Pheasant,
    PlainsBison,
    ProngHorn,
    Puma,
    Raccoon,
    RaccoonDog,
    RedDeer,
    RedFox,
    Reindeer,
    RioGrandeTurkey,
    RockPtarmigan,
    RockmountainElk,
    RoeDeer,
    RondaIbex,
    RooseveltElk,
    ScrubHare,
    SiberianMuskDeer,
    SidestripedJackal,
    SikaDeer,
    SoutheasternIbex,
    Springbok,
    TuftedDuck,
    TundraBeanGoose,
    Warthog,
    WaterBuffalo,
    WesternCapercaillie,
    WhitetailDeer,
    WildBoar,
    WildHog,
    WildTurkey,
    WillowPtarmigan,
    HogDeer,
    MagpieGoose,
    EasternKangaroo,
    SambarDeer,
    Banteng,
    SaltwaterCrocodile,
    StubbleQuail,
    JavanRusa,
    Unknown,
}
impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Serialize, Deserialize)]
#[strum(serialize_all = "title_case")]
pub enum Reserves {
    All,
    Hirschfelden,
    LaytonLake,
    MedvedTaigaNationalPark,
    VurhongaSavannah,
    ParqueFernando,
    YukonValley,
    CuatroColinasGameReserve,
    SilverRidgePeaks,
    TeAwaroaNationalPark,   
    RanchoDelArroyo,
    MississippiAcresPreserve,
    RevontuliCoast,
    NewEnglandMountains,
    EmeraldCoast,
    Unknown,
}
impl fmt::Display for Reserves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, EnumIter, VariantArray, EnumString, Deserialize, Serialize)]
#[strum(serialize_all = "title_case")]
pub enum Ratings {
    All,
    GreatOne,
    Diamond,
    Gold,
    Silver,
    Bronze,
    None,
}
impl fmt::Display for Ratings {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}
fn ratings_to_i32(rating: &Ratings) -> i32 {
    match rating {
        Ratings::GreatOne => 6,
        Ratings::Diamond => 5,
        Ratings::Gold => 4,
        Ratings::Silver => 3,
        Ratings::Bronze => 2,
        Ratings::None => 1,
        _ => 0,
    }
}
impl Ord for Ratings {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = ratings_to_i32(self);
        let y = &ratings_to_i32(other);
        x.cmp(y)
    }
}
impl PartialOrd for Ratings {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
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

#[derive(Debug, VariantArray, Clone, Copy, EnumString, Serialize, Deserialize)]
pub enum Gender {
    Male,
    Female,
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Trophy {
    pub id: f32,
    pub species: Species,
    pub reserve: Reserves,
    pub rating: Ratings,
    pub score: f32,
    pub weight: f32,
    pub fur: String,
    pub date: String,
    pub gender: Gender,
    pub cash: i32,
    pub xp: i32,
    pub session_score: i32,
    pub integrity: bool,
    pub tracking: f32,
    pub weapon_score: f32,
    pub shot_distance: f32,
    pub shot_damage: f32,
    pub mods: bool,
}

impl Default for Trophy {
    fn default() -> Self {
        Trophy {
            id: random_f32(),
            species: random_enum(Species::VARIANTS.iter()).to_owned(),
            reserve: random_enum(Reserves::VARIANTS.iter()).to_owned(),
            rating: random_enum(Ratings::VARIANTS.iter()).to_owned(),
            score: random_f32(),
            weight: random_f32(),
            date: "2021-01-01 12:00:00".to_string(),
            fur: "Dark".to_string(),
            gender: random_enum(Gender::VARIANTS.iter()).to_owned(),
            cash: 100,
            xp: 200,
            session_score: 300,
            integrity: true,
            tracking: random_f32(),
            weapon_score: random_f32(),
            shot_distance: random_f32(),
            shot_damage: random_f32(),
            mods: false,
        }
    }
}

pub struct TrophyFilter {
    pub species: Species,
    pub reserve: Reserves,
    pub rating: Ratings,
    pub sort_by: SortBy,
}
impl Default for TrophyFilter {
    fn default() -> Self {
        TrophyFilter {
            species: Species::All,
            reserve: Reserves::All,
            rating: Ratings::All,
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
    }
}
impl Ord for TrophyCols {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = trophy_col_order(self);
        let y = trophy_col_order(other);
        x.cmp(&y)
    }
}