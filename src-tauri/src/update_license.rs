use serde_json::{json, Value};
use std::fs;
use std::path::PathBuf;

pub fn update_license(
    app_handle: &tauri::AppHandle,
    expired_date_code: String,
) -> Result<(), String> {
    let app_dir =
        tauri::api::path::app_data_dir(&app_handle.config()).ok_or("Cannot get app data dir")?;

    let license_path: PathBuf = app_dir.join("app_session.json");

    if !license_path.exists() {
        return Err("License file not found".to_string());
    }

    let content = fs::read_to_string(&license_path).map_err(|e| e.to_string())?;

    let mut json: Value = serde_json::from_str(&content).map_err(|e| e.to_string())?;

    json["session_code"] = json!(expired_date_code);

    fs::write(&license_path, serde_json::to_string_pretty(&json).unwrap())
        .map_err(|e| e.to_string())?;

    Ok(())
}
