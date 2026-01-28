use hex;
use serde_json::json;
use sha2::{Digest, Sha256};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

pub fn init_license(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let app_dir =
        tauri::api::path::app_data_dir(&app_handle.config()).ok_or("Cannot get app data dir")?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    let license_path: PathBuf = app_dir.join("license.json");
    if license_path.exists() {
        return Ok(());
    }

    let disk_id =
        get_disk_id_windows(Some("Teguh")).ok_or("Gagal mengambil Disk ID dari sistem Windows")?;

    let license_data = json!({
        "information_number": disk_id,
        "expired_date": ""
    });

    fs::write(
        license_path,
        serde_json::to_string_pretty(&license_data).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

fn get_disk_id_windows(salt: Option<&str>) -> Option<String> {
    let output = Command::new("cmd").args(["/C", "vol C:"]).output().ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.to_lowercase().contains("volume serial number") {
            let serial = line.split("is").nth(1)?.trim().to_string();

            return Some(match salt {
                Some(s) if !s.is_empty() => {
                    let mut hasher = Sha256::new();
                    hasher.update(serial.as_bytes());
                    hasher.update(s.as_bytes());
                    hex::encode(hasher.finalize())
                }
                _ => serial,
            });
        }
    }
    None
}
