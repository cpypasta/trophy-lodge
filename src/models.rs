use strum::VariantArray;
use strum_macros::{EnumIter, VariantArray};
use std::fmt;
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

#[derive(PartialEq, Debug, Clone, Copy, EnumIter, VariantArray)]
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

#[derive(PartialEq, Debug, Clone, Copy, EnumIter)]
pub enum SortBy {
    Date,
    Score,
    Weight,
    Rating,
}
impl fmt::Display for SortBy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let model =  fmt_model(&self);
        write!(f, "{}", model)
    }
}

#[derive(Debug, VariantArray, Clone)]
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

#[derive(Debug, Clone)]
pub struct Trophy<'a> {
    pub species: Species,
    pub reserve: Reserves,
    pub rating: Ratings,
    pub score: f32,
    pub weight: f32,
    pub fur: &'a str,
    pub date: &'a str,
    pub gender: Gender,
}

impl Default for Trophy<'_> {
    fn default() -> Self {
        Trophy {
            species: random_enum(Species::VARIANTS.iter()).to_owned(),
            reserve: random_enum(Reserves::VARIANTS.iter()).to_owned(),
            rating: random_enum(Ratings::VARIANTS.iter()).to_owned(),
            score: random_f32(),
            weight: random_f32(),
            date: "2021-01-01",
            fur: "Dark",
            gender: random_enum(Gender::VARIANTS.iter()).to_owned(),
        }
    }
}