use serde::{Deserialize, Serialize};
use ggez::glam::Vec2;

pub type Tick = u32;


#[derive(Debug, Serialize, Deserialize)]
struct GameState {
    tick: Tick,
    ball: Vec2,
    left_paddle: Vec2,
    right_paddle: Vec2,
}

impl GameState {
    fn new(tick: Tick, ball: Vec2, left_paddle: Vec2, right_paddle: Vec2) -> Self {
        Self {
            tick,
            ball,
            left_paddle,
            right_paddle,
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct PlayerMove {
    tick: Tick,
    dir: Direction,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Message {
    Ok(Side),
    Taken,
    State(GameState),
    Score(u32, u32),
    Join,
    Move(PlayerMove),
}

#[derive(Debug, Serialize, Deserialize)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Direction {
    pub up: bool,
    pub down: bool,
}

pub enum ReadType {
    AllRead,
    WrongRead,
    MessageRead(Message, std::net::SocketAddr),
}
