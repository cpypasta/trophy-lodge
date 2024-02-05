use crate::models::Trophy;
use std::path::Path;
use std::fs::OpenOptions;

const FILE: &str = "./data/trophies.csv";

pub fn read_trophies() -> Vec<Trophy> {
    if !Path::new(FILE).exists() {
        return Vec::new();
    }

    let file = OpenOptions::new()
        .read(true)
        .open(FILE)
        .unwrap();
    let mut rdr = csv::Reader::from_reader(file);
    let mut trophies = Vec::new();
    for trophy in rdr.deserialize() {
        if let Ok(t) = trophy {
            trophies.push(t);
        }
    }
    trophies
}

fn trophy_exists(trophy: &Trophy) -> bool {
    let trophies = read_trophies();
    for t in trophies {
        if t.id == trophy.id {
            return true;
        }
    }
    false
}

pub fn save_trophy(trophy: &Trophy) -> bool {
    if trophy_exists(trophy) {
        return false;
    }

    let file_exists = Path::new(FILE).exists();
    let file = OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(FILE)
        .unwrap();
    let mut wtr = csv::WriterBuilder::new().has_headers(!file_exists).from_writer(file);
    wtr.serialize(trophy).unwrap();
    wtr.flush().unwrap();
    true
}