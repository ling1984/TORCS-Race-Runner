use std::sync::Mutex;
use crate::{track_banners::change_banners, 
    practice::{handle_params, start_practice, stop_practice, set_logo_path, PracticeDriverState}, 
    race::{start_race, stop_race, RaceState}};

mod team_name;
mod car_logo;
mod sgi_encoder;
mod track_banners;
mod race;
mod practice;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(PracticeDriverState {
            driver: Mutex::new(None),
            params: Mutex::new(None),
            logo_path: Mutex::new(None),
        })
        .manage(RaceState {
            children: Mutex::new(Vec::new()),
            race_running: Mutex::new(false),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![start_race, stop_race, handle_params, start_practice, stop_practice, set_logo_path, change_banners])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
