use std::{process::{Child, Command, Stdio}, sync::Mutex};
use serde::{Deserialize, Serialize};
use crate::{car_logo::{reset_car_logo, overlay_car_logo}, team_name::update_team_name};

#[derive(Serialize, Deserialize, Clone)]

pub struct RaceTeam {
    name: String,
    logo_path: String,
    script_path: String,
}

#[derive(Clone, Serialize)]
struct RaceDriverStatus {
    driver_index: usize,
    state: String,
    port: Option<u16>,
}

struct RaceDriverManager {
    children: Vec<Mutex<Child>>,
    statuses: Vec<RaceDriverStatus>,
    race_running: bool,
}


#[tauri::command]
pub fn start_race(app: tauri::AppHandle, race_teams: Vec<RaceTeam>) -> Result<(), String> {
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
    tauri::async_runtime::spawn(async move {
        start_scripts(app, race_teams).await;
    });

    Ok(())
}


async fn start_scripts (app: tauri::AppHandle, race_teams: Vec<RaceTeam>) {
    // Then start each driver with a little delay in between.
    for (index, team) in race_teams.iter().enumerate() {
        if team.script_path.is_empty() {
            eprintln!("No script path provided for team {}, skipping driver start.", index);
            continue;
        }
        
        let _child = Command::new("python")
        .arg("-u") // unbuffered output
        .arg(&team.script_path)
        .arg("--port")
        .arg((3001 + index).to_string()) // assign ports 3001, 3002, ... to drivers
        .stdout(Stdio::piped())
        .spawn()
        .expect(&format!("Failed to start driver script for team {index}"));

        std::thread::sleep(std::time::Duration::from_millis(50)); // delay between starting drivers
    }
}