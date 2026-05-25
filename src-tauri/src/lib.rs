use crate::{
    practice::{handle_params, set_logo_path, start_practice, stop_practice, PracticeDriverState},
    race::{start_race, stop_race, RaceState},
    track_banners::change_banners,
};
use std::sync::Mutex;
use tokio::sync::Mutex as TokioMutex;
use tauri::{AppHandle, Manager, WindowEvent};

mod car_logo;
mod practice;
mod race;
mod sgi_encoder;
mod team_name;
mod track_banners;

async fn shutdown_sequence(app: AppHandle) {
    let practice_state = app.state::<PracticeDriverState>();
    let race_state = app.state::<RaceState>();

    // normal sync function
    let _ = stop_practice(practice_state);

    // async function
    let _ = stop_race(race_state).await;

    // exit after async work completes
    app.exit(0);
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .setup(|app| {
            let window = app.get_webview_window("main").unwrap();
            window.clone().on_window_event(move |event| {
                if let WindowEvent::CloseRequested { api, .. } = event {
                    println!("Window close requested");
                    api.prevent_close();
                    window.destroy().unwrap();
                    let app_handle = window.app_handle().clone();
                    tauri::async_runtime::spawn(async move {
                        shutdown_sequence(app_handle).await;
                    });
                }
            });

            Ok(())
        })
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
