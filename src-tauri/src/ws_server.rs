// use futures::{SinkExt, StreamExt};
// use serde::{Deserialize, Serialize};
// use std::sync::{Arc, Mutex};
// use warp::ws::{Message, WebSocket};
// use warp::Filter;

// #[derive(Serialize, Deserialize, Clone)]
// pub struct ScoreData {
//     pub team_a_name: String,
//     pub team_a_score: i32,
//     pub team_a_foul: i32,
//     pub team_a_timeout: i32,

//     pub team_b_name: String,
//     pub team_b_score: i32,
//     pub team_b_foul: i32,
//     pub team_b_timeout: i32,

//     pub timer: i32,
//     pub timeout: i32,
//     pub quarter: i32,
// }

// pub async fn run_ws_server() {
//     let state = Arc::new(Mutex::new(ScoreData {
//         team_a_name: "".into(),
//         team_a_score: 0,
//         team_a_foul: 0,
//         team_a_timeout: 0,

//         team_b_name: "".into(),
//         team_b_score: 0,
//         team_b_foul: 0,
//         team_b_timeout: 0,

//         timer: 0,
//         timeout: 0,
//         quarter: 0,
//     }));

//     let state_filter = warp::any().map(move || Arc::clone(&state));

//     let route =
//         warp::path("ws")
//             .and(warp::ws())
//             .and(state_filter)
//             .map(|ws: warp::ws::Ws, state| {
//                 ws.on_upgrade(move |socket| handle_connection(socket, state))
//             });

//     println!("WebSocket server berjalan di ws://0.0.0.0:8071/ws");

//     warp::serve(route).run(([0, 0, 0, 0], 8071)).await;
// }

// async fn handle_connection(ws: WebSocket, state: Arc<Mutex<ScoreData>>) {
//     let (mut tx, mut rx) = ws.split();

//     let initial_json = serde_json::to_string(&*state.lock().unwrap()).unwrap();
//     let _ = tx.send(Message::text(initial_json)).await;

//     while let Some(Ok(msg)) = rx.next().await {
//         if msg.is_text() {
//             if let Ok(text) = msg.to_str() {
//                 if let Ok(new_data) = serde_json::from_str::<ScoreData>(text) {
//                     *state.lock().unwrap() = new_data.clone();

//                     let json = serde_json::to_string(&new_data).unwrap();
//                     let _ = tx.send(Message::text(json)).await;
//                 }
//             }
//         }
//     }
// }
// /====================================================================
use futures::{FutureExt, StreamExt};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::sync::mpsc;
use tokio_stream::wrappers::UnboundedReceiverStream;
use warp::{
    ws::{Message, WebSocket},
    Filter,
};

type ClientId = usize;
type Clients =
    Arc<Mutex<HashMap<ClientId, mpsc::UnboundedSender<std::result::Result<Message, warp::Error>>>>>;

#[derive(Serialize, Deserialize, Clone)]
pub struct ScoreData {
    pub team_a_name: String,
    pub team_a_score: i32,
    pub team_a_foul: i32,
    pub team_a_timeout: i32,

    pub team_b_name: String,
    pub team_b_score: i32,
    pub team_b_foul: i32,
    pub team_b_timeout: i32,

    pub timer: String,
    pub timeout: String,
    pub quarter: String,
}

pub async fn run_ws_server() {
    let state = Arc::new(Mutex::new(ScoreData {
        team_a_name: "".into(),
        team_a_score: 0,
        team_a_foul: 0,
        team_a_timeout: 0,

        team_b_name: "".into(),
        team_b_score: 0,
        team_b_foul: 0,
        team_b_timeout: 0,

        timer: "".into(),
        timeout: "".into(),
        quarter: "".into(),
    }));

    let clients: Clients = Arc::new(Mutex::new(HashMap::new()));

    let state_filter = warp::any().map(move || Arc::clone(&state));
    let clients_filter = warp::any().map(move || Arc::clone(&clients));

    let next_client_id = Arc::new(Mutex::new(0));
    let id_filter = warp::any().map(move || {
        let mut id_counter = next_client_id.lock().unwrap();
        *id_counter += 1;
        *id_counter
    });

    let route = warp::path("ws")
        .and(warp::ws())
        .and(state_filter)
        .and(clients_filter)
        .and(id_filter)
        .map(|ws: warp::ws::Ws, state, clients, client_id| {
            ws.on_upgrade(move |socket| handle_connection(socket, state, clients, client_id))
        });

    println!("WebSocket server berjalan di ws://0.0.0.0:8071/ws");

    warp::serve(route).run(([0, 0, 0, 0], 8071)).await;
}

fn broadcast_update(clients: &Clients, json_message: &str) {
    let clients_guard = clients.lock().unwrap();
    for tx in clients_guard.values() {
        let _ = tx.send(Ok(Message::text(json_message)));
    }
}

async fn handle_connection(
    ws: WebSocket,
    state: Arc<Mutex<ScoreData>>,
    clients: Clients,
    client_id: ClientId,
) {
    println!("Client connected: {}", client_id);
    let (ws_tx, mut ws_rx) = ws.split();

    let (client_tx, client_rx) = mpsc::unbounded_channel();

    let client_rx_stream = UnboundedReceiverStream::new(client_rx);

    // tokio::task::spawn(client_rx.forward(ws_tx).map(|result| {
    //     if let Err(e) = result {
    //         eprintln!("websocket send error: {}", e);
    //     }
    // }));

    tokio::task::spawn(client_rx_stream.forward(ws_tx).map(|result| {
        if let Err(e) = result {
            eprintln!("websocket send error: {}", e);
        }
    }));

    clients.lock().unwrap().insert(client_id, client_tx);

    let initial_json = serde_json::to_string(&*state.lock().unwrap()).unwrap();
    let _ = clients
        .lock()
        .unwrap()
        .get(&client_id)
        .unwrap()
        .send(Ok(Message::text(initial_json)));

    while let Some(result) = ws_rx.next().await {
        let msg = match result {
            Ok(msg) => msg,
            Err(e) => {
                eprintln!("websocket error(client {}): {}", client_id, e);
                break;
            }
        };

        if msg.is_text() {
            if let Ok(text) = msg.to_str() {
                if let Ok(new_data) = serde_json::from_str::<ScoreData>(text) {
                    *state.lock().unwrap() = new_data.clone();

                    let json = serde_json::to_string(&new_data).unwrap();

                    broadcast_update(&clients, &json);
                }
            }
        }
    }

    clients.lock().unwrap().remove(&client_id);
    println!("Client disconnected: {}", client_id);
}
