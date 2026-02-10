// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use futures_util::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use serialport::SerialPort;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tauri::{AppHandle, Manager, State};
use tokio::net::TcpListener;
use tokio::sync::mpsc::UnboundedSender;
use tokio::time::{sleep, Duration};
use tokio_tungstenite::accept_async;
use tokio_tungstenite::tungstenite::Message;

mod decrypt_license;
mod init_license;
mod print_with_rust;
mod rabbitmq;
mod read_license;
mod serial;
mod update_license;
mod utils;
mod web_server;
mod ws_server;

#[derive(Serialize, Deserialize, Debug, Clone)]
struct AppState {
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    event_id: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
}
#[derive(Serialize, Deserialize, Debug)]
struct ConfigState {
    file_path: String,
}

#[derive(Clone)]
struct WsState {
    senders: Arc<Mutex<Vec<UnboundedSender<Message>>>>,
}

#[tauri::command]
fn init_license(app: tauri::AppHandle) -> Result<(), String> {
    init_license::init_license(&app)
}

#[tauri::command]
fn read_license(app_handle: tauri::AppHandle) -> Result<serde_json::Value, String> {
    read_license::read_license(&app_handle)
}

#[tauri::command]
fn get_date_from_hash(token: String, info_number: String) -> String {
    decrypt_license::get_date_from_hash(&token, &info_number)
}

#[tauri::command]
fn get_original_hash(token: String) -> String {
    decrypt_license::get_original_hash(&token)
}

#[tauri::command]
fn update_license(app: tauri::AppHandle, expired_date_code: String) -> Result<(), String> {
    update_license::update_license(&app, expired_date_code)
}

#[tauri::command]
fn get_disk_id_windows() -> Option<String> {
    init_license::get_disk_id_windows()
}

#[tauri::command]
fn list_serial_ports() -> Result<Vec<String>, String> {
    serial::list_ports()
}

#[tauri::command]
fn print_with_rust(message: String) {
    print_with_rust::print_with_rust(&message);
}

#[tauri::command]
fn connect_serial_port(
    port_name: String,
    // port_map: State<serial::PortMap>,
) -> Result<String, String> {
    match serial::connect_port(&port_name) {
        Ok(_) => Ok(format!("Successfully connected to {}", port_name)),
        Err(e) => Err(e),
    }
}

