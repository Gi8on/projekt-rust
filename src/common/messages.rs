use serde::{Deserialize, Serialize};
use std::net::{SocketAddr, UdpSocket};

use crate::configuration::FromConfiguration;

pub type Tick = u32;
pub type PlayerId = u32;

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct GameState {
    pub tick: Tick,
    pub ball: (f32, f32),
    pub left_paddle: (f32, f32),
    pub right_paddle: (f32, f32),
}

impl GameState {
    pub fn new(
        tick: Tick,
        ball: (f32, f32),
        left_paddle: (f32, f32),
        right_paddle: (f32, f32),
    ) -> Self {
        Self {
            tick,
            ball,
            left_paddle,
            right_paddle,
        }
    }

    pub fn update(&mut self, gs: GameState) {
        if gs.tick > self.tick {
            self.tick = gs.tick;
            self.ball = gs.ball;
            self.left_paddle = gs.left_paddle;
            self.right_paddle = gs.right_paddle;
        }
    }
}

impl Default for GameState {
    fn default() -> Self {
        Self {
            tick: 0,
            ball: (0.0, 0.0),
            left_paddle: (0.0, 0.0),
            right_paddle: (0.0, 0.0),
        }
    }
}

impl FromConfiguration for GameState {
    fn from_configuration(config: &crate::configuration::Configuration) -> Self {
        Self {
            tick: 0,
            ball: (config.screen_width / 2.0, config.screen_height / 2.0),
            left_paddle: (config.paddle_width / 2.0, config.screen_height / 2.0),
            right_paddle: (
                config.screen_width - config.paddle_width / 2.0,
                config.screen_height / 2.0,
            ),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct PlayerMove {
    pub player_id: PlayerId,
    pub tick: Tick,
    pub dir: Direction,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Message {
    // sent by server
    Ok(Side, PlayerId),
    Taken,
    State(GameState),
    Ready,
    Score(u32, u32),
    // sent by both
    EndingGame(PlayerId),
    // sent by client
    Join,
    Move(PlayerMove),
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, Copy)]
pub enum Direction {
    Up(bool),
    Down(bool),
}

pub enum ReadType {
    AllRead,
    WrongRead,
    MessageRead(Message, std::net::SocketAddr),
}

pub fn get_message(socket: &UdpSocket) -> ReadType {
    let mut buf = [0; 1024];
    let (amt, who) = match socket.recv_from(&mut buf) {
        Ok((amt, who)) => (amt, who),
        Err(_) => {
            // Timeout
            return ReadType::AllRead;
        }
    };

    match serde_json::from_slice(buf[..amt].as_ref()) {
        Ok(msg) => ReadType::MessageRead(msg, who),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            eprintln!(
                "Coudn't read message: {:?}",
                String::from_utf8_lossy(&buf[..amt])
            );
            ReadType::WrongRead
        }
    }
}

pub fn send_message(socket: &UdpSocket, msg: &Message, who: &SocketAddr) {
    let response = serde_json::to_string(msg).unwrap();
    socket
        .send_to(response.as_bytes(), who)
        .expect("Couldn't send response");
}

// Wait for a message from a specific address
// Socket should be in blocking mode
pub fn wait_for_message(socket: &UdpSocket, who: &SocketAddr, msg: Message) {
    loop {
        match get_message(socket) {
            ReadType::MessageRead(msg_recv, addr) => {
                if addr == *who && msg == msg_recv {
                    break;
                }
            }
            _ => continue,
        }
    }
}

// Just sends two times so that the message is received
pub fn send_safely(socket: &UdpSocket, msg: &Message, who: &SocketAddr) {
    send_message(socket, msg, who);
    send_message(socket, msg, who);
}
