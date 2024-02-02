#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window

mod models;
mod data;

use egui::{FontFamily, FontId, TextStyle, Color32};
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
        viewport: egui::ViewportBuilder::default().with_icon(icon_data),
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

fn set_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Heading, FontId::new(40.0, FontFamily::Proportional)),
        (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Button, FontId::new(20.0, FontFamily::Proportional)),
        (TextStyle::Small, FontId::new(MEDIUM_FONT, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

fn combo_options<I, T>(ui: &mut egui::Ui, current: &mut T, values: I) 
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    for item in values {
        ui.selectable_value(current, item, item.to_string());        
    }
}

fn create_combo<T, I>(ui: &mut egui::Ui, label: &str, value: &mut T, values: I) 
where I: Iterator<Item = T>,
      T: fmt::Display + PartialEq + Copy {
    ui.label(label);
    egui::ComboBox::new(format!("{}_filter", label.to_lowercase()), "")
        .selected_text(value.to_string())
        .show_ui(ui, |ui| {
            ui.set_min_width(200.0);
            combo_options(ui, value, values);
        });    
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
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        set_style(ctx);

        Self { 
            menu: Sidebar::Trophies, 
            species: Species::All,
            reserves: Reserves::All,
            ratings: Ratings::All,
            sort_by: SortBy::Date,
            data: create_trophies(30),
        }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::TopBottomPanel::top("top_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.horizontal_wrapped(|ui| {
                    ui.vertical(|ui| {
                        ui.add_space(10.0);
                        ui.add(egui::Image::new(egui::include_image!("../static/logo2.png"))
                                .fit_to_original_size(0.8)
                        );
                        ui.add_space(10.0);
                    });
                    ui.add_space(15.0);
                    ui.horizontal_centered(|ui| {
                        ui.heading(egui::RichText::new("Trophy Lodge"));
                    });
                    ui.add_space(20.0);
                    
                    ui.with_layout(
                        egui::Layout::default().with_cross_align(egui::Align::RIGHT),
                        |ui| {
                            ui.label(egui::RichText::new("mvision69").color(Color32::DARK_GREEN).size(SMALL_FONT));
                            ui.label(egui::RichText::new("100 kills").size(SMALL_FONT));
                            ui.label(egui::RichText::new("2 diamonds").size(SMALL_FONT));
                            ui.label(egui::RichText::new("1 great one").size(SMALL_FONT));
            
                        }
                    );
                });
            });
        egui::SidePanel::left("left_panel")
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

        egui::CentralPanel::default().show(ctx, |ui| {
            match self.menu {
                Sidebar::Trophies => {
                    ui.collapsing("Filter & Sort", |ui| {
                        ui.add_space(10.0);
                        egui::Grid::new("filter_sort")
                            .num_columns(4)
                            .striped(false)
                            .spacing([30.0, 10.0]) // horizontal, vertical
                            .show(ui, |ui| {
                                create_combo(ui, "Species", &mut self.species, Species::iter());
                                create_combo(ui, "Rating", &mut self.ratings, Ratings::iter());
                                ui.end_row(); 
                                create_combo(ui, "Reserve", &mut self.reserves, Reserves::iter());
                                create_combo(ui, "Sort By", &mut self.sort_by, SortBy::iter());
                                ui.end_row();    
                            });
                    });               

                    ui.add_space(20.0);
                    let trophies = TableBuilder::new(ui)
                        .striped(true)
                        .resizable(true)
                        .sense(egui::Sense::click())
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
                                            if ui.button(egui::RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("Rewards clicked");
                                            }
                                        });
                                    });
                                    row.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            if ui.button(egui::RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("Hunt clicked");
                                            }
                                        });
                                    });    
                                    row.col(|ui| {
                                        ui.vertical_centered(|ui| {
                                            if ui.button(egui::RichText::new("view").size(MEDIUM_FONT)).clicked() {
                                                println!("screenshots clicked");
                                            }
                                        });
                                    });                                                                     
                                });
                            }
                        });
                }
                Sidebar::Challenges => {
                    ui.label(egui::RichText::new("Challenges"));
                }
                Sidebar::Friends => {
                    ui.label(egui::RichText::new("Friends"));
                }
                Sidebar::Settings => {
                    ui.label(egui::RichText::new("Settings"));
                }
            }
        });

        egui::TopBottomPanel::bottom("bottom_panel")
            .resizable(false)
            .show(ctx, |ui| {
                ui.add_space(10.0);
                ui.label(egui::RichText::new("Attached to game and waiting for harvest...")
                    .size(SMALL_FONT)
                );
                ui.add_space(10.0);
            });
    }
}