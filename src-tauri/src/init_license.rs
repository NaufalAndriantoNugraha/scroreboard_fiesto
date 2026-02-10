use md5;
use serde_json::json;
use std::fs;
use std::path::PathBuf;

pub fn init_license(app_handle: &tauri::AppHandle) -> Result<(), String> {
    let app_dir =
        tauri::api::path::app_data_dir(&app_handle.config()).ok_or("Cannot get app data dir")?;

    fs::create_dir_all(&app_dir).map_err(|e| e.to_string())?;

    let license_path: PathBuf = app_dir.join("app_session.json");
    if license_path.exists() {
        return Ok(());
    }

    // let disk_id = get_disk_id_windows().ok_or("Gagal mengambil Disk ID dari sistem Windows")?;

    let license_data = json!({
        // "expired_date_code": ""
        "session_code": ""
    });

    fs::write(
        license_path,
        serde_json::to_string_pretty(&license_data).unwrap(),
    )
    .map_err(|e| e.to_string())?;
    Ok(())
}

pub fn get_disk_id_windows() -> Option<String> {
    let output = std::process::Command::new("cmd")
        .args(["/C", "vol C:"])
        .output()
        .ok()?;

    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        let l = line.to_lowercase();
        if l.contains("serial number") || l.contains("nomor seri") {
            let serial = line.split_whitespace().last()?.trim().to_string();

            let salt = "Teguh";
            let combined = format!("{}{}", serial, salt);
            let digest = md5::compute(combined.as_bytes());

            return Some(format!("{:x}", digest));
        }
        // if line.to_lowercase().contains("volume serial number") {
        //     let serial = line.split("is").nth(1)?.trim().to_string();
        //     let salt = "Teguh";

        //     // Menggabungkan serial dan salt
        //     let combined = format!("{}{}", serial, salt);
        //     let digest = md5::compute(combined.as_bytes());

        //     // Hasil digest langsung bisa di-format ke hex string (32 karakter)
        //     return Some(format!("{:x}", digest));
        // }
    }
    None
}
