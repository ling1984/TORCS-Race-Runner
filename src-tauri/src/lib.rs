use crate::{
    practice::{handle_params, set_logo_path, start_practice, stop_practice, PracticeDriverState},
    race::{start_race, stop_race, RaceState},
    track_banners::change_banners,
};
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;

mod car_logo;
mod practice;
mod race;
mod sgi_encoder;
mod team_name;
mod track_banners;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .manage(PracticeDriverState {
            driver: Mutex::new(None),
            params: Mutex::new(None),
            logo_path: Mutex::new(None),
        })
        .manage(RaceState {
            children: TokioMutex::new(Vec::new()),
            is_running: TokioMutex::new(false),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![
            start_race,
            stop_race,
            handle_params,
            start_practice,
            stop_practice,
            set_logo_path,
            change_banners
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
