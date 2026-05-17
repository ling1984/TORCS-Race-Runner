use std::process::Stdio;
use serde::{Deserialize, Serialize};
use tokio::process::{Child, Command};
use tokio::io::{AsyncBufReadExt, BufReader};
use tokio::sync::Mutex;
use tauri::{Emitter, Manager};
use crate::{car_logo::{overlay_car_logo, reset_car_logo}, team_name::update_team_name};

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
    pub race_running: Mutex<bool>,
}


#[tauri::command]
pub async fn start_race(race_teams: Vec<RaceTeam>, app: tauri::AppHandle) -> Result<(), String> {
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
    let state_ref = app.state::<RaceState>();
    {
        *state_ref.race_running.lock().await = true; // TODO change this
    }
    let app_clone = app.clone();
    tauri::async_runtime::spawn(async move {
        if let Err(e) = start_scripts(app_clone, race_teams).await {
            eprintln!("Error starting scripts: {}", e);
        }
    });

    Ok(())
}


async fn start_scripts (app: tauri::AppHandle, race_teams: Vec<RaceTeam>) -> Result<(), Box<dyn std::error::Error>> {

    // Handling race state and locking children mutex to push new children to it.
    let race_state = app.state::<RaceState>();

    // Then start each driver with a little delay in between.
    for (index, team) in race_teams.iter().enumerate() {
        if team.script_path.is_empty() {
            eprintln!("No script path provided for team {}, skipping driver start.", index);
            continue;
        }
        
        let mut child = Command::new("python")
        .arg("-u") // unbuffered output
        .arg(&team.script_path)
        .arg("--port")
        .arg((3001 + index).to_string()) // assign ports 3001, 3002, ... to drivers
        .stdout(Stdio::piped())
        .spawn()
        .map_err(|e| format!("Failed to start driver script for team {index}: {e}"))?;
        

        let stdout = child.stdout.take().ok_or("Failed to get stdout")?;

        let mut reader = BufReader::new(stdout).lines();

        while let Ok(Some(line)) = reader.next_line().await {
            println!("{}",line);

            if line.contains("Waiting for server on") { // format is: Waiting for server on 3001............
                let port = extract_port(&line);

                let _ = app.emit(
                    "driver-status",
                    RaceDriverStatus {
                        index : index,
                        team_name: race_teams[index].name.clone(),
                        state: "connecting".into(),
                        port,
                    },
                );
            }

            if line.contains("Client connected on") { // format is: Client connected on 3001..............
                let port = extract_port(&line);

                let _ = app.emit(
                    "driver-status",
                    RaceDriverStatus {
                        index: index,
                        team_name: race_teams[index].name.clone(),
                        state: "connected".into(),
                        port,
                    },
                );

                break;
            }
        }
        {
            let mut race_children = race_state.children.lock().await;
            race_children.push(child);
        }

    }
    Ok(())
}

fn extract_port(line: &str) -> String {
    line.split("on ").nth(1).unwrap_or("").trim().into()
}

#[tauri::command]
pub async fn stop_race(race_state: tauri::State<'_, RaceState>) -> Result<(), String> {
    {
        *race_state.race_running.lock().await = false;
    }
    let mut children = race_state.children.lock().await;
    for child in children.iter_mut() {
        child.kill().await.map_err(|e| e.to_string())?;
        child.wait().await.ok(); // cleanup
    }
    children.clear();
    Ok(())
}