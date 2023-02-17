// hide console on Windows in release
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eframe::{egui, HardwareAcceleration};
use self_update::cargo_crate_version;

mod app_logic;

fn main() -> Result<(), anyhow::Error> {
    let _status = self_update::backends::github::Update::configure()
        .repo_owner("developerx-official")
        .repo_name("ui_test")
        .bin_name("ui_test")
        .show_download_progress(true)
        .current_version(cargo_crate_version!())
        .build()?
        .update()?;
    #[cfg(target_os = "windows")]
    winconsole::window::hide();
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
    .unwrap();
    Ok(())
}
