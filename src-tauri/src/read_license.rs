use serde_json::Value;
use std::fs;
use std::path::PathBuf;

pub fn read_license(app_handle: &tauri::AppHandle) -> Result<Value, String> {
    let app_dir =
        tauri::api::path::app_data_dir(&app_handle.config()).ok_or("Cannot get app data dir")?;

    let license_path: PathBuf = app_dir.join("app_session.json");

    if !license_path.exists() {
        return Err("License file not found".to_string());
    }

    let content = fs::read_to_string(&license_path).map_err(|e| e.to_string())?;

    let json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    Ok(json)
}
