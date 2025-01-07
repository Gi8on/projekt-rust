use projekt::{
    common::messages::{get_message, Direction, Message, PlayerMove, ReadType, Side, Tick},
    configuration::{Configuration, FromConfiguration},
    messages::{send_message, send_safely},
    multiplayer_pong::MultiplayerPong,
    arguments::parse_server,
};
use std::{net::UdpSocket, thread, time::Instant};

const TICKSPERSECOND: u64 = 30;

#[derive(Default)]
pub struct PlayerInput {
    tick: Tick,
    up: bool,
    down: bool,
}

impl PlayerInput {
    fn update(&mut self, player_move: PlayerMove) {
        if player_move.tick > self.tick {
            self.tick = player_move.tick;
            match player_move.dir {
                Direction::Up(b) => self.up = b,
                Direction::Down(b) => self.down = b,
            }
        }
    }
}

fn accept_player(socket: &UdpSocket, side: Side) -> std::net::SocketAddr {
    loop {
        let (msg, who) = match get_message(socket) {
            ReadType::MessageRead(msg, who) => (msg, who),
            _ => continue,
        };

        match msg {
            Message::Join => {
                send_message(socket, &Message::Ok(side), &who);
                return who;
            }
            _ => send_message(socket, &Message::Taken, &who),
        }
        panic!("sdhjkfhds");
    }
}

fn main() {
    let (ip, port) = parse_server();

    // Bind the socket to an address and port
    let socket = UdpSocket::bind(format!("{}:{}", ip, port)).expect("couldn't bind to address");
    println!("Listening on {}:{}", ip, port);

    socket
        .set_read_timeout(None)
        .expect("set_read_timeout call failed");
    let player_left_addr = accept_player(&socket, Side::Left);
    println!("Connected left player: {:?}", player_left_addr);
    let player_right_addr = accept_player(&socket, Side::Right);
    println!("Connected right player: {:?}", player_right_addr);

    send_safely(&socket, &Message::Ready, &player_left_addr);
    send_safely(&socket, &Message::Ready, &player_right_addr);

    socket
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let mut multiplayer_pong = MultiplayerPong::from_configuration(&Configuration::default());

    let interval = std::time::Duration::from_millis(1000 / TICKSPERSECOND);
    let mut left_last_move = PlayerInput::default();
    let mut right_last_move = PlayerInput::default();
    let mut _score = (0, 0);

    let dt = 1.0 / TICKSPERSECOND as f32;
    let mut tick = 0;

    loop {
        let start = Instant::now();

        loop {
            let (msg, who) = match get_message(&socket) {
                ReadType::MessageRead(msg, who) => (msg, who),
                ReadType::AllRead => break,
                _ => continue,
            };

            match msg {
                Message::Move(player_move) => {
                    if who == player_left_addr {
                        println!("Left player moved: {:?}", player_move);
                        left_last_move.update(player_move);
                    } else if who == player_right_addr {
                        println!("Right player moved: {:?}", player_move);
                        right_last_move.update(player_move);
                    } else {
                        println!("Unknown player {}", who);
                    }
                }
                _ => {
                    println!("Unexpected message: {:?}", msg);
                }
            }
        }

        // Update game state
        let (_round_result, game_state) = multiplayer_pong.multi_game_round(
            (left_last_move.up, left_last_move.down),
            (right_last_move.up, right_last_move.down),
            dt,
            tick,
        );

        // Send game state to players
        send_message(&socket, &Message::State(game_state), &player_left_addr);
        send_message(&socket, &Message::State(game_state), &player_right_addr);

        tick += 1;

        // Sleep for the remaining time to maintain the interval
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
    }
}
