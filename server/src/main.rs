use std::{env, io::Error};
use futures_util::{future, StreamExt, TryStreamExt};
use log::{info, error};
use std::collections::HashMap;
use tokio::net::{TcpListener, TcpStream};
type Tx = UnboundedSender<Message>;
use tokio_tungstenite::tungstenite::protocol::Message;
use std::net::SocketAddr;
use std::sync::{Arc, RwLock};
use futures_util::pin_mut;
use futures_channel::mpsc::{unbounded, UnboundedSender};
type SharedState = Arc<RwLock<State>>;
use serde::{Serialize, Deserialize};
type Peers = HashMap<SocketAddr, Tx>;
use taskini_common::{
    Tasks,
    Task
};
#[derive(Debug)]
struct State {
    peers: Peers,
    tasks: Tasks,
    version: usize,
}
#[tokio::main]
async fn main() -> Result<(), Error> {
    let tasks: Vec<String> = Vec::new();
    let state = State {
        peers: HashMap::new(),
        tasks: Tasks::new(),
        version: 0,
    };
    let shared_state = SharedState::new(RwLock::new(state));
    let _ = env_logger::try_init();
    let addr = env::args().nth(1).unwrap_or_else(|| "127.0.0.1:8080".to_string());
    let try_socket = TcpListener::bind(&addr).await;
    let listener = try_socket.expect("Failed to bind");
    info!("Listening on: {}", addr);
    while let Ok((stream, _)) = listener.accept().await {
        tokio::spawn(accept_connection(stream, shared_state.clone()));
    }
    Ok(())
}
fn add_peer(shared_state: &SharedState, addr: &SocketAddr, tx: UnboundedSender<Message>) {
    shared_state.write().unwrap().peers.insert(*addr, tx);
}
fn handle_message (shared_state: &SharedState, addr: &SocketAddr, msg: Message) {
    let msg_text = msg.to_text().unwrap();
    info!("{}: {}", addr, msg_text);
    let mut state = shared_state.write().unwrap();
    state.tasks.push(Task {
        created_by: *addr,
        title: msg_text.to_string(),
    });
    state.version += 1;
    info!("state: {:?}", state);
}
fn notify (shared_state: &SharedState) {
    let state = shared_state.read().unwrap();
    for peer in state.peers.iter().map(|(_, tx)| tx) {
        send_state(& peer, &state);
    }
}
fn send_state(tx: & Tx, state: &State) {
    let json_str = serde_json::to_string(&state.tasks).unwrap();
    match tx.unbounded_send(json_str.into()) {
        Err(e) => {
            error!("{}",e);
        },
        Ok(_) => {}
    }
}
async fn accept_connection(stream: TcpStream, shared_state: SharedState) {
    let (tx, rx) = unbounded();
    let addr = stream.peer_addr().expect("connected streams should have a peer address");
    info!("New peer: {}", addr);
    let ws_stream = tokio_tungstenite::accept_async(stream)
        .await
        .expect("Error during the websocket handshake occurred");
    info!("New WebSocket connection: {}", addr);
    let (outgoing, incoming) = ws_stream.split();
    add_peer(&shared_state, &addr, tx.clone());
    send_state(&tx, &shared_state.read().unwrap());
    let handle_incoming = incoming.try_for_each(|msg| {
        handle_message(&shared_state, &addr, msg);
        notify(&shared_state);
        future::ok(())
    });
    let receive_from_others = rx.map(Ok).forward(outgoing);
    pin_mut!(handle_incoming, receive_from_others);
    future::select(handle_incoming, receive_from_others).await;
    println!("{} disconnected", &addr);
    shared_state.write().unwrap().peers.remove(&addr);
}
