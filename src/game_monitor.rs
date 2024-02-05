
use proc_mem::Process;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::str::FromStr;
use std::path::PathBuf;
use crate::models::*;
use crate::data;
use chrono::prelude::*;
use convert_case::{Case, Casing};
use windows::Win32::Foundation::CloseHandle;
use windows::Win32::System::Threading::{OpenProcess, PROCESS_QUERY_INFORMATION, PROCESS_VM_READ};
use windows::Win32::System::ProcessStatus::GetModuleFileNameExW;

struct Pointer {
    pub base_address: usize,
}
impl Pointer {
    fn new(base_address: usize) -> Self {
        Self { base_address }
    }
    fn add(mut self, proc: &Process, offset: usize) -> Self {
        self.base_address = read_usize(proc, self.base_address, offset);
        self
    }
    fn value(self) -> usize {
        self.base_address
    }
}

#[derive(Debug)]
struct Game {
    pub proc: Process,
    pub base_address: usize,
    pub directory: Option<String>,
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

fn get_process_path(pid: u32) -> Option<String> {
    unsafe {
        if let Ok(process_handle) = OpenProcess(PROCESS_QUERY_INFORMATION | PROCESS_VM_READ, false, pid) {
            let mut buffer: [u16; 260] = [0; 260];
            let len = GetModuleFileNameExW(process_handle, None, &mut buffer);
            CloseHandle(process_handle).unwrap();
            if len > 0 {
                return Some(String::from_utf16_lossy(&buffer[..len as usize]));
            }
        }
        None
    }
}

fn get_game() -> Option<Game> {
    let mut game: Option<Game> = None;
    let game_name = "theHunterCotW_F.exe";
    if let Ok(cotw_proc) = Process::with_name(game_name) {
        if let Ok(cotw) = cotw_proc.module(game_name) {   
            let cotw_pid = cotw_proc.pid().clone();             
            game = Some(Game { 
                proc: cotw_proc, 
                base_address: cotw.base_address(),
                directory: get_process_path(cotw_pid),
            });
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

fn read_usize(cotw: &Process, address: usize, offset: usize) -> usize {
    read_mem::<usize>(cotw, address, offset)
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

fn using_mods(game_dir: &Option<String>) -> bool {
    match game_dir {
        Some(dir) => {
            let path = PathBuf::from(dir);
            path.parent().unwrap().join("dropzone").exists()
        }
        None => false
    }
}

fn get_shot_base_address(base_address: usize, proc: &Process) -> usize {  
    Pointer::new(base_address).add(proc, 0x023C9B78).add(proc, 0x30).add(proc, 0xD8).add(proc, 0x260).value()
}

fn find_fur(proc: &Process, fur_lookup: usize, fur_lookup_offset: usize, mut max_fur_count: usize, fur_name_key: i32) -> Option<String> {
    let fur_key_size = 0x08;
    let mut current_offset = fur_lookup + fur_lookup_offset;

    while max_fur_count > 0 {
        let current_fur_key = read_int(proc, current_offset, 0x0);
        if fur_name_key == current_fur_key {
            let fur_lookup_address = read_usize(proc, fur_lookup, 0x0);            
            let fur_name_offset = read_int(proc, current_offset, 0x04);
            println!("{} + {}", fur_lookup_address, fur_name_offset);
            let fur_name_address = fur_lookup_address + fur_name_offset as usize;
            return Some(read_string(&proc, fur_name_address as usize, 0x0));
        }
        current_offset += fur_key_size;
        max_fur_count -= 1;
    }
    None
}

fn get_fur2(proc: &Process, base_address: usize, fur_name_key: i32) -> Option<String> {
    let fur_lookup = Pointer::new(base_address).add(proc, 0x0227B640).add(proc, 0x0).add(proc, 0x0).value() + 0x10;
    find_fur(proc, fur_lookup, 0x150, 0x2B2, fur_name_key)
}

fn get_fur(proc: &Process, base_address: usize, fur_name_key: i32) -> Option<String> {
    let fur_lookup = Pointer::new(base_address).add(proc, 0x0227B640).add(proc, 0x0).add(proc, 0x10).value() + 0x10;
    find_fur(proc, fur_lookup, 0x160, 0x030000, fur_name_key)
}

fn get_fur_name(proc: &Process, base_address: usize, fur_name_key: i32) -> String {
    if let Some(fur_name) = get_fur(proc, base_address, fur_name_key) {
        fur_name
    } else if let Some(fur_name) = get_fur2(proc, base_address, fur_name_key) {
        fur_name
    } else {
        "Unknown".to_string()
    }
}

pub fn monitor(status_tx: Sender<String>, trophy_tx: Sender<Trophy>) {
    let mut last_session_score = 0i32;
    let game_proc: Process;
    let game_directory: Option<String>;
    let base_address: usize;
    let harvest_base_address: usize;

    loop {
        if let Some(game) = get_game() {
            game_proc = game.proc;
            game_directory = game.directory;
            base_address = game.base_address;
            harvest_base_address = read_mem::<usize>(&game_proc, base_address, 0x023D1EF0) + 0x280;
            status_tx.send(format!("Game: 0x{:X}; Harvest: 0x{:X}", base_address, harvest_base_address)).unwrap();
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
            let shot_info_base_address = get_shot_base_address(base_address, &game_proc);
            let trophy_species = read_string(&game_proc, harvest_base_address, offsets.species);
            let trophy_reserve = read_mem::<usize>(&game_proc, harvest_base_address, offsets.reserve);
            let trophy_reserve = read_string(&game_proc, trophy_reserve, 0x0);
            let trophy_rating = read_byte(&game_proc, harvest_base_address, offsets.rating);
            let score = read_float(&game_proc, harvest_base_address, offsets.score);
            let weight = read_float(&game_proc, harvest_base_address, offsets.weight);
            let tracking = read_float(&game_proc, base_address, offsets.tracking);
            let cash = read_int(&game_proc, harvest_base_address, offsets.cash);
            let xp = read_int(&game_proc, harvest_base_address, offsets.xp);
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
            let fur_key = read_int(&game_proc, harvest_base_address, offsets.fur_offset);
            let fur = get_fur_name(&game_proc, base_address, fur_key);
            println!("Fur: {}", fur);
            let trophy = Trophy {
                id: score + weight + tracking + cash as f32 + xp as f32,
                species: Species::from_str(&trophy_species).unwrap_or(Species::Unknown),
                reserve: Reserves::from_str(&trophy_reserve).unwrap_or(Reserves::Unknown),
                rating,
                score,
                weight,
                fur,
                date: Local::now().to_string(),
                gender,
                cash,
                xp,
                session_score,
                integrity: read_int(&game_proc, harvest_base_address, offsets.integrity) == 1,
                tracking,
                weapon_score: read_float(&game_proc, shot_info_base_address, offsets.weapon_score),
                shot_distance: read_float(&game_proc, shot_info_base_address, offsets.shot_distance),
                shot_damage: read_float(&game_proc, shot_info_base_address, offsets.shot_damage) * 100.0,
                mods: using_mods(&game_directory),
            };
            if data::save_trophy(&trophy) {
                trophy_tx.send(trophy).unwrap();
                status_tx.send(format!("Stored {} trophy on {}", trophy_species, trophy_reserve)).unwrap();
            }
        }
        thread::sleep(Duration::from_secs(5));
    }
}