#[tauri::command]
fn disconnect_serial_port(port_map: State<serial::PortMap>) -> Result<String, String> {
    serial::disconnect_all_ports(&port_map)
        .map(|_| "Successfully disconnected from all ports".to_string())
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn trigger_alarm(port_name: String, duration: String) -> Result<String, String> {
    let duration: u64 = match duration.parse::<u64>() {
        Ok(num) => num,
        Err(e) => return Err(e.to_string()),
    };

    match serial::trigger_serial_alarm(&port_name, duration).await {
        Ok(_) => Ok("Successfully triggered alarm".to_string()),
        Err(e) => Err(e.to_string()),
    }
}

#[tauri::command]
fn toggle_fullscreen(window: tauri::Window) {
    if let Some(is_fullscreen) = window.is_fullscreen().ok() {
        window.set_fullscreen(!is_fullscreen).ok();
    }
}

// #[tauri::command]
// fn update_time(
//     time: String,
//     // state: tauri::State<'_, Arc<Mutex<AppState>>>,
//     ws_state: tauri::State<'_, WsState>,
// ) -> Result<(), String> {
//     let state = state.lock().unwrap();
//     let host = state.rabbitmq_host.clone();
//     let username = state.rabbitmq_username.clone();
//     let password = state.rabbitmq_password.clone();
//     let routing_key = format!("sportkit.basket.{}.{}.time", state.event_id, state.field_id);

//     let time_for_rabbit = time.clone();

//     tokio::spawn(async move {
//         let _ =
//             rabbitmq::produce_to_rabbitmq(host, username, password, routing_key, time_for_rabbit)
//                 .await;
//     });
//     list.retain(|sender| sender.send(Message::Text(time.clone())).is_ok());

//     // let senders_arc = ws_state.senders.clone();
//     // let time_clone = time.clone();

//     // tokio::spawn(async move {
//     //     let mut list = senders_arc.lock().unwrap();

//     //     println!("{}", time_clone);

//     //     list.retain(|sender| sender.send(Message::Text(time_clone.clone())).is_ok());
//     // });

//     Ok(())
// }

#[tauri::command]
fn update_time(time: String, ws_state: tauri::State<'_, WsState>) -> Result<(), String> {
    // Ambil Arc<Mutex<Vec<Senders>>> dari state → ini boleh di-clone
    let senders_arc = ws_state.senders.clone();
    let time_clone = time.clone(); // klo mau print dll

    tokio::spawn(async move {
        let mut list = senders_arc.lock().unwrap();

        println!("{}", time_clone);

        list.retain(|sender| sender.send(Message::Text(time_clone.clone())).is_ok());
    });

    Ok(())
}

#[tauri::command]
fn update_quarter(
    quarter: String,
    state: tauri::State<'_, Arc<Mutex<AppState>>>,
) -> Result<(), String> {
    let state = state.lock().unwrap();
    let host = state.rabbitmq_host.clone();
    let username = state.rabbitmq_username.clone();
    let password = state.rabbitmq_password.clone();
    let routing_key = format!(
        "sportkit.basket.{}.{}.quarter",
        state.event_id, state.field_id
    );

    tokio::spawn(async move {
        let _ = rabbitmq::produce_to_rabbitmq(host, username, password, routing_key, quarter).await;
    });

    Ok(())
}

#[tauri::command]
fn save_config(
    rabbitmq_host: String,
    rabbitmq_username: String,
    rabbitmq_password: String,
    event_id: String,
    field_id: String,
    scorer_url: String,
    dark_statistic_url: String,
    light_statistic_url: String,
    man_of_the_match_url: String,
    top_player_url: String,
    app_state: tauri::State<'_, Arc<Mutex<AppState>>>,
    config_state: tauri::State<'_, Arc<Mutex<ConfigState>>>,
) -> Result<(), String> {
    let mut app_state = app_state.lock().unwrap();
    app_state.rabbitmq_host = rabbitmq_host;
    app_state.rabbitmq_username = rabbitmq_username;
    app_state.rabbitmq_password = rabbitmq_password;
    app_state.event_id = event_id;
    app_state.field_id = field_id;
    app_state.scorer_url = scorer_url;
    app_state.dark_statistic_url = dark_statistic_url;
    app_state.light_statistic_url = light_statistic_url;
    app_state.man_of_the_match_url = man_of_the_match_url;
    app_state.top_player_url = top_player_url;
    let relative_path = config_state.lock().unwrap().file_path.to_string();
    let file_path = utils::get_absolute_path_in_home(&relative_path).unwrap();

    let _ = utils::ensure_file_exists(&file_path);

    match serde_json::to_string(&*app_state) {
        Ok(serialized) => match std::fs::write(file_path, &serialized) {
            Ok(_) => {
                return Ok(());
            }
            Err(e) => {
                let err_msg = format!("Failed to write config file: {}", e);
                println!("{}", &err_msg);
                return Err(err_msg);
            }
        },
        Err(e) => {
            let err_msg = format!("Failed to serialize state: {}", e);
            println!("{}", &err_msg);
            return Err(err_msg);
        }
    };
    // Optionally, log or do something after updating the state
}

#[tauri::command]
fn open_config(app: AppHandle) -> Result<(), String> {
    let config_window = app.get_window("configurationpage").unwrap();
    config_window.show().unwrap();
    Ok(())
}

#[tauri::command]
fn end_config(app: AppHandle) -> Result<(), String> {
    let config_window = app.get_window("configurationpage").unwrap();
    let main_window = app.get_window("indexpage").unwrap();
    let controller_window = app.get_window("controllerpage").unwrap();
    config_window.hide().unwrap();
    main_window.show().unwrap();
    controller_window.show().unwrap();
    Ok(())
}

#[tauri::command]
fn get_config(state: tauri::State<'_, Arc<Mutex<AppState>>>) -> Result<AppState, String> {
    let state = state.lock().map_err(|e| e.to_string())?;
    Ok(state.clone())
}

#[tauri::command]
fn close_all_processes() {
    std::process::exit(0);
}

#[tauri::command]
fn get_local_ip() -> String {
    local_ip_address::local_ip().unwrap().to_string()
}

async fn start_ws_server(state: WsState) {
    let listener = TcpListener::bind("127.0.0.1:9000")
        .await
        .expect("WebSocket server failed to start");

    println!("WS Server listening on ws://127.0.0.1:9000");

    loop {
        let (stream, _) = listener.accept().await.unwrap();

        let state_clone = state.clone();

        tokio::spawn(async move {
            if let Err(e) = handle_ws_connection(stream, state_clone).await {
                println!("WS connection error: {}", e);
            }
        });
    }
}

async fn handle_ws_connection(
    stream: tokio::net::TcpStream,
    state: WsState,
) -> Result<(), Box<dyn std::error::Error>> {
    let ws_stream = accept_async(stream).await?;
    let (mut write, mut read) = ws_stream.split();

    // channel untuk kirim ke client
    let (tx, mut rx) = tokio::sync::mpsc::unbounded_channel::<Message>();

    {
        let mut list = state.senders.lock().unwrap();
        list.push(tx);
    }

    // Task untuk menulis ke client
    tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            let _ = write.send(msg).await;
        }
    });

    while let Some(Ok(msg)) = read.next().await {
        println!("Client says: {:?}", msg);
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    let config_state = ConfigState {
        file_path: "basket_config.json".to_string(),
    };

    let file_path = utils::get_absolute_path_in_home(&config_state.file_path).unwrap();

    let app_state = match utils::read_file(&file_path) {
        Ok(contents) => match serde_json::from_str::<AppState>(&contents) {
            Ok(data) => {
                println!("Deserialized data: {:?}", data);
                data
            }
            Err(e) => {
                println!("Error parsing JSON: {}", e);
                AppState {
                    rabbitmq_host: "initial_host".to_string(),
                    rabbitmq_username: "initial_username".to_string(),
                    rabbitmq_password: "initial_password".to_string(),
                    event_id: "initial_event_id".to_string(),
                    field_id: "initial_field_id".to_string(),
                    scorer_url: "initial_scorer_url".to_string(),
                    dark_statistic_url: "initial_dark_statistic_url".to_string(),
                    light_statistic_url: "initial_light_statistic_url".to_string(),
                    man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                    top_player_url: "initial_top_player_url".to_string(),
                }
            }
        },
        Err(e) => {
            println!("Error reading file: {}", e);
            AppState {
                rabbitmq_host: "initial_host".to_string(),
                rabbitmq_username: "initial_username".to_string(),
                rabbitmq_password: "initial_password".to_string(),
                event_id: "initial_event_id".to_string(),
                field_id: "initial_field_id".to_string(),
                scorer_url: "initial_scorer_url".to_string(),
                dark_statistic_url: "initial_dark_statistic_url".to_string(),
                light_statistic_url: "initial_light_statistic_url".to_string(),
                man_of_the_match_url: "initial_man_of_the_match_url".to_string(),
                top_player_url: "initial_top_player_url".to_string(),
            }
        }
    };

    tauri::Builder::default()
        .manage({
            let state = Arc::new(Mutex::new(app_state));
            state
        })
        .manage({
            let state = Arc::new(Mutex::new(config_state));
            state
        })
        .manage(Arc::new(Mutex::new(
            HashMap::<String, Box<dyn SerialPort>>::new(),
        )))
        .invoke_handler(tauri::generate_handler![
            update_time,
            update_quarter,
            list_serial_ports,
            connect_serial_port,
            disconnect_serial_port,
            trigger_alarm,
            save_config,
            get_config,
            open_config,
            end_config,
            close_all_processes,
            toggle_fullscreen,
            get_local_ip,
            init_license,
            read_license,
            update_license,
            get_date_from_hash,
            get_original_hash,
            print_with_rust,
            get_disk_id_windows,
        ])
        .setup(|app| {
            let splashscreen_window = app.get_window("splashscreen").unwrap();
            // let controller_window = app.get_window("controllerpage").unwrap();
            // let main_windows = app.get_window("indexpage").unwrap();
            let _configuration_windows = app.get_window("configurationpage").unwrap();

            tauri::async_runtime::spawn(async move {
                sleep(Duration::from_secs(1)).await;

                splashscreen_window.show().unwrap();
                // configuration_windows.show().unwrap();
                // main_windows.show().unwrap();
                // controller_window.show().unwrap();
            });
            let ws_state = WsState {
                senders: Arc::new(Mutex::new(Vec::new())),
            };

            tauri::async_runtime::spawn(start_ws_server(ws_state.clone()));
            app.manage(ws_state);

            let app_handle = app.handle();

            app.listen_global("stop-ad", move |_| {
                let window = app_handle.get_window("indexpage").unwrap();
                window.emit("stop-ad", {}).unwrap();
            });

            tauri::async_runtime::spawn(async move { ws_server::run_ws_server().await });
            tauri::async_runtime::spawn(async move { web_server::run_http_server().await });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
