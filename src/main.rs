#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window

use egui::{FontFamily, FontId, TextStyle};

const ICON: &[u8] = include_bytes!("../static/icon.png");

fn main() -> Result<(), eframe::Error> {
    let icon_data = eframe::icon_data::from_png_bytes(ICON).expect("Failed to load icon");
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]).with_icon(icon_data),
        ..Default::default()
    };
    eframe::run_native(
        "Trophy Lodge Tracker",
        options,
        Box::new(|cc| Box::new(MyApp::new(cc))),
    )
}

fn set_style(ctx: &egui::Context) {
    let mut style = (*ctx.style()).clone();
    style.text_styles = [
        (TextStyle::Body, FontId::new(20.0, FontFamily::Proportional)),
    ].into();
    ctx.set_style(style);
}

struct MyApp {
    name: String,
}
impl MyApp {
    fn new(cc: &eframe::CreationContext<'_>) -> Self {
        let ctx = &cc.egui_ctx;
        set_style(ctx);
        
        Self { name: String::from("Hello") }
    }
}
impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(&self.name);
        });
    }
}