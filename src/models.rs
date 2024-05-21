use serde::{Deserialize, Serialize};
use tokio::time::Instant;
use uuid::Uuid;


#[derive(Serialize, Deserialize)]
pub struct Question {
    pub id: String,
    pub content: String,
    pub options : Vec<String>
}

#[derive(Serialize, Deserialize)]
pub struct  Answer {
    pub question: String,
    pub chosen_option: usize,
}

pub struct Player {
    pub id: String,
    pub name: String,
    pub score: i32,
    pub last_time_response: Instant,
    pub ws : tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
}

#[derive(Serialize, Deserialize)]
pub enum ClientMessage {
    Question(Question),
    Answer(Answer),
    Score(i32),
    StartRound,
    EndRound,
    DeclareWinner(String)
}





