use std::{process::{Child, Command}, sync::Mutex};
use tauri::{State};
use serde::{Deserialize, Serialize};

use crate::{car_logo::reset_car_logo, car_logo::overlay_car_logo, team_name::update_team_name};

mod team_name;
mod car_logo;
mod sgi_encoder;
mod track_banners;


// race
#[derive(Serialize, Deserialize, Clone)]
struct RaceTeam {
    name: String,
    logo_path: String,
    script_path: String,
}
#[tauri::command]
fn start_race(race_teams: Vec<RaceTeam>) -> Result<(), String> {
    // TODO Replace this with a set path stored globally.
    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();

    // Handle team name and logo first
    for (index, team) in race_teams.iter().enumerate() {
        
        let index_u32 = index as u32;
        update_team_name(index_u32, &team.name, &exe_dir)
            .map_err(|e| e.to_string())?;

        if team.logo_path.is_empty() {
            match reset_car_logo(index_u32, &exe_dir) {
                Ok(()) => println!("Car logo reset successfully."),
                Err(e) => eprintln!("Car logo reset error: {e}"),
            }
        }
        else {
            match overlay_car_logo(index_u32, &team.logo_path, &exe_dir) {
                Ok(()) => println!("Car logo for team {} overlayed successfully.", index_u32),
                Err(e) => eprintln!("Car logo overlay error for team {}: {e}", index_u32),
            }
        }
    }

    // Then start each driver with a little delay in between.
    for (index, team) in race_teams.iter().enumerate() {
        if team.script_path.is_empty() {
            eprintln!("No script path provided for team {}, skipping driver start.", index);
            continue;
        }
        
        let _child = Command::new("python")
        .arg(&team.script_path)
        .arg("--port")
        .arg((3001 + index).to_string()) // assign ports 3001, 3002, ... to drivers
        .spawn()
        .map_err(|e| e.to_string())?;

        std::thread::sleep(std::time::Duration::from_millis(50)); // delay between starting drivers
    }

    Ok(())
}


// practice

#[derive(Serialize, Deserialize, Clone)]
struct DriverParams {
    target_speed: i32,
    steer_gain: i32,
    centering_gain: f64,
    brake_threshold: f64,
    gear_thresholds: Vec<f64>,
    traction_control: bool,
    team_name: String,
}

struct DriverState {
    driver: Mutex<Option<Child>>, // later we do drivers: Mutex<HashMap<u32, Child>>
    params: Mutex<Option<DriverParams>>,
    logo_path: Mutex<Option<String>>,
}

#[tauri::command]
fn handle_params(params: DriverParams, state: State<DriverState>) -> Result<(), String> {
    let mut guard = state.params.lock().unwrap();
    *guard = Some(params);
    Ok(())
}

#[tauri::command]
fn start_practice(state: State<DriverState>) -> Result<(), String> {    
    // Get the current driver process if it exists
    let mut driver_guard = state.driver.lock().unwrap();
    if driver_guard.is_some() {
        return Err("Process already running".into());
    }

    // Get the stored parameters
    let params_guard = state.params.lock().unwrap();
    let params = params_guard.as_ref().ok_or("No parameters set")?;
    
    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();

    // -- Change team name and logo before starting the driver --
    update_team_name(0, &params.team_name, &exe_dir)
        .map_err(|e| e.to_string())?;

    // Get the stored logo path and change car logo if it exists
    let logo_path_guard = state.logo_path.lock().unwrap();

    // If there is some logo path (we set None if ""), we change the logo
    if let Some(ref logo_path) = *logo_path_guard{
        match overlay_car_logo(0, logo_path, &exe_dir) {
            Ok(()) => println!("Car logo overlayed successfully."),
            Err(e) => eprintln!("Car logo overlay error: {e}"),
        }
    } else { // else reset
        match reset_car_logo(0, &exe_dir) {
            Ok(()) => println!("Car logo reset successfully."),
            Err(e) => eprintln!("Car logo reset error: {e}"),
        }
    }

    // -- Start the driver script with parameters --

    // Serialize parameters to JSON
    let params_json = serde_json::to_string(params)
        .map_err(|e| e.to_string())?;

    let driver_script_path = exe_dir.join("gym_torcs").join("torcs_jm_par.py");
    // final racerunner.exe needs to be same dir as gym_torcs
    println!("Running driver script at: {:?}", driver_script_path);
    
    let child = Command::new("python")
        .arg(&driver_script_path)
        .arg("--parameters")
        .arg(&params_json)
        .spawn()
        .map_err(|e| e.to_string())?;

    *driver_guard = Some(child);

    Ok(())
}

#[tauri::command]
fn set_logo_path(path: String, state: State<DriverState>) -> Result<(), String> {
    let mut guard = state.logo_path.lock().unwrap();
    println!("path is {}", path);
    
    // we set it to none if it is empty
    // so that we can reset if empty
    *guard = if path.is_empty() {
        None
    } else {
        Some(path)
    };
    Ok(())
}

#[tauri::command]
fn stop_practice(state: State<DriverState>) -> Result<(), String> {
    let mut guard = state.driver.lock().unwrap();

    if let Some(child) = guard.as_mut() {
        child.kill().map_err(|e| e.to_string())?;
        child.wait().ok(); // cleanup

        *guard = None;
        Ok(())
    } else {
        Err("No process running".into())
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DriverState {
            driver: Mutex::new(None),
            params: Mutex::new(None),
            logo_path: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![start_race, handle_params, start_practice, stop_practice, set_logo_path, track_banners::change_banners])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
