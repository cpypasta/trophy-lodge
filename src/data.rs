use crate::models::*;
use crate::challenges::*;
use std::path::Path;
use std::fs::{self, OpenOptions};
use std::sync::mpsc::Sender;
use serde::de::DeserializeOwned;
use serde::ser::Serialize;

const TROPHIES: &str = "./data/trophies.csv";
pub const GRINDS: &str = "./data/grinds.csv";
const CHALLENGES: &str = "./data/challenges";

pub fn init() {
    let trophy_path = Path::new(TROPHIES).parent().unwrap();
    let grind_path = Path::new(GRINDS).parent().unwrap();
    let challenge_path = Path::new(CHALLENGES);
    if !trophy_path.exists() {
        std::fs::create_dir_all(trophy_path).unwrap();
    }
    if !grind_path.exists() {
        std::fs::create_dir_all(grind_path).unwrap();
    }
    if !challenge_path.exists() {
        std::fs::create_dir_all(challenge_path).unwrap();
    }
}

fn read_csv<T: DeserializeOwned>(path: &str) -> Vec<T> {
    if !Path::new(path).exists() {
        return Vec::new();
    }

    let file = OpenOptions::new()
        .read(true)
        .open(path)
        .unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut items = Vec::new();
    for item in rdr.deserialize() {
        if let Ok(i) = item {
            items.push(i);
        }
    }
    items
}

pub fn create_csv<T: Serialize>(path: &str, items: Vec<T>) {
    if Path::new(path).exists() {
        std::fs::remove_file(path).unwrap();
    }

    if items.len() == 0 {
        return;
    }

    let file = OpenOptions::new()
        .create(true)
        .write(true)
        .open(path)
        .unwrap();
    let mut wtr = csv::WriterBuilder::new().has_headers(true).from_writer(file);
    for item in items {
        wtr.serialize(item).unwrap();
    }
    wtr.flush().unwrap();
}

fn append_csv<T: Serialize>(path: &str, items: Vec<T>) {
    let file_exists = Path::new(path).exists();
    if !file_exists {
        std::fs::File::create(path).unwrap();
    }
    let file = OpenOptions::new()
        .append(true)
        .open(path)
        .unwrap();
    let mut wtr = csv::WriterBuilder::new().has_headers(!file_exists).from_writer(file);
    for item in items {
        wtr.serialize(item).unwrap();
    }
    wtr.flush().unwrap();
}

pub fn read_trophies() -> Vec<Trophy> {
    read_csv(TROPHIES)
}

pub fn trophy_exists(trophy: &Trophy) -> bool {
    let trophies = read_trophies();
    for t in trophies {
        if t.id == trophy.id {
            return true;
        }
    }
    false
}

pub fn save_trophy(trophy: &Trophy, grind_tx: &Sender<GrindKill>) {
    append_csv(TROPHIES, vec![trophy]);
    let grinds = grinds_to_add(&trophy.species, &trophy.reserve);
    for g in grinds {
        add_kill(&g);
        grind_tx.send(GrindKill { name: g.clone() }).unwrap();
    }
}

pub fn get_grinds() -> Vec<Grind>{
    read_csv(GRINDS)
}

pub fn grinds_to_add(species: &Species, reserve: &Reserve) -> Vec<String> {
    get_grinds()
        .iter()
        .filter(|g| g.species == *species && g.reserve == *reserve && g.active == true)
        .map(|r| r.name.clone())
        .collect()
}

pub fn add_grind(grind: Grind) {
    append_csv(GRINDS, vec![grind]);
}

pub fn remove_grind(name: String) {
    let grinds = get_grinds();
    let mut new_grinds = Vec::new();
    for g in grinds {
        if g.name != name {
            new_grinds.push(g);
        }
    }
    create_csv(GRINDS, new_grinds);
}

pub fn set_grind_active(name: String, active: bool) {
    let mut grinds = get_grinds();
    let mut new_grinds = Vec::new();
    for g in grinds.iter_mut() {
        if g.name == name {
            g.active = active;
        }
        new_grinds.push(g);
    }
    create_csv(GRINDS, new_grinds);
}

pub fn stop_grind(name: String) {
    set_grind_active(name, false);
}

pub fn start_grind(name: String) {
    set_grind_active(name, true);
}

fn add_kill(name: &String) {
    let mut grinds = get_grinds();
    let mut new_grinds = Vec::new();
    for g in grinds.iter_mut() {
        if g.name == *name {
            g.kills += 1;
        }
        new_grinds.push(g);
    }
    create_csv(GRINDS, new_grinds);
}

pub fn get_challenges() -> Vec<ChallengeSummary> {
    let mut challenge_summaries = Vec::new();
    for entry in fs::read_dir(CHALLENGES).unwrap() {
        let entry = entry.unwrap();
        let path = entry.path();
        let file = path.to_str().unwrap();
        let challenges = read_csv::<Challenge>(file);
        challenge_summaries.push(into_summary(&challenges));
    }
    challenge_summaries
}

pub fn challenge_exists(challenge: &Challenge) -> bool {
    let filename = Path::new(&CHALLENGES.to_string()).join(&create_challenge_filename(challenge));
    Path::new(filename.to_str().unwrap()).exists()
}

pub fn save_challenge(challenge: &Challenge) {
    let challenges = process_challenge(challenge);
    let filename = Path::new(&CHALLENGES.to_string()).join(&create_challenge_filename(challenge));
    create_csv(filename.to_str().unwrap(), challenges);
}

pub fn delete_challenge(name: &String) {
    let filename = Path::new(&CHALLENGES.to_string()).join(convert_challenge_name(name));
    fs::remove_file(filename).unwrap_or_default();
}