#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod models;
mod data;

use egui::*;
use std::str::FromStr;
use eframe::Storage;
use std::fmt;
use strum::{IntoEnumIterator, VariantArray};
use models::*;
use data::*;
use egui_extras::{Column, TableBuilder};

const ICON: &[u8] = include_bytes!("../static/icon.png");
const SMALL_FONT: f32 = 14.0;
const MEDIUM_FONT: f32 = 16.0;

fn main() -> Result<(), eframe::Error> {
    let icon_data = eframe::icon_data::from_png_bytes(ICON).expect("Failed to load icon");
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_inner_size([1000.0, 850.0]).with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "Trophy Lodge",
        options,
        Box::new(|cc| {
            egui_extras::install_image_loaders(&cc.egui_ctx);
            Box::new(MyApp::new(cc))
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
            ui.set_min_width(200.0);
            combo_options(ui, value, values, capture);
        });
}

fn summary_metric(ui: &mut Ui, label: &str, value: String) {
    ui.horizontal(|ui| {
        ui.small(RichText::new(format!("{}", value)));    
        ui.small(RichText::new(format!("{}:", label)).strong());
    });
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
    ];
    cols.iter().map(|x| x.to_string()).collect()
}

#[derive(PartialEq)]
enum Sidebar {
    Trophies,
    Challenges,
    Friends,
    Settings,
}

