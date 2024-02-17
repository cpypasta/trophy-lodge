
use proc_mem::Process;
use std::sync::mpsc::Sender;
use std::thread;
use std::time::Duration;
use std::str::FromStr;
use std::path::PathBuf;
use std::convert::From;
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
    fn address(self) -> usize {
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

fn read_mem<T: Default>(cotw: &Process, address: usize, offset: usize) -> Option<T> {
    if let Ok(result) = cotw.read_mem::<T>(address + offset) {
        Some(result)
    } else {
        None
    }
}

fn read_int(cotw: &Process, address: usize, offset: usize) -> i32 {
    if let Some(x) = read_mem::<i32>(cotw, address, offset) {
        x
    } else {
        -1
    }
}

fn read_float(cotw: &Process, address: usize, offset: usize) -> f32 {
    if let Some(x) = read_mem::<f32>(cotw, address, offset) {
        x
    } else {
        f32::default()
    }
}

fn read_byte(cotw: &Process, address: usize, offset: usize) -> u8 {
    if let Some(x) = read_mem::<u8>(cotw, address, offset) {
        x
    } else {
        u8::default()
    }
}

fn read_usize(cotw: &Process, address: usize, offset: usize) -> usize {
    if let Some(x) = read_mem::<usize>(cotw, address, offset) {
        x
    } else {
        usize::default()
    }
}

fn read_string(cotw: &Process, address: usize, offset: usize, format: bool) -> String {
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
    if format {
        result.replace("-", " ").to_case(Case::Title)
    } else {
        result
    }
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
    Pointer::new(base_address).add(proc, 0x023C9B78).add(proc, 0x30).add(proc, 0xD8).add(proc, 0x260).address()
}

fn find_fur(proc: &Process, fur_lookup: usize, fur_lookup_offset: usize, mut max_fur_count: usize, fur_name_key: i32) -> Option<String> {
    let fur_key_size = 0x08;
    let mut current_offset = fur_lookup + fur_lookup_offset;

    while max_fur_count > 0 {
        let current_fur_key = read_int(proc, current_offset, 0x0);
        if fur_name_key == current_fur_key {
            let fur_lookup_address = read_usize(proc, fur_lookup, 0x0);            
            let fur_name_offset = read_int(proc, current_offset, 0x04);
            let fur_name_address = fur_lookup_address + fur_name_offset as usize;
            return Some(read_string(&proc, fur_name_address as usize, 0x0, true));
        }
        current_offset += fur_key_size;
        max_fur_count -= 1;
    }
    None
}

fn get_fur2(proc: &Process, base_address: usize, fur_name_key: i32) -> Option<String> {
    let fur_lookup = Pointer::new(base_address).add(proc, 0x0227B640).add(proc, 0x0).add(proc, 0x0).address() + 0x10;
    find_fur(proc, fur_lookup, 0x150, 0x2B2, fur_name_key)
}

fn get_fur(proc: &Process, base_address: usize, fur_name_key: i32) -> Option<String> {
    let fur_lookup = Pointer::new(base_address).add(proc, 0x0227B640).add(proc, 0x0).add(proc, 0x10).address() + 0x10;
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

fn valid_string(s: &String) -> bool {
    s != "" && s.chars().all(|c| c.is_whitespace() || c.is_ascii_alphabetic())
}

pub fn monitor(status_tx: Sender<String>, trophy_tx: Sender<Trophy>, user_tx: Sender<String>, grind_tx: Sender<GrindKill>, challenge_tx: Sender<ChallengeKill>) {
    let mut last_weight = 0f32;
    let game_proc: Process;
    let game_directory: Option<String>;
    let base_address: usize;
    loop {
        if let Some(game) = get_game() {
            game_proc = game.proc;
            game_directory = game.directory;
            base_address = game.base_address;
            break;
        }
        status_tx.send("Waiting for game...".to_string()).unwrap();
        thread::sleep(Duration::from_secs(2));
    }

    let mut harvest_base_address: usize;
    loop {
        harvest_base_address = Pointer::new(base_address).add(&game_proc, 0x023D1EF0).address() + 0x280;
        if harvest_base_address > 1000 {
            status_tx.send(format!("Attached to game: {:X} {:X}. Waiting for kill.", base_address, harvest_base_address)).unwrap();
            break;
        }

        status_tx.send("Waiting for game to be fully loaded...".to_string()).unwrap();
        thread::sleep(Duration::from_secs(2));
    }

    let offsets = Offsets::new();
    let mut game_open = false;
    loop {
        let session_score = read_int(&game_proc, harvest_base_address, offsets.session_score);
        println!("Session score: {}", session_score);
        if session_score == -1 && game_open {
            status_tx.send("Game has been closed. No longer tracking.".to_string()).unwrap();
            break;
        } 
        if session_score != -1 {
            game_open = true;
        }
        let username_address = Pointer::new(base_address).add(&game_proc, 0x023A5580).address() + 0x390;
        let username = read_string(&game_proc, username_address, 0x0, false);
        user_tx.send(username).unwrap_or_default();
        
        let weight = read_float(&game_proc, harvest_base_address, offsets.weight);
        if game_open && weight != last_weight && weight > 0.0e-15 { 
            last_weight = weight;
            let shot_info_base_address = get_shot_base_address(base_address, &game_proc);
            let mut trophy_species = read_string(&game_proc, harvest_base_address, offsets.species, true);
            if !valid_string(&trophy_species) {
                trophy_species = read_string(&game_proc, read_usize(&game_proc, harvest_base_address, 0x0), 0x0, true);
            }
            let species = Species::from_str(&trophy_species).unwrap_or(Species::Unknown);
            let trophy_reserve = Pointer::new(harvest_base_address).add(&game_proc, offsets.reserve).address();
            let mut trophy_reserve = read_string(&game_proc, trophy_reserve, 0x0, true);
            if !valid_string(&trophy_reserve) {
                trophy_reserve = read_string(&game_proc, harvest_base_address, offsets.reserve, true);
            }
            let reserve = Reserve::from_str(&trophy_reserve).unwrap_or(Reserve::Unknown);
            let trophy_rating = read_byte(&game_proc, harvest_base_address, offsets.rating);
            let score = read_float(&game_proc, harvest_base_address, offsets.score);
            let tracking = read_float(&game_proc, base_address, offsets.tracking);
            let cash = read_int(&game_proc, harvest_base_address, offsets.cash);
            let xp = read_int(&game_proc, harvest_base_address, offsets.xp);
            let rating = match trophy_rating {
                0 => Rating::Diamond,
                1 => Rating::Gold,
                2 => Rating::Silver,
                3 => Rating::Bronze,
                4 => Rating::None,
                _ => Rating::GreatOne,
            };
            let trophy_gender = read_int(&game_proc, harvest_base_address, offsets.gender);
            let gender = match trophy_gender {
                1 => Gender::Male,
                _ => Gender::Female,
            };            
            let fur_key = read_int(&game_proc, harvest_base_address, offsets.fur_offset);
            let fur = get_fur_name(&game_proc, base_address, fur_key);
            let grind_trophy = data::grinds_to_add(&species, &reserve);
            let grind = if grind_trophy.len() > 0 {
                Some(grind_trophy.join("/"))
            } else {
                None
            };

            let trophy = Trophy {
                id: score + weight + tracking + cash as f32 + xp as f32,
                species,
                reserve, 
                rating,
                score,
                weight,
                fur,
                date: Local::now().to_rfc3339(),
                gender,
                cash,
                xp,
                session_score,
                integrity: Boolean::from(read_byte(&game_proc, harvest_base_address, offsets.integrity) == 1),
                tracking,
                weapon_score: read_float(&game_proc, shot_info_base_address, offsets.weapon_score),
                shot_distance: read_float(&game_proc, shot_info_base_address, offsets.shot_distance),
                shot_damage: read_float(&game_proc, shot_info_base_address, offsets.shot_damage) * 100.0,
                mods: Boolean::from(using_mods(&game_directory)),
                grind,
            };
            if trophy.valid() {
                if !data::trophy_exists(&trophy) {
                    data::save_trophy(&trophy, &grind_tx, &challenge_tx);
                    trophy_tx.send(trophy).unwrap_or_default();
                    status_tx.send(format!("Stored {} trophy from {}", trophy_species, trophy_reserve)).unwrap_or_default();
                } else {
                    status_tx.send("Trophy is already saved".to_string()).unwrap_or_default();
                }
            } else {
                status_tx.send(format!("Problem processing trophy with name {trophy_species}")).unwrap_or_default();
                println!("{}", trophy_species);
                println!("{}", trophy_reserve);
                println!("{:?}", trophy);
            }                
        }
        thread::sleep(Duration::from_secs(3));
    }
}