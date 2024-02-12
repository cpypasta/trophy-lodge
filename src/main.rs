#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod data;
mod game_monitor;
mod challenges;

use egui::*;
use std::str::FromStr;
use eframe::Storage;
use std::fmt;
use strum::{IntoEnumIterator, VariantArray};
use models::*;
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use std::sync::mpsc;
use std::sync::mpsc::Receiver;
use chrono::prelude::*;
use std::thread;

const ICON: &[u8] = include_bytes!("../static/icon.png");
const SMALL_FONT: f32 = 14.0;
const MEDIUM_FONT: f32 = 16.0;

fn main() -> Result<(), eframe::Error> {
    let icon_data = eframe::icon_data::from_png_bytes(ICON).expect("Failed to load icon");
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1400.0, 930.0]).with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "Trophy Lodge 🎯",
        options,
        Box::new(|cc| {
            let (status_tx, status_rx) = mpsc::channel::<String>();
            let (user_tx, user_rx) = mpsc::channel::<String>();
            let (trophy_tx, trophy_rx) = mpsc::channel::<Trophy>();
            let (grind_tx, grind_rx) = mpsc::channel::<GrindKill>();
            thread::spawn(move || {
                game_monitor::monitor(status_tx, trophy_tx, user_tx, grind_tx);
            });

            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(cc, status_rx, trophy_rx, user_rx, grind_rx))
        }),
    )
}

fn set_style(ctx: &Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(40.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(MEDIUM_FONT, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

fn combo_options<I, T, F: FnMut(T)>(ui: &mut Ui, current: T, values: I, mut capture: F) 
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    for item in values {
        let value = ui.selectable_value(&mut current.clone(), item, item.to_string());        
        if value.clicked() {
            capture(item);
        }
    }
}

fn create_combo<T, I, F: FnMut(T)>(ui: &mut Ui, label: &str, value: T, values: I, capture: F)
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    ui.label(label);
    ComboBox::new(format!("{}_filter", label.to_lowercase()), "")
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            ui.set_min_width(280.0);
            combo_options(ui, value, values, capture);
        });
}

fn filter_data(trophy_filter: &TrophyFilter, mut data: Vec<Trophy>) -> Vec<Trophy> {
    if trophy_filter.species != Species::All {
        data.retain(|x| x.species == trophy_filter.species);
    }
    if trophy_filter.reserve != Reserve::All {
        data.retain(|x| x.reserve == trophy_filter.reserve);
    }
    if trophy_filter.rating != Rating::All {
        data.retain(|x| x.rating == trophy_filter.rating);
    }
    if trophy_filter.gender != Gender::All {
        data.retain(|x| x.gender == trophy_filter.gender);
    }
    if trophy_filter.grind != "" {
        data.retain(|x| {
            if let Some(grinds) = &x.grind {
                for grind in grinds.split("/") {
                    let same = grind.to_string() == trophy_filter.grind.clone();
                    if same {
                        return true;
                    }
                }
            }
            false
        });
    }
    match trophy_filter.sort_by {
        SortBy::Date => data.sort_by(|a, b| b.date.cmp(&a.date)),
        SortBy::Score => data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap()),
        SortBy::Weight => data.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap()),
        SortBy::Rating => data.sort_by(|a, b| b.rating.cmp(&a.rating)),
        SortBy::ShotDistance => data.sort_by(|a, b| b.shot_distance.partial_cmp(&a.shot_distance).unwrap()),
    }    
    data
}

fn summary_metric(ui: &mut Ui, label: &str, value: String) {
    ui.small(RichText::new(format!("{}:", label)).strong());
    ui.small(RichText::new(format!("{}", value)));    
}

fn col_label(ui: &mut Ui, value: String) {
    ui.vertical_centered(|ui| ui.add(Label::new(value).wrap(false)));
}

fn available_cols() -> Vec<String> {
    TrophyCols::VARIANTS.iter().map(|x| x.to_string()).collect()
}