struct MyApp {
    menu: Sidebar,
    species: Species,
    reserves: Reserves,
    ratings: Ratings,
    sort_by: SortBy,
    data: Vec<Trophy<'static>>,
    filtered_data: Vec<Trophy<'static>>,
    trophy_filter: TrophyFilter,
    trophy_cols: Vec<String>,
    selected_cols: Vec<String>,
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        let test_data = create_trophies(30);
        let filtered_data = test_data.clone();
        let trophy_filter = TrophyFilter::default();
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
            sort_by: SortBy::Date,
            data: test_data,
            filtered_data,
            trophy_filter,
            trophy_cols: available_cols(),
            selected_cols,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {        
        TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| { 
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.add(Image::new(include_image!("../static/logo2.png"))
                                .fit_to_original_size(0.8)
                        );
                        ui.add_space(10.0);
                    });
                    ui.add_space(15.0);
                    ui.horizontal_centered(|ui| {
                        ui.heading(RichText::new("Trophy Lodge"));
                        ui.small(RichText::new(format!("v{}", env!("CARGO_PKG_VERSION"))));
                    });
                    ui.with_layout(                        
                        Layout::default().with_cross_align(Align::RIGHT),
                        |ui| {
                            ui.small(RichText::new("mvision69").color(Color32::DEBUG_COLOR));
                            ui.add_space(5.0);
                            summary_metric(ui, "Trophies", 2000.to_string());
                            summary_metric(ui, "Diamonds", 1000.to_string());
                            summary_metric(ui, "Great Ones", 1.to_string());
                        }
                    );
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
                    ui.selectable_value(&mut self.menu, Sidebar::Challenges, "Challenges");
                    ui.add_space(5.0);
                    ui.selectable_value(&mut self.menu, Sidebar::Friends, "Friends");
                    ui.add_space(5.0);
                    ui.selectable_value(&mut self.menu, Sidebar::Settings, "Settings");
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
                            .num_columns(5)
                            .striped(false)
                            .spacing([30.0, 10.0])
                            .show(ui, |ui| {
                                create_combo(ui, "Species", &mut self.species, Species::iter(), |x| { 
                                    self.trophy_filter.species = x;
                                });
                                create_combo(ui, "Rating", &mut self.ratings, Ratings::iter(), |x| {
                                    self.trophy_filter.rating = x;
                                });
                                ui.vertical(|ui| {
                                    ui.add_space(5.0);
                                    if ui.button("Apply").clicked() {
                                        self.filtered_data = self.data.clone();
                                        if self.trophy_filter.species != Species::All {
                                            self.filtered_data.retain(|&x| x.species == self.trophy_filter.species);
                                        }
                                        if self.trophy_filter.reserve != Reserves::All {
                                            self.filtered_data.retain(|&x| x.reserve == self.trophy_filter.reserve);
                                        }
                                        if self.trophy_filter.rating != Ratings::All {
                                            self.filtered_data.retain(|&x| x.rating == self.trophy_filter.rating);
                                        }
                                        match self.trophy_filter.sort_by {
                                            SortBy::Date => self.filtered_data.sort_by(|&a, &b| b.date.cmp(&a.date)),
                                            SortBy::Score => self.filtered_data.sort_by(|a, b| b.score.partial_cmp(&a.score).unwrap()),
                                            SortBy::Weight => self.filtered_data.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap()),
                                            SortBy::Rating => self.filtered_data.sort_by(|a, b| b.rating.cmp(&a.rating)),
                                            SortBy::ShotDistance => self.filtered_data.sort_by(|a, b| b.shot_distance.partial_cmp(&a.shot_distance).unwrap()),
                                        }
                                    }    
                                });                            
                                ui.end_row(); 
                                create_combo(ui, "Reserve", &mut self.reserves, Reserves::iter(), |x| {
                                    self.trophy_filter.reserve = x;
                                });
                                create_combo(ui, "Sort By", &mut self.sort_by, SortBy::iter(), |x| {
                                    self.trophy_filter.sort_by = x;
                                });
                                ui.vertical(|ui| {
                                    ui.add_space(5.0);                                
                                    if ui.button("Reset").clicked() {
                                        self.trophy_filter = TrophyFilter::default();
                                        self.species = self.trophy_filter.species;
                                        self.reserves = self.trophy_filter.reserve;
                                        self.ratings = self.trophy_filter.rating;
                                        self.sort_by = self.trophy_filter.sort_by;
                                        self.filtered_data = self.data.clone();
                                    }
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
                    let trophies = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)                        
                        .sense(Sense::click())
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
                                        ui.add(Label::new(RichText::new(h).strong()).wrap(false))
                                    }); 
                                });
                            }
                        })
                        .body(|mut body| {
                            for trophy in self.filtered_data.iter() {
                                body.row(30.0, |mut row| {
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
                                            col_label(ui, trophy.score.to_string());
                                        });
                                    }
                                    if self.selected_cols.contains(&"Weight".to_string()) {
                                        row.col(|ui| { 
                                            col_label(ui, trophy.weight.to_string());
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
                                    if self.selected_cols.contains(&"Weapon Score".to_string()) {
                                        row.col(|ui| { 
                                            col_label(ui, trophy.weapon_score.to_string());
                                        });        
                                    }                                                
                                    if self.selected_cols.contains(&"Shot Distance".to_string()) {
                                        row.col(|ui| { 
                                            col_label(ui, trophy.shot_distance.to_string());
                                        });        
                                    }                                        
                                    if self.selected_cols.contains(&"Shot Damage".to_string()) {
                                        row.col(|ui| { 
                                            col_label(ui, trophy.shot_damage.to_string());
                                        });        
                                    } 
                                });
                            }
                        });
                }
                Sidebar::Challenges => {
                    ui.label(RichText::new("Challenges"));
                }
                Sidebar::Friends => {
                    ui.label(RichText::new("Friends"));
                }
                Sidebar::Settings => {
                    ui.label(RichText::new("Settings"));
                }
            }
        });

        TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(RichText::new("Attached to game and waiting for harvest...")
                    .color(Color32::LIGHT_YELLOW)
                    .size(SMALL_FONT)
                );
                ui.add_space(10.0);
            });
    }

    fn save(&mut self, storage: &mut dyn Storage) {
        if let Ok(value) = serde_json::to_string(&self.selected_cols) {
            storage.set_string("selected_cols", value);
        }
    }   
}