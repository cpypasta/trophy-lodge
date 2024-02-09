#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod data;
mod game_monitor;

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
        viewport: ViewportBuilder::default().with_inner_size([1300.0, 800.0]).with_icon(icon_data),
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

fn combo_options<I, T, F: FnMut(T)>(ui: &mut Ui, current: &mut T, values: I, mut capture: F) 
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    for item in values {
        let value = ui.selectable_value(current, item, item.to_string());        
        if value.clicked() {
            capture(item);
        }
    }
}

fn create_combo<T, I, F: FnMut(T)>(ui: &mut Ui, label: &str, value: &mut T, values: I, capture: F)
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    ui.label(label);
    ComboBox::new(format!("{}_filter", label.to_lowercase()), "")
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            ui.set_min_width(250.0);
            combo_options(ui, value, values, capture);
        });
}

fn filter_data(trophy_filter: &TrophyFilter, mut data: Vec<Trophy>) -> Vec<Trophy> {
    if trophy_filter.species != Species::All {
        data.retain(|x| x.species == trophy_filter.species);
    }
    if trophy_filter.reserve != Reserves::All {
        data.retain(|x| x.reserve == trophy_filter.reserve);
    }
    if trophy_filter.rating != Ratings::All {
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
    ];
    cols.iter().map(|x| x.to_string()).collect()
}

#[derive(PartialEq)]
enum Sidebar {
    Trophies,
    Grinds,
}

struct MyApp {
    menu: Sidebar,
    species: Species,
    reserves: Reserves,
    ratings: Ratings,
    grind: String,
    gender: Gender,
    sort_by: SortBy,
    user_rx: Receiver<String>,
    username: String,
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
    grind_reserve: Reserves,
    grind_rx: Receiver<GrindKill>,
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
            species: Species::All,
            reserves: Reserves::All,
            ratings: Ratings::All,
            grind: "".to_string(),
            gender: Gender::All,
            sort_by: SortBy::Date,
            user_rx,
            username: String::from("Unknown User"),
            trophies,
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
            grind_reserve: Reserves::Unknown,
            grind_rx,
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
                            ui.group(|ui| {
                                Grid::new("summary_metrics")
                                .num_columns(2)
                                .striped(false)
                                .spacing([5.0, 5.0])
                                .show(ui, |ui| {
                                    summary_metric(ui, "Trophies", self.trophies.len().to_string());
                                    ui.end_row();
                                    summary_metric(ui, "Diamonds", self.trophies.iter().filter(|x| x.rating == Ratings::Diamond).count().to_string());
                                    ui.end_row();
                                    summary_metric(ui, "Great Ones", self.trophies.iter().filter(|x| x.rating == Ratings::GreatOne).count().to_string());
                                    ui.end_row();
                                });
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
            });
        });

        CentralPanel::default().show(ctx, |ui| {                
            match self.menu {                
                Sidebar::Trophies => {
                    ui.collapsing("Configure", |ui| {
                        ui.add_space(20.0);
                        ui.strong("Filter & Sort");
                        ui.add_space(10.0);
                        Grid::new("filter_sort")
                            .num_columns(7)
                            .striped(false)
                            .spacing([30.0, 10.0])
                            .show(ui, |ui| {
                                create_combo(ui, "Species", &mut self.species, Species::iter(), |x| { 
                                    self.trophy_filter.species = x;
                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());                                                                        
                                });
                                create_combo(ui, "Rating", &mut self.ratings, Ratings::iter(), |x| {
                                    self.trophy_filter.rating = x;
                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                });
                                ui.label("Grind");
                                ComboBox::new("grind_filter", "")
                                .selected_text(&self.grind)
                                .show_ui(ui, |ui| {
                                    ui.set_min_width(200.0);
                                    for g in self.grinds.iter() {
                                        let combo = ui.selectable_value(&mut self.grind, g.name.clone(), g.name.clone());
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
                                        self.species = self.trophy_filter.species;
                                        self.reserves = self.trophy_filter.reserve;
                                        self.ratings = self.trophy_filter.rating;
                                        self.sort_by = self.trophy_filter.sort_by;
                                        self.gender = self.trophy_filter.gender;
                                        self.grind = self.trophy_filter.grind.clone();
                                        self.filtered_trophies = self.trophies.clone();
                                    }   
                                });                            
                                ui.end_row(); 
                                create_combo(ui, "Reserve", &mut self.reserves, Reserves::iter(), |x| {
                                    self.trophy_filter.reserve = x;
                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                });
                                create_combo(ui, "Gender", &mut self.gender, Gender::iter(), |x| {
                                    self.trophy_filter.gender = x;
                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                });                                
                                create_combo(ui, "Sort By", &mut self.sort_by, SortBy::iter(), |x| {
                                    self.trophy_filter.sort_by = x;
                                    self.filtered_trophies = filter_data(&self.trophy_filter, self.trophies.clone());
                                });                                  
                                ui.end_row();  

                            });
                        ui.add_space(20.0);
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
                },
                Sidebar::Grinds => {
                    ui.collapsing("Create", |ui| {
                        ui.add_space(20.0);
                        Grid::new("grinds")
                        .num_columns(2)
                        .spacing([10.0, 10.0])
                        .show(ui, |ui| {
                            ui.label("Name");
                            ui.text_edit_singleline(&mut self.grind_name);
                            ui.end_row();
                            create_combo(ui, "Species", &mut self.grind_species.clone(), Species::iter(), |x| { 
                                self.grind_species = x;                                                                     
                            });           
                            ui.end_row();      
                            create_combo(ui, "Reserve", &mut self.grind_reserve.clone(), Reserves::iter(), |x| { 
                                self.grind_reserve = x;                                                                     
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
                                self.grind_reserve = Reserves::Unknown;
                            }
                        }                 
                        ui.add_space(20.0);
                        ui.separator();
                    });
                    ui.add_space(20.0);
                    let grinds = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .sense(Sense::click())
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
                                            data::stop_grind(grind.name.clone());
                                            grind.active = false;                                        
                                        }
                                    } else {
                                        ui.style_mut().visuals.widgets.hovered.weak_bg_fill = Color32::DARK_GREEN;
                                        if ui.button("Start").clicked() {
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
                }
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