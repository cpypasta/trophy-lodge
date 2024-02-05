
use proc_mem::Process;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::str::FromStr;
use crate::models::*;
use chrono::prelude::*;
use convert_case::{Case, Casing};

struct Game {
    pub proc: Process,
    pub base_address: usize,
}

struct Offsets {
    species: usize,
    gender: usize,
    weight: usize,
    tracking: usize,
    xp: usize,
    cash: usize,
    score: usize,
    rating: usize,
    integrity: usize,
    fur_offset: usize,
    session_score: usize,
    reserve: usize,
    weapon_score: usize,
    shot_distance: usize,
    shot_damage: usize,
}
impl Offsets {
    fn new() -> Self {
        Self {
            species: 0x0,
            gender: 0x20,
            weight: 0x24,
            tracking: 0x28,
            xp: 0x34,
            cash: 0x38,
            score: 0x3C,
            rating: 0xAC,
            integrity: 0x4C,
            fur_offset: 0x50,
            session_score: 0xB0,
            reserve: 0x60, // pointer
            weapon_score: 0x18,
            shot_distance: 0x1C,
            shot_damage: 0x20,
        }
    }
}

fn get_game() -> Option<Game> {
    let mut game: Option<Game> = None;
    let game_name = "theHunterCotW_F.exe";
    if let Ok(cotw_proc) = Process::with_name(game_name) {
        if let Ok(cotw) = cotw_proc.module(game_name) {                
            game = Some(Game { proc: cotw_proc, base_address: cotw.base_address() })
        }
    }
    game
}

fn read_mem<T: Default>(cotw: &Process, address: usize, offset: usize) -> T {
    if let Ok(result) = cotw.read_mem::<T>(address + offset) {
        result
    } else {
        T::default()
    }
}

fn read_int(cotw: &Process, address: usize, offset: usize) -> i32 {
    read_mem::<i32>(cotw, address, offset)
}

fn read_float(cotw: &Process, address: usize, offset: usize) -> f32 {
    read_mem::<f32>(cotw, address, offset)
}

fn read_byte(cotw: &Process, address: usize, offset: usize) -> u8 {
    read_mem::<u8>(cotw, address, offset)
}

fn read_string(cotw: &Process, address: usize, offset: usize) -> String {
    let mut result = String::new();
    let mut s_offset = 0x0;
    loop {
        let byte = read_byte(&cotw, address, offset + s_offset);
        if byte == 0 {
            break;
        }
        result.push(byte as char);
        s_offset += 0x1;
    }
    result.to_case(Case::Title)
}

pub fn monitor(status_tx: Sender<String>, trophy_tx: Sender<Trophy>) {
    let mut last_session_score = 0i32;
    let game_proc: Process;
    let base_address: usize;
    let harvest_base_address: usize;
    let mut shot_info_base_address: usize;

    loop {
        if let Some(game) = get_game() {
            game_proc = game.proc;
            base_address = game.base_address;
            harvest_base_address = read_mem::<usize>(&game_proc, base_address, 0x023D1EF0) + 0x280;
            shot_info_base_address = read_mem::<usize>(&game_proc, base_address, 0x023C9B78);
            shot_info_base_address = read_mem::<usize>(&game_proc, shot_info_base_address, 0x30);
            shot_info_base_address = read_mem::<usize>(&game_proc, shot_info_base_address, 0xD8);
            shot_info_base_address = read_mem::<usize>(&game_proc, shot_info_base_address, 0x260);
            status_tx.send(format!("Game: 0x{:X}; Harvest: 0x{:X}; Shot:  0x{:X}", base_address, harvest_base_address, shot_info_base_address)).unwrap();
            break;
        }
        status_tx.send("Waiting for game...".to_string()).unwrap();
        thread::sleep(Duration::from_secs(2));
    }

    let offsets = Offsets::new();
    loop {
        let session_score = read_int(&game_proc, harvest_base_address, offsets.session_score);
        if session_score != last_session_score {
            last_session_score = session_score;
            let trophy_species = read_string(&game_proc, harvest_base_address, offsets.species);
            let trophy_reserve = read_mem::<usize>(&game_proc, harvest_base_address, offsets.reserve);
            let trophy_reserve = read_string(&game_proc, trophy_reserve, 0x0);
            let trophy_rating = read_byte(&game_proc, harvest_base_address, offsets.rating);
            let rating = match trophy_rating {
                1 => Ratings::Diamond,
                2 => Ratings::Silver,
                3 => Ratings::Bronze,
                4 => Ratings::None,
                _ => Ratings::GreatOne,
            };
            let trophy_gender = read_int(&game_proc, harvest_base_address, offsets.gender);
            let gender = match trophy_gender {
                1 => Gender::Male,
                _ => Gender::Female,
            };

            let trophy = Trophy {
                species: Species::from_str(&trophy_species).unwrap(),
                reserve: Reserves::from_str(&trophy_reserve).unwrap(),
                rating,
                score: read_float(&game_proc, harvest_base_address, offsets.score),
                weight: read_float(&game_proc, harvest_base_address, offsets.weight),
                fur: "Dark".to_string(),
                date: Utc::now().to_string(),
                gender,
                cash: read_int(&game_proc, harvest_base_address, offsets.cash),
                xp: read_int(&game_proc, harvest_base_address, offsets.xp),
                session_score,
                integrity: read_int(&game_proc, harvest_base_address, offsets.integrity) == 1,
                tracking: read_float(&game_proc, base_address, offsets.tracking),
                weapon_score: read_float(&game_proc, shot_info_base_address, offsets.weapon_score),
                shot_distance: read_float(&game_proc, shot_info_base_address, offsets.shot_distance),
                shot_damage: read_float(&game_proc, shot_info_base_address, offsets.shot_damage) * 100.0,
                mods: false,
            };
            trophy_tx.send(trophy).unwrap();
            status_tx.send(format!("Stored {} trophy on {}", trophy_species, trophy_reserve)).unwrap();
        }
        thread::sleep(Duration::from_secs(5));
    }
}