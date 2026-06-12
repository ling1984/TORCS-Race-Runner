use crate::{
    car_logo::{overlay_car_logo, reset_car_logo},
    team_name::update_team_name,
};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, process::Stdio};
use tauri::{Emitter, Manager};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::process::{Child, ChildStdout, Command};
use tokio::sync::Mutex;
use tauri_plugin_store::StoreExt;

#[derive(Serialize, Deserialize, Clone)]

pub struct RaceTeam {
    name: String,
    logo_path: String,
    script_path: String,
}

#[derive(Clone, Serialize)]
struct RaceDriverStatus {
    index: usize,
    team_name: String,
    state: String,
    port: String,
}

pub struct RaceState {
    pub children: Mutex<Vec<Child>>,
    pub is_running: Mutex<bool>,
}

#[tauri::command]
pub async fn start_race(
    race_teams: Vec<RaceTeam>,
    app: tauri::AppHandle,
    race_state: tauri::State<'_, RaceState>,
) -> Result<(), String> {

    let store = app
        .store("settings.json")
        .map_err(|e| format!("Failed to access store: {e}"))?;

    let exe_dir: PathBuf = store
        .get("folder_path")
        .and_then(|v| v.as_str().map(|s| s.to_string()))
        .ok_or_else(|| "folder_path missing or not a string".to_string())
        .map(PathBuf::from)?;

    // Handle team name and logo first
    for (index, team) in race_teams.iter().enumerate() {
        let index_u32 = index as u32;
        update_team_name(index_u32, &team.name, &exe_dir).map_err(|e| e.to_string())?;

        if team.logo_path.is_empty() {
            match reset_car_logo(index_u32, &exe_dir) {
                Ok(()) => println!("Car logo reset successfully."),
                Err(e) => eprintln!("Car logo reset error: {e}"),
            }
        } else {
            match overlay_car_logo(index_u32, &team.logo_path, &exe_dir) {
                Ok(()) => println!("Car logo for team {} overlayed successfully.", index_u32),
                Err(e) => eprintln!("Car logo overlay error for team {}: {e}", index_u32),
            }
        }
    }

    {
        // sets is_running to true
        *race_state.is_running.lock().await = true;
    }

    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start_scripts(app_clone, race_teams).await {
            eprintln!("Error starting scripts: {}", e);
        }
    });

    Ok(())
}

async fn start_scripts(
    app: tauri::AppHandle,
    race_teams: Vec<RaceTeam>,
) -> Result<(), Box<dyn std::error::Error>> {

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

    // Handling race state and locking children mutex to push new children to it.
    let race_state = app.state::<RaceState>();

    // Then start each driver with a little delay in between.
    for (index, team) in race_teams.iter().enumerate() {
        if team.script_path.is_empty() {
            eprintln!(
                "No script path provided for team {}, skipping driver start.",
                index
            );
            continue;
        }

        let stdout: ChildStdout;
        // only start the process if is_running is true, else return ok()
        // solves bug where because we starting drivers sequentially, stopping the first driver without connecting would just start the next one.
        if *race_state.is_running.lock().await {
            // Without this {}, it is possible to start a process -> stop the race (kill children) -> add process to children
            // therefore leaving the process alive when it shouldn't be
            {
                let mut race_children = race_state.children.lock().await;
                let mut cmd = Command::new(&python_alias);

                cmd.arg("-u") // unbuffered output
                    .arg(&team.script_path)
                    .arg("--port")
                    .arg((3001 + index).to_string()) // assign ports 3001, 3002, ... to drivers
                    .stdout(Stdio::piped());

                // If on windows, do not create a window when running the script
                #[cfg(windows)]
                cmd.creation_flags(CREATE_NO_WINDOW);

                let mut child = cmd.spawn()
                    .map_err(|e| format!("Failed to start driver script for team {index}: {e}"))?;

                stdout = child.stdout.take().ok_or("Failed to get stdout")?;

                race_children.push(child);
            }
        } else {
            return Ok(());
        }

        let mut reader = BufReader::new(stdout).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("scr_driver {}: {}", index, line);

            if line.contains("Waiting for server on") {
                // format is: Waiting for server on 3001............
                let port = extract_port(&line);

                let _ = app.emit(
                    "driver-status",
                    RaceDriverStatus {
                        index,
                        team_name: race_teams[index].name.clone(),
                        state: "connecting".into(),
                        port,
                    },
                );
            }

            if line.contains("Client connected on") {
                // format is: Client connected on 3001..............
                let port = extract_port(&line);

                let _ = app.emit(
                    "driver-status",
                    RaceDriverStatus {
                        index,
                        team_name: race_teams[index].name.clone(),
                        state: "connected".into(),
                        port,
                    },
                );

                // If we drop reader, we get an OS error in the python script if they print anything
                // so we keep reader going in a background thread.
                tokio::spawn(async move {
                    while let Ok(Some(line)) = reader.next_line().await {
                        // format: . . Server has stopped the race on 3001. You were in 1 place.
                        // if we have app reference, we could emit a finish event here with the final position?
                        println!("scr_driver {}: {}", index, line);
                    }
                });

                break;
            }
        }
    }
    Ok(())
}

fn extract_port(line: &str) -> String {
    line.split("on ").nth(1).unwrap_or("").trim().into()
}

#[tauri::command]
pub async fn stop_race(race_state: tauri::State<'_, RaceState>) -> Result<(), String> {
    println!("Stopping race");
    *race_state.is_running.lock().await = false;
    let mut children = race_state.children.lock().await;
    for child in children.iter_mut() {
        child.kill().await.map_err(|e| e.to_string())?;
        child.wait().await.ok(); // cleanup
    }
    println!("children pre-clearing: {}", children.len());
    children.clear();
    println!("children len: {}", children.len());
    Ok(())
}