fn default_cols() -> Vec<String> {
    let cols = [
        TrophyCols::Species,
        TrophyCols::Reserve,
        TrophyCols::Rating,
        TrophyCols::Score,
        TrophyCols::Weight,
        TrophyCols::ShotDistance,
        TrophyCols::ShotDamage,
        TrophyCols::Fur,
    ];
    cols.iter().map(|x| x.to_string()).collect()
}

fn get_species(reserve: Reserve) -> Vec<Species> {
    if reserve == Reserve::All || reserve == Reserve::Unknown {
        Species::iter().collect()
    } else {
        let mut species = reserve_species().get(&reserve).unwrap().clone();
        species.insert(0, Species::All);
        species.push(Species::Unknown);
        species
    }
}

fn show_reserve_summary<F: FnMut(Reserve)>(ui: &mut Ui, reserve: &Reserve, trophies: &Vec<Trophy>, mut capture: F) {
    ui.vertical(|ui| {
        ui.horizontal(|ui| {
            ui.add_space(20.0);
            let image = Image::new(reserve_image(&reserve)).fit_to_original_size(1.0);
            let reserve_btn = ImageButton::new(image);
            if ui.add(reserve_btn).clicked() {
                capture(reserve.clone());
            };
        });
        ui.add_space(5.0);

        let mut total = 0;
        let mut diamonds = 0;
        let mut great_ones = 0;
        for t in trophies.iter().filter(|x| x.reserve == *reserve) {
            total += 1;
            if t.rating == Rating::Diamond {
                diamonds += 1;
            }
            if t.rating == Rating::GreatOne {
                great_ones += 1;
            }
        }
        Grid::new(format!("{}_summary_metrics", reserve.to_string()))
        .num_columns(2)
        .striped(false)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            let padding = 50.0;
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Trophies")).strong());
            });
            ui.small(RichText::new(format!("{}", total)));    
            ui.end_row();
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Diamonds")).strong());
            });
            ui.small(RichText::new(format!("{}", diamonds)));   
            ui.end_row(); 
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Great Ones")).strong());
            });
            ui.small(RichText::new(format!("{}", great_ones)));    
        });
    });    
}

fn show_species_summary(ui: &mut Ui, reserve: &Reserve, species: &Species, trophies: &Vec<Trophy>) {
    ui.vertical(|ui| {
        ui.vertical_centered(|ui| {
            ui.add(Label::new(RichText::new(species.to_string()).small()).wrap(false));
        });
        ui.vertical_centered(|ui| {
            let image = Image::new(species_image(&species)).fit_to_original_size(1.0);
            let reserve_btn = ImageButton::new(image);
            let _ = ui.add(reserve_btn);
        });
        ui.add_space(5.0);

        let mut total = 0;
        let mut diamonds = 0;
        let mut great_ones = 0;
        for t in trophies.iter().filter(|x| x.reserve == *reserve) {
            if t.species == *species {
                total += 1;
                if t.rating == Rating::Diamond {
                    diamonds += 1;
                }
                if t.rating == Rating::GreatOne {
                    great_ones += 1;
                }
            }
        }
        Grid::new(format!("{}_summary_metrics", species.to_string()))
        .num_columns(2)
        .striped(false)
        .spacing([5.0, 5.0])
        .show(ui, |ui| {
            let padding = 30.0;
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Trophies")).strong());
            });
            ui.small(RichText::new(format!("{}", total)));    
            ui.end_row();
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Diamonds")).strong());
            });
            ui.small(RichText::new(format!("{}", diamonds)));   
            ui.end_row(); 
            ui.horizontal(|ui| {
                ui.add_space(padding);
                ui.small(RichText::new(format!("{}:", "Great Ones")).strong());
            });
            ui.small(RichText::new(format!("{}", great_ones)));    
        });
    });    
}

#[derive(PartialEq)]
enum Sidebar {
    Trophies,
    Grinds,
    Challenges,
}

#[derive(PartialEq)]
enum ChallengeTab {
    Create,
    Discover,
}

#[derive(PartialEq)]
enum TrophyTab {
    Lodge,
    Table,
}

