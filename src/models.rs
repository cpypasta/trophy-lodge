use strum::VariantArray;
use strum_macros::{EnumIter, VariantArray};
use std::fmt;
use std::cmp::{Ord, Ordering};
use convert_case::{Case, Casing};
use rand::prelude::*;

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

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray)]
pub enum Species {
    All,
    RedDeer,
    RoeDeer,
    FallowDeer,
    WildBoar,
    EuropeanBison,
    Moose,
    Reindeer,
}
impl fmt::Display for Species {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray)]
pub enum Reserves {
    All,
    Hirschfelden,
    LaytonLake,
    MedvedTaiga,
    VurhongaSavannah,
    ParqueFernando,
    YukonValley,
    SilverRidgePeaks,
    CuatroColinas,
    TeAwaroa,   
}
impl fmt::Display for Reserves {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(PartialEq, Eq, Debug, Clone, Copy, EnumIter, VariantArray)]
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

#[derive(Debug, VariantArray, Clone, Copy)]
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

#[derive(Debug, Clone, Copy)]
pub struct Trophy<'a> {
    pub species: Species,
    pub reserve: Reserves,
    pub rating: Ratings,
    pub score: f32,
    pub weight: f32,
    pub fur: &'a str,
    pub date: &'a str,
    pub gender: Gender,
    pub cash: i32,
    pub xp: i32,
    pub session_score: i32,
    pub integrity: bool,
    pub weapon_score: f32,
    pub shot_distance: f32,
    pub shot_damage: f32,
}

impl Default for Trophy<'_> {
    fn default() -> Self {
        Trophy {
            species: random_enum(Species::VARIANTS.iter()).to_owned(),
            reserve: random_enum(Reserves::VARIANTS.iter()).to_owned(),
            rating: random_enum(Ratings::VARIANTS.iter()).to_owned(),
            score: random_f32(),
            weight: random_f32(),
            date: "2021-01-01 12:00:00",
            fur: "Dark",
            gender: random_enum(Gender::VARIANTS.iter()).to_owned(),
            cash: 100,
            xp: 200,
            session_score: 300,
            integrity: true,
            weapon_score: random_f32(),
            shot_distance: random_f32(),
            shot_damage: random_f32(),
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

#[derive(PartialOrd, PartialEq, Eq, VariantArray, Debug, EnumIter)]
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
    WeaponScore,
    ShotDistance,
    ShotDamage,
}
impl fmt::Display for TrophyCols {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}
impl std::str::FromStr for TrophyCols {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Species" => Ok(TrophyCols::Species),
            "Reserve" => Ok(TrophyCols::Reserve),
            "Rating" => Ok(TrophyCols::Rating),
            "Score" => Ok(TrophyCols::Score),
            "Weight" => Ok(TrophyCols::Weight),
            "Fur" => Ok(TrophyCols::Fur),
            "Gender" => Ok(TrophyCols::Gender),
            "Date" => Ok(TrophyCols::Date),
            "Cash" => Ok(TrophyCols::Cash),
            "Xp" => Ok(TrophyCols::XP),
            "Session Score" => Ok(TrophyCols::SessionScore),
            "Integrity" => Ok(TrophyCols::Integrity),
            "Weapon Score" => Ok(TrophyCols::WeaponScore),
            "Shot Distance" => Ok(TrophyCols::ShotDistance),
            "Shot Damage" => Ok(TrophyCols::ShotDamage),
            _ => Err(()),
        }
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
        TrophyCols::WeaponScore => 12,
        TrophyCols::ShotDistance => 13,
        TrophyCols::ShotDamage => 14,
    }
}
impl Ord for TrophyCols {
    fn cmp(&self, other: &Self) -> Ordering {
        let x = trophy_col_order(self);
        let y = trophy_col_order(other);
        x.cmp(&y)
    }
}