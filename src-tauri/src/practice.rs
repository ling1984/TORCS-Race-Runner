use serde::{Deserialize, Serialize};
#[cfg(windows)]
use std::os::windows::process::CommandExt;
use std::{
    path::PathBuf, process::{Child, Command}, sync::Mutex
};
use tauri::State;
use tauri_plugin_store::StoreExt;


use crate::{
    car_logo::{overlay_car_logo, reset_car_logo},
    team_name::update_team_name,
};

// pub because we initialise it in lib.rs
pub struct PracticeDriverState {
    pub driver: Mutex<Option<Child>>,
    pub params: Mutex<Option<PracticeDriverParams>>,
    pub logo_path: Mutex<Option<String>>,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct PracticeDriverParams {
    target_speed: i32,
    steer_gain: i32,
    centering_gain: f64,
    brake_threshold: f64,
    gear_thresholds: Vec<f64>,
    traction_control: bool,
    team_name: String,
}

#[tauri::command]
pub fn handle_params(
    params: PracticeDriverParams,
    state: State<PracticeDriverState>,
) -> Result<(), String> {
    let mut guard = state.params.lock().unwrap();
    *guard = Some(params);
    Ok(())
}

#[tauri::command]
pub fn start_practice(state: State<PracticeDriverState>, app: tauri::AppHandle) -> Result<(), String> {
    // Get the current driver process if it exists
    let mut driver_guard = state.driver.lock().unwrap();
    if driver_guard.is_some() {
        return Err("Process already running".into());
    }

    // Get the stored parameters
    let params_guard = state.params.lock().unwrap();
    let params = params_guard.as_ref().ok_or("No parameters set")?;

    let store = app
        .store("settings.json")
        .map_err(|e| format!("Failed to access store: {e}"))?;

    let exe_dir: PathBuf = store
        .get("folder_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .ok_or_else(|| "folder_path missing or not a string".to_string())
        .map(PathBuf::from)?;
    
    // -- Change team name and logo before starting the driver --
    update_team_name(0, &params.team_name, &exe_dir).map_err(|e| e.to_string())?;

    // Get the stored logo path and change car logo if it exists
    let logo_path_guard = state.logo_path.lock().unwrap();

    // If there is some logo path (we set None if ""), we change the logo
    if let Some(ref logo_path) = *logo_path_guard {
        match overlay_car_logo(0, logo_path, &exe_dir) {
            Ok(()) => println!("Car logo overlayed successfully."),
            Err(e) => eprintln!("Car logo overlay error: {e}"),
        }
    } else {
        // else reset
        match reset_car_logo(0, &exe_dir) {
            Ok(()) => println!("Car logo reset successfully."),
            Err(e) => eprintln!("Car logo reset error: {e}"),
        }
    }

    // -- Start the driver script with parameters --

    // Serialize parameters to JSON
    let params_json = serde_json::to_string(params).map_err(|e| e.to_string())?;

    let driver_script_path = exe_dir.join("gym_torcs").join("torcs_jm_par.py");
    // final racerunner.exe needs to be same dir as gym_torcs
    println!("Running driver script at: {:?}", driver_script_path);

    // get python alias
    let store = app
        .store("settings.json")
        .map_err(|e| format!("Failed to access store: {e}"))?;

    let mut python_alias = store
        .get("python_alias")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .unwrap_or_else(|| "python".to_string());
    if python_alias.is_empty() {
        python_alias = "python".to_string();
    }

    #[cfg(windows)]
    const CREATE_NO_WINDOW: u32 = 0x08000000;

    let mut cmd = Command::new(&python_alias);
    cmd.arg(&driver_script_path)
        .arg("--parameters")
        .arg(&params_json);

    // If on windows, do not create a window when running the script
    #[cfg(windows)]
    cmd.creation_flags(CREATE_NO_WINDOW);

    let child = cmd.spawn()
        .map_err(|e| e.to_string())?;

    *driver_guard = Some(child);

    Ok(())
}

#[tauri::command]
pub fn set_logo_path(path: String, state: State<PracticeDriverState>) -> Result<(), String> {
    let mut guard = state.logo_path.lock().unwrap();
    println!("path is {}", path);

    // we set it to none if it is empty
    // so that we can reset if empty
    *guard = if path.is_empty() { None } else { Some(path) };
    Ok(())
}

#[tauri::command]
pub fn stop_practice(state: State<PracticeDriverState>) -> Result<(), String> {
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
