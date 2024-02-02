#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window

mod models;
mod data;

use egui::*;
use std::convert::Into;
use std::fmt;
use strum::IntoEnumIterator;
use models::*;
use data::*;
use egui_extras::{Column, TableBuilder};

const ICON: &[u8] = include_bytes!("../static/icon.png");
const SMALL_FONT: f32 = 14.0;
const MEDIUM_FONT: f32 = 16.0;

fn main() -> Result<(), eframe::Error> {
    let icon_data = eframe::icon_data::from_png_bytes(ICON).expect("Failed to load icon");
    let options = eframe::NativeOptions {
        viewport: ViewportBuilder::default().with_icon(icon_data),
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

fn create_combo<T, I, F: FnMut(T)>(ui: &mut Ui, label: &str, value: &mut T, values: I, mut capture: F)
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
    ui.small(RichText::new(format!("{}:", label)).strong());
    ui.small(RichText::new(format!("{}", value)));    
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
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        let test_data = create_trophies(30);
        let mut filtered_data = test_data.clone();
        set_style(ctx);

        Self { 
            menu: Sidebar::Trophies, 
            species: Species::All,
            reserves: Reserves::All,
            ratings: Ratings::All,
            sort_by: SortBy::Date,
            data: test_data,
            filtered_data: filtered_data,
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &Context, _frame: &mut eframe::Frame) {
        TopBottomPanel::top("top_panel")
            .resizable(true)
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
                    });
                    ui.add_space(100.0);
                    ui.vertical(|ui| {
                        ui.add_space(20.0);                        
                        Grid::new("summary")
                            .num_columns(3)
                            .striped(false)
                            .spacing([5.0, 10.0])
                            .show(ui, |ui| {
                                summary_metric(ui, "Trophies", 300.to_string());      
                                summary_metric(ui, "Top Species", Species::RedDeer.to_string());   
                                summary_metric(ui, "Challenges Active", 3.to_string());                       
                                ui.end_row();
                                summary_metric(ui, "Diamonds", 10.to_string());
                                summary_metric(ui, "Top Reserve", Reserves::SilverRidgePeaks.to_string());    
                                summary_metric(ui, "Challenges Won", 2.to_string());                      
                                ui.end_row();        
                                summary_metric(ui, "Great Ones", 1.to_string());
                                summary_metric(ui, "Top Weapon", ".300 Magnum".to_string()); 
                                summary_metric(ui, "Challenge Invites", 0.to_string());
                                ui.end_row();
                            });
                    });                    
                    ui.with_layout(
                        Layout::default().with_cross_align(Align::RIGHT),
                        |ui| {
                            ui.small(RichText::new("mvision69").color(Color32::DARK_GREEN));
                            if ui.link(RichText::new("2 friends online").small()).clicked() {
                                println!("Friends clicked");
                            }
            
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
                    ui.collapsing("Filter & Sort", |ui| {
                        ui.add_space(10.0);
                        Grid::new("filter_sort")
                            .num_columns(4)
                            .striped(false)
                            .spacing([30.0, 10.0])
                            .show(ui, |ui| {
                                create_combo(ui, "Species", &mut self.species, Species::iter(), |x| { 
                                    self.filtered_data = self.filtered_data.iter().filter(|a| a.species == x).map(|a| a.clone()).collect();
                                    println!("Selected species: {:?}", x);
                                });
                                // create_combo(self, ui, "Rating", &mut self.ratings, Ratings::iter());
                                // ui.end_row(); 
                                // create_combo(self, ui, "Reserve", &mut self.reserves, Reserves::iter());
                                // create_combo(self, ui, "Sort By", &mut self.sort_by, SortBy::iter());
                                // ui.end_row();    
                            });
                    });               

                    ui.add_space(20.0);
                    let trophies = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .sense(Sense::click())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto())
                        .column(Column::auto());
                    trophies
                        .header(30.0, |mut header| {
                            header.col(|ui| { ui.vertical_centered( |ui| ui.strong("Species")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.strong("Reserve")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.strong("Rating")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.strong("Score")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.strong("Weight")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.label("Rewards")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.label("Hunt")); });
                            header.col(|ui| { ui.vertical_centered(|ui| ui.label("Images")); });
                        })
                        .body(|mut body| {
                            for trophy in self.data.iter() {
                                body.row(30.0, |mut row| {
                                    row.col(|ui| { 
                                        ui.vertical_centered(|ui| ui.label(trophy.species.to_string())); 
                                    });
                                    row.col(|ui| { 
                                        ui.vertical_centered(|ui| ui.label(trophy.reserve.to_string())); 
                                    });
                                    row.col(|ui| { 
                                        ui.vertical_centered(|ui| ui.label(trophy.rating.to_string())); 
                                    });
                                    row.col(|ui| { 
                                        ui.horizontal_centered(|ui| ui.label(trophy.score.to_string())); 
                                    });
                                    row.col(|ui| { 
                                        ui.horizontal_centered(|ui| ui.label(trophy.weight.to_string())); 
                                    });
                                    row.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            if ui.button(RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("Rewards clicked");
                                            }
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            if ui.button(RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("Hunt clicked");
                                            }
                                        });
                                    });    
                                    row.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            if ui.button(RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("screenshots clicked");
                                            }
                                        });
                                    });                                                                     
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
                    .size(SMALL_FONT)
                );
                ui.add_space(10.0);
            });
    }
}