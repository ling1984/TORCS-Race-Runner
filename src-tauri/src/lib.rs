use std::{process::{Child, Command, Output}, sync::Mutex};
use tauri::State;
use serde::{Deserialize, Serialize};

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
fn start_racer(state: State<DriverState>) -> Result<(), String> {    
    // Get the current driver process if it exists
    let mut driver_guard = state.driver.lock().unwrap();
    if driver_guard.is_some() {
        return Err("Process already running".into());
    }

    // Get the stored parameters
    let params_guard = state.params.lock().unwrap();
    let params = params_guard.as_ref().ok_or("No parameters set")?;
    
    // -- Change team name and logo before starting the driver --
    let change_team_name_output = change_team_name(&params.team_name);
    if change_team_name_output.status.success() {
        println!("Team name change succeeded");
    } else {
        let stderr = String::from_utf8_lossy(&change_team_name_output.stderr);
        println!("Team name change failed: {:?}", stderr);
    }

    // Get the stored logo path and change car logo if it exists
    let logo_path_guard = state.logo_path.lock().unwrap();
    if let Some(ref logo_path) = *logo_path_guard {
        let change_car_logo_output = change_car_logo(logo_path);
        if change_car_logo_output.status.success() {
            println!("Car logo change succeeded");
        } else {
            let stderr = String::from_utf8_lossy(&change_car_logo_output.stderr);
            println!("Car logo change failed: {:?}", stderr);
        }
    }
    println!("Logo path is {:?}", *logo_path_guard);

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

fn change_team_name(team_name: &String) -> Output {
    // To change the team name, we pass in the new name from the start_racer method.
    // We need to call a python script that writes to the correct file. Format is:
    // python change_team_name.py --car_index 0..9 --team_name "New Team Name"
    // if team_name variable is empty string, we write scr_driver. This is handled in the python script, so just pass the name.
    // car_index is assumed to be 0, as this app is for single car, that is fine.
    // we need to call "python change_team_name.py --team_name {team_name}"
    println!("Changing team name to: {}", team_name);

    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();
    let change_team_name_script_path = exe_dir.join("change_team_name.py");

    // return the output of the command
    Command::new("python")
        .arg(&change_team_name_script_path)
        .arg("--team_name")
        .arg(&team_name)
        .output()
        .expect("failed to execute process")

}

fn change_car_logo(logo_path: &String) -> Output {
    // Similar to change_team_name, we call a python script to change the team logo. The script takes in the new logo path and copies it to the correct location.
    // python change_car_logo.py --image_path "path/to/logo.png"
    println!("Changing team logo to: {}", logo_path);

    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();
    let change_car_logo_script_path = exe_dir.join("change_car_logo.py");

    Command::new("python")
        .arg(&change_car_logo_script_path)
        .arg("--image_path")
        .arg(&logo_path)
        .output()
        .expect("failed to execute process")
}

#[tauri::command]
fn set_logo_path(path: String, state: State<DriverState>) -> Result<(), String> {
    let mut guard = state.logo_path.lock().unwrap();
    *guard = Some(path);
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
            logo_path: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_dialog::init())
        .invoke_handler(tauri::generate_handler![handle_params, start_racer, stop_racer, set_logo_path])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
