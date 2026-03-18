use std::{process::{Child, Command}, sync::Mutex};

use tauri::State;

struct DriverState {
    driver: Mutex<Option<Child>>,  // later we do drivers: Mutex<HashMap<u32, Child>>
}

#[tauri::command]
fn start_racer(state: State<DriverState>) -> Result<(), String> {    
    // Get the current driver process if it exists
    let mut guard = state.driver.lock().unwrap();
    if guard.is_some() {
        return Err("Process already running".into());
    }
    // Run the process if it doesn't exist
    // final racerunner.exe needs to be same dir as gym_torcs

    // get the current exe path then go up one
    let exe_dir = std::env::current_exe()
        .expect("can't get exe path")
        .parent()
        .expect("exe has no parent")
        .to_path_buf();
    let driver_script_path = exe_dir.join("gym_torcs").join("torcs_jm_par.py");
    println!("Running driver script at: {:?}", driver_script_path);
    let child = Command::new("python")
        .arg(&driver_script_path)
        .spawn()
        .map_err(|e| e.to_string())?;

    *guard = Some(child);

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

// fn run_python_driver_script() -> Child {
//     let port = format!("3001");
//     // get the current exe path then go up one
//     let exe_dir = std::env::current_exe()
//         .expect("can't get exe path")
//         .parent()
//         .expect("exe has no parent")
//         .to_path_buf();

//     // final racerunner.exe needs to be same dir as gym_torcs
//     let driver_script_path = exe_dir.join("gym_torcs").join("torcs_jm_par.py");

//     Command::new("python")
//         .arg(&driver_script_path)
//         .arg("--port")
//         .arg(port)
//         .arg("--features")
//         .arg(format!("{},30,0.20,0.9,0,20,40,80,100,180,true", 180))
//         .spawn()
//         .expect("failed to execute process")

// }


#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(DriverState {
            driver: Mutex::new(None),
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![start_racer, stop_racer])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
