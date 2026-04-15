use std::fs;
use std::path::PathBuf;

pub fn update_team_name(car_index: u32, team_name: &str, exe_dir: &PathBuf) -> Result<(), String> {
    let scr_server_xml_path = exe_dir.join("torcs").join("drivers").join("scr_server").join("scr_server.xml");
    
    let content = fs::read_to_string(&scr_server_xml_path)
        .map_err(|e| format!("Failed to read XML: {}", e))?;
    
    let mut lines: Vec<String> = content.lines().map(|s| s.to_string()).collect();
    let name_line_index = 17 + (car_index as usize * 11);
    
    if name_line_index >= lines.len() {
        return Err("Car index out of bounds".into());
    }
    
    let mut line_parts: Vec<&str> = lines[name_line_index].split('"').collect();
    if line_parts.len() > 3 {
        line_parts[3] = team_name;
        lines[name_line_index] = line_parts.join("\"");
    }
    
    fs::write(&scr_server_xml_path, lines.join("\n"))
        .map_err(|e| format!("Failed to write XML: {}", e))?;
        
    Ok(())
}