struct MyApp {
    menu: Sidebar,
    user_rx: Receiver<String>,
    username: String,
    trophy_tab: TrophyTab,
    trophy_reserve: Reserve,
    trophies: Vec<Trophy>,
    filtered_trophies: Vec<Trophy>,
    trophy_filter: TrophyFilter,
    trophy_cols: Vec<String>,
    selected_cols: Vec<String>,
    status_rx: Receiver<String>,
    status_msg: String,
    trophy_rx: Receiver<Trophy>,
    grinds: Vec<Grind>,
    grind_name: String,
    grind_species: Species,
    grind_reserve: Reserve,
    grind_rx: Receiver<GrindKill>,
    challenge_tab: ChallengeTab,
    challenge: Challenge,
}
impl MyApp {
    fn new(
        cc: &eframe::CreationContext<'_>, 
        status_rx: Receiver<String>, 
        trophy_rx: Receiver<Trophy>, 
        user_rx: Receiver<String>, 
        grind_rx: Receiver<GrindKill>
    ) -> Self {
        data::init();

        let ctx = &cc.egui_ctx;
        let trophies: Vec<Trophy> = data::read_trophies();
        let filtered_trophies = trophies.clone();
        let trophy_filter = TrophyFilter::default();
        let grinds = data::get_grinds();
        set_style(ctx);

        let mut selected_cols = default_cols();
        if let Some(storage) = cc.storage {
            if let Some(cols) = storage.get_string("selected_cols") {
                if let Ok(cols_value) = serde_json::from_str::<Vec<String>>(&cols) {
                    selected_cols = cols_value;
                }
            }
        }

        Self { 
            menu: Sidebar::Trophies, 
            user_rx,
            username: String::from("Unknown User"),
            trophy_tab: TrophyTab::Lodge,
            trophies,
            trophy_reserve: Reserve::All,
            filtered_trophies,
            trophy_filter,
            trophy_cols: available_cols(),
            selected_cols,
            status_rx,
            status_msg: "".to_string(),
            trophy_rx,
            grinds,
            grind_name: "".to_string(),
            grind_species: Species::Unknown,
            grind_reserve: Reserve::Unknown,
            grind_rx,
            challenge_tab: ChallengeTab::Create,
            challenge: Challenge::default(),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {        
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .min_height(120.0)
            .max_height(120.0)
            .show(ctx, |ui| { 
                StripBuilder::new(ui)
                .size(Size::exact(90.0))
                .size(Size::exact(320.0))
                .size(Size::remainder())
                .size(Size::exact(220.0))
                .horizontal(|mut strip| {
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            ui.add_space(10.0);
                            ui.add(Image::new(include_image!("../static/logo2.png"))
                                    .fit_to_original_size(0.8)
                            );
                            ui.add_space(10.0);
                        });
                    });
                    strip.cell(|ui| {
                        ui.horizontal_centered(|ui| {
                            ui.heading(RichText::new("Trophy Lodge"));
                            ui.small(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))));
                        });
                    });
                    strip.cell(|ui| { 
                       ui.with_layout(Layout::top_down(Align::RIGHT), |ui| {
                            ui.horizontal(|ui| {   
                                ui.add_space(5.0);
                                if self.status_msg.contains("Game has been closed") {
                                    ui.label(RichText::new(&self.status_msg).color(Color32::RED).size(SMALL_FONT));
                                } else {
                                    if self.status_msg.contains("Waiting for game") {
                                        ui.spinner();
                                    }
                                    ui.label(RichText::new(&self.status_msg).color(Color32::LIGHT_YELLOW).size(SMALL_FONT));
                                }
                            });  
                        }); 
                    });
                    strip.cell(|ui| {
                        ui.vertical(|ui| {
                            if let Ok(username) = self.user_rx.try_recv() {
                                if username != "" {
                                    self.username = username;
                                }
                            }
                            ui.with_layout(Layout::right_to_left(Align::LEFT), |ui| {
                                ui.small(RichText::new(&self.username).color(Color32::DEBUG_COLOR));
                            });
                            ui.add_space(10.0);
                            ui.horizontal(|ui| {
                                ui.add_space(80.0);
                                Grid::new("summary_metrics")
                                .num_columns(2)
                                .striped(false)
                                .spacing([5.0, 5.0])
                                .show(ui, |ui| {
                                    summary_metric(ui, "Trophies", self.trophies.len().to_string());
                                    ui.end_row();
                                    summary_metric(ui, "Diamonds", self.trophies.iter().filter(|x| x.rating == Rating::Diamond).count().to_string());
                                    ui.end_row();
                                    summary_metric(ui, "Great Ones", self.trophies.iter().filter(|x| x.rating == Rating::GreatOne).count().to_string());
                                    ui.end_row();
                                });
                            });
                        });
                    });               
                });
            });

        SidePanel::left("left_panel")
        .resizable(false)
        .min_width(120.0)
        .show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add_space(10.0);
                ui.selectable_value(&mut self.menu, Sidebar::Trophies, "Trophies");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.menu, Sidebar::Grinds, "Grinds");
                ui.add_space(5.0);
                ui.selectable_value(&mut self.menu, Sidebar::Challenges, "Challenges");
            });
        });

        CentralPanel::default().show(ctx, |ui| {                
            match self.menu {                
                Sidebar::Trophies => {
                    ui.horizontal(|ui| {
                        ui.selectable_value(&mut self.trophy_tab, TrophyTab::Lodge, "Lodge");
                        ui.add_space(5.0);
                        ui.selectable_value(&mut self.trophy_tab, TrophyTab::Table, "Table");
                    });
                    ui.add_space(20.0);
                    match self.trophy_tab {
                        TrophyTab::Lodge => {
                            if self.trophy_reserve == Reserve::All {
                                ScrollArea::vertical().show(ui, |ui| {
                                    Grid::new("lodge_reserves")
                                    .num_columns(5)
                                    .striped(false)
                                    .spacing([20.0, 20.0])
                                    .show(ui, |ui| {
                                        let org_hover_weak_bg_fill = ui.style().visuals.widgets.hovered.weak_bg_fill;
                                        let org_inactive_weak_bg_fill = ui.style().visuals.widgets.inactive.weak_bg_fill;
                                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::BROWN;
                                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = Color32::TRANSPARENT;

                                        let reserves = Reserve::iter().filter(|x| *x != Reserve::Unknown && *x != Reserve::All);
                                        for (i, r) in reserves.enumerate() {
                                            show_reserve_summary(ui, &r, &self.trophies, |x| {
                                                self.trophy_reserve = x;
                                            });
                                            if (i+1) % 5 == 0 {
                                                ui.end_row();
                                            }
                                        }
                                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = org_hover_weak_bg_fill;
                                        ui.style_mut().visuals.widgets.inactive.weak_bg_fill = org_inactive_weak_bg_fill;
                                    });
                                });
                            } else {
                                ui.horizontal(|ui| {                                    
                                    ui.heading(self.trophy_reserve.to_string());
                                    ui.add_space(10.0);
                                    if ui.button("Back").clicked() {
                                        self.trophy_reserve = Reserve::All;
                                    }
                                });
                                if self.trophy_reserve != Reserve::All {
                                    ui.add_space(20.0);
                                    let species = reserve_species().get(&self.trophy_reserve).unwrap().clone();
                                    ScrollArea::vertical().show(ui, |ui| {
                                        Grid::new("lodge_reserve_species")
                                        .num_columns(5)
                                        .striped(false)
                                        .spacing([20.0, 20.0])
                                        .show(ui, |ui| {                                    
                                            for (i, s) in species.iter().enumerate() {
                                                show_species_summary(ui, &self.trophy_reserve, s, &self.trophies);
                                                if (i+1) % 5 == 0 {
                                                    ui.end_row();
                                                }
                                            }
                                        });
                                    });
                                }
                            }
                        },
                        TrophyTab::Table => {
                            ui.collapsing("Configure", |ui| {
                                ui.add_space(10.0);
                                ui.strong("Filter & Sort");
                                ui.add_space(10.0);
                                Grid::new("filter_sort")
                                    .num_columns(7)
                                    .striped(false)
                                    .spacing([30.0, 10.0])
                                    .show(ui, |ui| {
                                        create_combo(ui, "Reserve", self.trophy_filter.reserve, Reserve::iter(), |x| {
                                            self.trophy_filter.reserve = x;
                                            self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                        });
                                        create_combo(ui, "Rating", self.trophy_filter.rating, Rating::iter(), |x| {
                                            self.trophy_filter.rating = x;
                                            self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                        });
                                        ui.label("Grind");
                                        ComboBox::new("grind_filter", "")
                                        .selected_text(&self.trophy_filter.grind)
                                        .show_ui(ui, |ui| {
                                            ui.set_min_width(200.0);
                                            for g in self.grinds.iter() {
                                                let combo = ui.selectable_value(&mut self.trophy_filter.grind, g.name.clone(), g.name.clone());
                                                if combo.clicked() {
                                                    self.trophy_filter.grind = g.name.clone();
                                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                                }
                                            }
                                        });                                                              
                                        ui.vertical(|ui| {
                                            ui.add_space(5.0);
                                            ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::BROWN;
                                            if ui.button("Reset").clicked() {
                                                self.trophy_filter = TrophyFilter::default();
                                                self.filtered_trophies = self.trophies.clone();
                                            }   
                                        });                            
                                        ui.end_row(); 
                                        let species = get_species(self.trophy_filter.reserve);
                                        create_combo(ui, "Species", self.trophy_filter.species, species.into_iter(), |x| { 
                                            self.trophy_filter.species = x;
                                            self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());                                                                        
                                        });
                                        create_combo(ui, "Gender", self.trophy_filter.gender, Gender::iter(), |x| {
                                            self.trophy_filter.gender = x;
                                            self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                        });                                
                                        create_combo(ui, "Sort By", self.trophy_filter.sort_by, SortBy::iter(), |x| {
                                            self.trophy_filter.sort_by = x;
                                            self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                        });                                  
                                        ui.end_row();  
        
                                    });
                                ui.add_space(10.0);
                                ui.strong("Columns");
                                ui.add_space(10.0);
                                Grid::new("viewed_cols")
                                .num_columns(3)
                                .striped(false)
                                .spacing([10.0, 10.0])
                                .show(ui, |ui| {
                                    for (i, col) in self.trophy_cols.iter().enumerate() {
                                        let mut value = self.selected_cols.contains(col);
                                        if ui.checkbox(&mut value, col).changed() {
                                            if value {
                                                self.selected_cols.push(col.clone());
                                            } else {
                                                self.selected_cols.retain(|x| x != col);
                                            }
                                        }
                                        if i % 3 == 2 {
                                            ui.end_row();
                                        }
                                    }           
                                });
                                ui.add_space(20.0);             
                                ui.separator();
                            });               
        
                            ui.add_space(20.0);
        
                            if let Ok(trophy) = self.trophy_rx.try_recv() {
                                self.trophies.push(trophy);
                                self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());                        
                            }
                            let trophies = TableBuilder::new(ui)
                                .striped(true)
                                .resizable(true)                        
                                .sense(Sense::click())
                                .max_scroll_height(f32::INFINITY)
                                .columns(Column::auto(), self.selected_cols.len());
                            trophies
                                .header(30.0, |mut header| {
                                    self.selected_cols.sort_by(|a, b| {
                                        let trophy_col_a = TrophyCols::from_str(&a).unwrap();
                                        let trophy_col_b = TrophyCols::from_str(&b).unwrap();
                                        trophy_col_a.cmp(&trophy_col_b)
                                    });
                                    for h in self.selected_cols.iter() {
                                        header.col(|ui| { 
                                            ui.vertical_centered(|ui| {
                                                ui.add(Label::new(RichText::new(h).strong()).wrap(false));
                                            }); 
                                        });
                                    }
                                })
                                .body(|body| {
                                    body.rows(30.0, self.filtered_trophies.len(), |mut row| {    
                                        let trophy = self.filtered_trophies.get(row.index()).unwrap();
                                        let row_index = row.index().clone();
                                        if self.selected_cols.contains(&"Species".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.species.to_string());
                                            });
                                        }
                                        if self.selected_cols.contains(&"Reserve".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.reserve.to_string());
                                            });
                                        }
                                        if self.selected_cols.contains(&"Rating".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.rating.to_string());
                                            });
                                        }
                                        if self.selected_cols.contains(&"Score".to_string()) {
                                            row.col(|ui| { 
                                                let score = format!("{:.2}", trophy.score);
                                                col_label(ui, score);
                                            });
                                        }
                                        if self.selected_cols.contains(&"Weight".to_string()) {
                                            row.col(|ui| { 
                                                let weight = format!("{:.2}", trophy.weight);
                                                col_label(ui, weight);
                                            });        
                                        }    
                                        if self.selected_cols.contains(&"Fur".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.fur.to_string());
                                            });        
                                        }                 
                                        if self.selected_cols.contains(&"Gender".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.gender.to_string());
                                            });        
                                        }                                                             
                                        if self.selected_cols.contains(&"Date".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.date.to_string());
                                            });        
                                        }
                                        if self.selected_cols.contains(&"Cash".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.cash.to_string());
                                            });        
                                        }         
                                        if self.selected_cols.contains(&"Xp".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.xp.to_string());
                                            });        
                                        }   
                                        if self.selected_cols.contains(&"Session Score".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.session_score.to_string());
                                            });        
                                        }                                         
                                        if self.selected_cols.contains(&"Integrity".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.integrity.to_string());
                                            });        
                                        }                                           
                                        if self.selected_cols.contains(&"Tracking".to_string()) {
                                            row.col(|ui| { 
                                                let tracking = format!("{:.2}", trophy.tracking);
                                                col_label(ui, tracking);
                                            });        
                                        }  
                                        if self.selected_cols.contains(&"Weapon Score".to_string()) {
                                            row.col(|ui| { 
                                                let weapon_score = format!("{:.2}", trophy.weapon_score);
                                                col_label(ui, weapon_score);
                                            });        
                                        }                                                
                                        if self.selected_cols.contains(&"Shot Distance".to_string()) {
                                            row.col(|ui| { 
                                                let shot_distance = format!("{:.2}", trophy.shot_distance);
                                                col_label(ui, shot_distance);
                                            });        
                                        }                                        
                                        if self.selected_cols.contains(&"Shot Damage".to_string()) {
                                            row.col(|ui| { 
                                                let shot_damage = format!("{:.0}%", trophy.shot_damage);
                                                col_label(ui, shot_damage);
                                            });        
                                        }                                         
                                        if self.selected_cols.contains(&"Mods".to_string()) {
                                            row.col(|ui| { 
                                                col_label(ui, trophy.mods.to_string());
                                            });        
                                        }          
                                        if self.selected_cols.contains(&"Grind".to_string()) {
                                            row.col(|ui| { 
                                                if let Some(grinds) = &trophy.grind {
                                                    let grind_split: Vec<String> = grinds.split("/").map(|x| x.to_string()).collect();
                                                    if grind_split.len() > 1 {
                                                        ui.vertical_centered(|ui| {
                                                            ui.horizontal(|ui| {
                                                                ui.add_space(8.0);
                                                                ComboBox::new(format!("{}_grind_display", row_index), "")
                                                                .selected_text(grind_split[0].clone())
                                                                .show_ui(ui, |ui| {
                                                                    ui.set_min_width(200.0);
                                                                    for g in grind_split {
                                                                        let _ = ui.selectable_label(false, g.clone());
                                                                    }
                                                                }); 
                                                            });
                                                        });                                                   
                                                    } else {
                                                        col_label(ui, grinds.clone());
                                                    }
                                                }
                                            });        
                                        }                                                           
                                    });
                                });
                        }
                    }

                },
                Sidebar::Grinds => {
                    ui.collapsing("Create", |ui| {
                        ui.add_space(10.0);
                        Grid::new("grinds")
                        .num_columns(2)
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut self.grind_name);
                            ui.end_row();
                            create_combo(ui, "Reserve", self.grind_reserve, Reserve::iter(), |x| { 
                                self.grind_reserve = x;                                                                     
                            });           
                            ui.end_row();     
                            let species = get_species(self.grind_reserve); 
                            create_combo(ui, "Species", self.grind_species, species.into_iter(), |x| { 
                                self.grind_species = x;                                                                     
                            });           
                            ui.end_row();       
                        });
                        ui.add_space(10.0);
                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_GREEN;
                        if ui.button("Start Grind").clicked() {
                            let grind = Grind {
                                name: self.grind_name.clone(),
                                species: self.grind_species.clone(),
                                reserve: self.grind_reserve.clone(),
                                active: true,
                                start: Local::now().to_rfc3339().to_owned(),
                                kills: 0,
                                is_deleted: false,
                            };
                            if grind.valid(&self.grinds) {
                                data::add_grind(grind.clone());
                                self.grinds.push(grind);
                                self.grind_name = "".to_string();
                                self.grind_species = Species::Unknown;
                                self.grind_reserve = Reserve::Unknown;
                            }
                        }                 
                        ui.add_space(20.0);
                        ui.separator();
                    });
                    ui.add_space(20.0);
                    let grinds = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .sense(Sense::hover())
                        .max_scroll_height(f32::INFINITY)
                        .columns(Column::auto(), 7);
                    grinds.header(30.0, |mut header| {
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Name").strong()).wrap(false));
                            });
                        });
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Species").strong()).wrap(false));
                            });
                        }); 
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Reserve").strong()).wrap(false));
                            });
                        }); 
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Days").strong()).wrap(false));
                            });
                        });
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Kills").strong()).wrap(false));
                            });
                        }); 
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Active").strong()).wrap(false));
                            });
                        }); 
                        header.col(|ui| {
                            ui.vertical_centered(|ui| {
                                ui.add(Label::new(RichText::new("Delete").strong()).wrap(false));
                            });
                        });
                    }).body(|body| {
                        self.grinds.retain(|x| !x.is_deleted);

                        if let Ok(grind_kill) = self.grind_rx.try_recv() {
                            for grind in self.grinds.iter_mut() {
                                if grind.name == grind_kill.name {
                                    grind.kills += 1;
                                }
                            }
                        } 
                        body.rows(30.0, self.grinds.len(), |mut row| {
                            let grind = self.grinds.get_mut(row.index()).unwrap();
                            row.col(|ui| {
                                col_label(ui, grind.name.clone());
                            });
                            row.col(|ui| {
                                col_label(ui, grind.species.to_string());
                            });
                            row.col(|ui| {
                                col_label(ui, grind.reserve.to_string());
                            });
                            row.col(|ui| {
                                let past = DateTime::parse_from_rfc3339(&grind.start).unwrap();
                                let now = Local::now();
                                let duration = now.signed_duration_since(past);
                                col_label(ui, duration.num_days().to_string());
                            });
                            row.col(|ui| {
                                col_label(ui, grind.kills.to_string());
                            });
                            row.col(|ui| {
                                ui.vertical_centered(|ui| {
                                    if grind.active {
                                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::BROWN;
                                        if ui.button("Stop").clicked() {
                                            println!("Stop grind");
                                            data::stop_grind(grind.name.clone());
                                            grind.active = false;                                        
                                        }
                                    } else {
                                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_GREEN;
                                        if ui.button("Start").clicked() {
                                            println!("Start grind");
                                            data::start_grind(grind.name.clone());
                                            grind.active = true;
                                        }
                                    }
                                });
                            });  
                            row.col(|ui| {
                                ui.vertical_centered(|ui| {
                                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::BROWN;
                                    if ui.button("Delete").clicked() {
                                        data::remove_grind(grind.name.clone());
                                        grind.is_deleted = true;
                                    }
                                });
                            });                                                         
                        });
                    });
                },
                Sidebar::Challenges => {
                    ui.collapsing("Create & Discover", |ui| {
                        ui.add_space(10.0);
                        ui.horizontal(|ui| {
                            ui.selectable_value(&mut self.challenge_tab, ChallengeTab::Create, "Create");
                            ui.add_space(5.0);
                            ui.selectable_value(&mut self.challenge_tab, ChallengeTab::Discover, "Discover");
                        });
                        match self.challenge_tab {
                            ChallengeTab::Create => {
                                ui.add_space(10.0);
                                ui.small("(use Unknown or 0 to ignore the field)");
                                ui.add_space(10.0);

                                Grid::new("create_challenge")
                                .num_columns(8)
                                .striped(false)
                                .spacing([10.0, 10.0])
                                .show(ui, |ui| {
                                    ui.label("Name");
                                    ui.add(TextEdit::singleline(&mut self.challenge.name).min_size([150.0, 20.0].into()));
                                    ui.end_row();
                                    create_combo(ui, "Reserve", self.challenge.reserve, Reserve::iter(), |x| {
                                        self.challenge.reserve = x;
                                    });
                                    create_combo(ui, "Gender", self.challenge.gender, Gender::iter(), |x| {
                                        self.challenge.gender = x;
                                    });
                                    ui.label("Shot Damage (min)");
                                    ui.add(Slider::new(&mut self.challenge.shot_damage, 0..=100));
                                    ui.label("Kills").on_hover_text("Number of kills to complete the challenge");
                                    ui.add(Slider::new(&mut self.challenge.kills, 1..=50));
                                    ui.end_row();

                                    let species = get_species(self.challenge.reserve);
                                    create_combo(ui, "Species", self.challenge.species, species.into_iter(), |x| {
                                        self.challenge.species = x;
                                    });
                                    create_combo(ui, "Mods", self.challenge.mods, Boolean::iter(), |x| {
                                        self.challenge.mods = x;
                                    });
                                    ui.label("Shot Distance (min)");
                                    ui.add(Slider::new(&mut self.challenge.shot_distance, 0..=1000));
                                    ui.label("Weight (min)");
                                    ui.add(Slider::new(&mut self.challenge.weight, 0.0..=2000.0));
                                    ui.end_row();

                                    create_combo(ui, "Weapon", self.challenge.weapon, Weapon::iter(), |x| {
                                        self.challenge.weapon = x;
                                    });
                                    create_combo(ui, "Rating", self.challenge.rating, Rating::iter(), |x| {
                                        self.challenge.rating = x;
                                    });
                                    ui.label("Tracking (max)");
                                    ui.add(Slider::new(&mut self.challenge.tracking, 0..=1000));
                                    ui.label("Score (min)");
                                    ui.add(Slider::new(&mut self.challenge.score, 0.0..=1100.0));
                                    ui.end_row();

                                    create_combo(ui, "Public", self.challenge.public, Boolean::iter(), |x| {
                                        self.challenge.public = x;
                                    });
                                    ui.label("");
                                    ui.label("");
                                    ui.label("Total Shots (max)");
                                    ui.add(Slider::new(&mut self.challenge.total_shots, 0..=10));
                                    ui.end_row();
                                });
                                ui.add_space(10.0);
                                ui.horizontal(|ui| {
                                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::BROWN;
                                    if ui.button("Reset").clicked() {
                                        self.challenge = Challenge::default();
                                    }
                                    ui.add_space(10.0);
                                    ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_GREEN;
                                    if ui.button("Start Challenge").clicked() {
                                        let challenges = challenges::process_challenge(&self.challenge);
                                        println!("{:?}", challenges);
                                    }
                                });
                                ui.add_space(20.0);
                                ui.separator();
                            },
                            ChallengeTab::Discover => {
                                ui.add_space(10.0);
                                ui.label("coming soon...");
                            }
                        }
                    });
                },
            }
        });

        if let Ok(status) = self.status_rx.try_recv() {
            self.status_msg = status;
        } else {
            ctx.request_repaint();
        }
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Ok(value) = serde_json::to_string(&self.selected_cols) {
            storage.set_string("selected_cols", value);
        }
    }   
}