use std::{process::{Child, Command}, sync::Mutex};
use tauri::State;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone)]
struct DriverParams {
    target_speed: i32,
    steer_gain: i32,
    centering_gain: f64,
    brake_threshold: f64,
    gear_thresholds: Vec<f64>,
    traction_control: bool,
}

struct DriverState {
    driver: Mutex<Option<Child>>, // later we do drivers: Mutex<HashMap<u32, Child>>
    params: Mutex<Option<DriverParams>>,
}

#[tauri::command]
fn handle_params(params: DriverParams, state: State<DriverState>) -> Result<(), String> {
    let mut guard = state.params.lock().unwrap();
    *guard = Some(params);
    Ok(())
}

#[tauri::command]
fn start_racer(state: State<DriverState>) -> Result<(), String> {    
    // Get the current driver process if it exists
    let mut driver_guard = state.driver.lock().unwrap();
    if driver_guard.is_some() {
        return Err("Process already running".into());
    }

    // Get the stored parameters
    let params_guard = state.params.lock().unwrap();
    let params = params_guard.as_ref().ok_or("No parameters set")?;
    
    // Serialize parameters to JSON
    let params_json = serde_json::to_string(params)
        .map_err(|e| e.to_string())?;

    // Get the current exe path then go up one
    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();
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
fn stop_racer(state: State<DriverState>) -> Result<(), String> {
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
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![handle_params, start_racer, stop_racer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
