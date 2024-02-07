use crate::models::{Trophy, Grind};
use std::path::Path;
use std::fs::{File, OpenOptions};
use sqlx::sqlite::SqliteConnection;
use sqlx::Executor;
use sqlx::Connection;

const FILE: &str = "./data/trophies.csv";
const GRINDS_DB: &str = "./data/grinds.db";

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

async fn get_grinds_connection() -> SqliteConnection {
    let file = Path::new(GRINDS_DB);
    if !file.exists() {
        File::create(file).unwrap();
    }
    SqliteConnection::connect(format!("sqlite://{GRINDS_DB}").as_str()).await.unwrap()
}

pub async fn get_grinds() -> Vec<Grind>{
    let mut conn = get_grinds_connection().await;
    let recs = sqlx::query_as!(Grind, "SELECT * FROM grinds WHERE is_deleted = FALSE")
        .fetch_all(&mut conn)
        .await
        .unwrap();
    recs
}

pub async fn add_grind(grind: Grind) {
    let mut conn = get_grinds_connection().await;
    let species = grind.species.to_string();
    let reserve = grind.reserve.to_string();
    conn.execute(sqlx::query!(
        "INSERT INTO grinds (name, species, reserve, active, start, kills) 
        VALUES (?,?,?,?,?,?);
    ", grind.name, species, reserve, grind.active, grind.start, grind.kills))
    .await.unwrap();
}

pub async fn remove_grind(name: String) {
    let mut conn = get_grinds_connection().await;
    conn.execute(sqlx::query!(
        "DELETE FROM grinds WHERE name = ?;", name
    )).await.unwrap();
}

pub async fn stop_grind(name: String) {
    let mut conn = get_grinds_connection().await;
    conn.execute(sqlx::query!(
        "UPDATE grinds SET active = 0 WHERE name = ?;", name
    )).await.unwrap();
}

pub async fn start_grind(name: String) {
    let mut conn = get_grinds_connection().await;
    conn.execute(sqlx::query!(
        "UPDATE grinds SET active = 1 WHERE name = ?;", name
    )).await.unwrap();
}