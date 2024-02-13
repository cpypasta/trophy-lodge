use strum_macros::{EnumIter, VariantArray, EnumString};
use std::fmt;
use std::cmp::{Ord, Ordering};
use convert_case::{Case, Casing};
use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use egui::{ImageSource, include_image};

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

pub fn species_image(species: &Species) -> ImageSource {
    match species {
        Species::AmericanAlligator => include_image!("../static/species/Alligator_frame.png"),
        Species::AntelopeJackrabbit => include_image!("../static/species/Antilopejackrabbit_frame.png"),
        Species::AxisDeer => include_image!("../static/species/Axisdeer_frame.png"),
        Species::Banteng => include_image!("../static/species/Banteng_frame.png"),
        Species::BeciteIbex => include_image!("../static/species/Beceiteibex_frame.png"),
        Species::BighornSheep => include_image!("../static/species/Bighornsheep_frame.png"),
        Species::BlackBear => include_image!("../static/species/Blackbear_frame.png"),
        Species::Blackbuck => include_image!("../static/species/Blackbuck_frame.png"),
        Species::BlackGrouse => include_image!("../static/species/BlackGrouse_frame.png"),
        Species::BlacktailDeer => include_image!("../static/species/Blacktaildeer_frame.png"),
        Species::BlueWildebeest => include_image!("../static/species/Bluewildebeest_frame.png"),
        Species::Bobcat | Species::MexicanBobcat => include_image!("../static/species/Bobcat_frame.png"),
        Species::BrownBear => include_image!("../static/species/Brownbear_frame.png"),
        Species::CanadaGoose => include_image!("../static/species/Canadagoose_frame.png"),
        Species::CapeBuffalo => include_image!("../static/species/Capebuffalo_frame.png"),
        Species::Caribou => include_image!("../static/species/Caribou_frame.png"),
        Species::Chamois => include_image!("../static/species/Chamois_frame.png"),
        Species::CinnamonTeal => include_image!("../static/species/Cinnamonteal_frame.png"),
        Species::Coyote => include_image!("../static/species/Coyote_frame.png"),
        Species::SaltwaterCrocodile => include_image!("../static/species/Crocodile_frame.png"),
        Species::EasternCottontailRabbit | Species::EuropeanRabbit | Species::WhiteTailedJackrabbit => include_image!("../static/species/ECTrabbit_frame.png"),
        Species::EurasianTeal => include_image!("../static/species/EurasianTeal_frame.png"),
        Species::EurasianWigeon => include_image!("../static/species/EurasianWigeon_frame.png"),
        Species::EuropeanHare => include_image!("../static/species/Europeanhare_frame.png"),
        Species::FallowDeer => include_image!("../static/species/Fallowdeer_frame.png"),
        Species::FeralGoat => include_image!("../static/species/Feralgoat_frame.png"),
        Species::FeralPig => include_image!("../static/species/Feralpig_frame.png"),
        Species::Gemsbok => include_image!("../static/species/Gemsbok_frame.png"),
        Species::Goldeneye => include_image!("../static/species/Goldeneye_frame.png"),
        Species::GrayFox => include_image!("../static/species/Grayfox_frame.png"),
        Species::GredosIbex => include_image!("../static/species/Gredosibex_frame.png"),
        Species::GreylagGoose => include_image!("../static/species/GreylagGoose_frame.png"),
        Species::GrayWolf => include_image!("../static/species/Greywolf_frame.png"),
        Species::GrizzlyBear => include_image!("../static/species/Grizzlybear_frame.png"),
        Species::HarlequinDuck => include_image!("../static/species/Harlequinduck_frame.png"),
        Species::HazelGrouse => include_image!("../static/species/HazelGrouse_frame.png"),
        Species::HogDeer => include_image!("../static/species/HogDeer_frame.png"),
        Species::IberianMouflon => include_image!("../static/species/Iberianmufflon_frame.png"),
        Species::IberianWolf => include_image!("../static/species/Iberianwolf_frame.png"),
        Species::JavanRusa => include_image!("../static/species/JavanRusa_frame.png"),
        Species::EasternGrayKangaroo => include_image!("../static/species/Kangaroo_frame.png"),
        Species::LesserKudu => include_image!("../static/species/Lesserkudu_frame.png"),
        Species::Lion => include_image!("../static/species/Lion_frame.png"),
        Species::EurasianLynx => include_image!("../static/species/Lynx_frame.png"),
        Species::MagpieGoose => include_image!("../static/species/MagpieGoose_frame.png"),
        Species::Mallard => include_image!("../static/species/Mallard_frame.png"),
        Species::Moose => include_image!("../static/species/Moose_frame.png"),
        Species::MountainGoat => include_image!("../static/species/Mountaingoat_frame.png"),
        Species::MountainHare => include_image!("../static/species/MountainHare_frame.png"),
        Species::MuleDeer => include_image!("../static/species/Muledeer_frame.png"),
        Species::SiberianMuskDeer => include_image!("../static/species/Muskdeer_frame.png"),
        Species::CollaredPeccary => include_image!("../static/species/Peccary_frame.png"),
        Species::RingNeckedPheasant => include_image!("../static/species/Pheasant_frame.png"),
        Species::EuropeanBison => include_image!("../static/species/Bison_frame.png"),
        Species::PlainsBison => include_image!("../static/species/Plainsbison_frame.png"),
        Species::Pronghorn => include_image!("../static/species/Pronghorn_frame.png"),
        Species::Puma => include_image!("../static/species/Puma_frame.png"),
        Species::BobwhiteQuail => include_image!("../static/species/Quail_frame.png"),
        Species::CommonRaccoon => include_image!("../static/species/Raccoon_frame.png"),
        Species::RaccoonDog => include_image!("../static/species/RaccoonDog_frame.png"),
        Species::RedDeer => include_image!("../static/species/Reddeer_frame.png"),
        Species::RedFox => include_image!("../static/species/Redfox_frame.png"),
        Species::Reindeer => include_image!("../static/species/Reindeer_frame.png"),
        Species::RoeDeer => include_image!("../static/species/Roedeer_frame.png"),
        Species::RondaIbex => include_image!("../static/species/Rondaibex_frame.png"),
        Species::RooseveltElk | Species::RockymountainElk => include_image!("../static/species/Rooseveltelk_frame.png"),
        Species::Sambar => include_image!("../static/species/Sambar_frame.png"),
        Species::ScrubHare => include_image!("../static/species/Scrubhare_frame.png"),
        Species::SideStripedJackal => include_image!("../static/species/Sidestripedjackal_frame.png"),
        Species::SikaDeer => include_image!("../static/species/Sikadeer_frame.png"),
        Species::SoutheasternSpanishIbex => include_image!("../static/species/Southeasternibex_frame.png"),
        Species::Springbok => include_image!("../static/species/Springbok_frame.png"),
        Species::StubbedQuail => include_image!("../static/species/StubbleQuail_frame.png"),
        Species::TuftedDuck => include_image!("../static/species/TuftedDuck_frame.png"),
        Species::TundraBeanGoose => include_image!("../static/species/TundraBeanGoose_frame.png"),
        Species::MerriamTurkey | Species::RioGrandeTurkey | Species::EasternWildTurkey => include_image!("../static/species/Turkey_frame.png"),
        Species::GreenWingedTeal => include_image!("../static/species/Cinnamonteal_frame.png"),
        Species::Warthog => include_image!("../static/species/Warthog_frame.png"),
        Species::WaterBuffalo => include_image!("../static/species/Waterbuffalo_frame.png"),
        Species::WesternCapercaillie => include_image!("../static/species/WesternCapercaillie_frame.png"),
        Species::WhitetailDeer => include_image!("../static/species/Whitetaildeer_frame.png"),
        Species::WildBoar => include_image!("../static/species/Wildboar_frame.png"),
        Species::WillowPtarmigan | Species::RockPtarmigan => include_image!("../static/species/WillowPtarmigan_frame.png"),
        _ => include_image!("../static/species/unknown.png"),
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

pub fn reserve_image(reserve: &Reserve) -> ImageSource {
    match reserve {
        Reserve::HirschfeldenHuntingReserve => include_image!("../static/reserves/hirschfelden_hunting_reserve.png"),
        Reserve::LaytonLakeDistrict => include_image!("../static/reserves/layton_lake_district.png"),
        Reserve::MedvedTaigaNationalPark => include_image!("../static/reserves/medved_taiga_national_park.png"),
        Reserve::VurhongaSavanna => include_image!("../static/reserves/vurhonga_savanna.png"),
        Reserve::ParqueFernando => include_image!("../static/reserves/parque_fernando.png"),
        Reserve::YukonValleyNatureReserve => include_image!("../static/reserves/yukon_valley.png"),
        Reserve::CuatroColinasGameReserve => include_image!("../static/reserves/cuatro.png"),
        Reserve::SilverRidgePeaks => include_image!("../static/reserves/silver_ridge.png"),
        Reserve::TeAwaroaNationalPark => include_image!("../static/reserves/te_awaroa.png"),
        Reserve::RanchoDelArroyo => include_image!("../static/reserves/rancho.png"),
        Reserve::MississippiAcresPreserve => include_image!("../static/reserves/mississippi.png"),
        Reserve::RevontuliCoast => include_image!("../static/reserves/revontuli.png"),
        Reserve::NewEnglandMountains => include_image!("../static/reserves/newengland.png"),
        Reserve::EmeraldCoastAustralia => include_image!("../static/reserves/emerald.png"),
        _ => include_image!("../static/reserves/unknown.png"),
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
    pub description: String,
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
            description: "".to_string(),
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
            kills: 1,
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

#[derive(Debug, Clone)]
pub struct ChallengeSummary {
    pub name: String,
    pub description: String,
    pub start: String,
    pub percent: f32,
}