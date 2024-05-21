use std::collections::HashMap;
use std::io::Error;
use models::{ClientMessage, Question};
use tokio::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use tokio::time::Instant;
use tokio_tungstenite::accept_async;
use tracing::{event, Level};
use uuid::Uuid;
use crate::models::Player;
use tracing_subscriber;

mod models;

type PlayerData = Arc<Mutex<HashMap<String, Player>>>;

let questions = vec![
    Question {

    }
]

async fn handle_connection(raw_stream: TcpStream, player_data: PlayerData){

    let ws_stream = accept_async(raw_stream).await.expect("Failed to accept");
    
    let player_id = Uuid::new_v4().to_string();

    tracing::info!("Player connected {}", player_id.to_string());

    let mut player = Player {
        id: player_id.clone(),
        name: "test".to_string(),
        score: 0,
        last_time_response: Instant::now(),
        ws: ws_stream
    };

    // storage data
    {
        let mut data = player_data.lock().unwrap();
        data.insert(player_id.clone(), player);
    }
}

async fn start_game_rounds(player_data: PlayerData) {

    let rounds = 2;

    for round_number in 1..=rounds {
        
        for player in player_data.lock(.unwrap().values_mut() {
            let message = ClientMessage::Question(questions[1]);
            player.ws.send(WSMessage::Text(serde_json::to_string(&message).unwrap()))
        }

    }
}


#[tokio::main]
async fn main()  -> Result<(), Error>{
    
    tracing_subscriber::fmt().init();

    let listener = match TcpListener::bind("127.0.0.1:8081").await {
        Ok(e) => e,
        Err(_) => {
            panic!("Error at listening creation")
        }
    };
    let player_data = Arc::new(Mutex::new(HashMap::new()));

    let mut number_of_players = 0;

    println!("Server running on port 8080");

    while let Ok((stream, _)) = listener.accept().await {
        if number_of_players <= 10 {
            let player_data_clone = player_data.clone();
            tokio::spawn(handle_connection(stream, player_data_clone));
            number_of_players += 1;
        }
    }
    Ok(())
}
