// hide console on Windows in release
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, HardwareAcceleration};

mod app_logic;

fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(640_f32, 480_f32)),
        resizable: true,
        centered: true,
        hardware_acceleration: HardwareAcceleration::Preferred,
        ..Default::default()
    };
    eframe::run_native(
        "My egui App",
        options,
        Box::new(|_cc| Box::<app_logic::MyApp>::default()),
    )
}