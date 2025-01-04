use std::net::UdpSocket;
use ggez::glam::Vec2;
use crate::communication::messages::{Message, Side, ReadType, PlayerMove, Direction, GameState};

const PORT: usize = 8080;
const IP: &str = "0.0.0.0";
const TICKSPERSECOND: u32 = 60;

fn get_message(socket: &UdpSocket) -> ReadType {
    let mut buf = [0; 1024];
    let (amt, who) = match socket.recv_from(&mut buf) {
            Ok((amt, who)) => (amt, who),
            Err(e) => {
                // Timeout
                return ReadType::AllRead;
            }
        };

    match serde_json::from_slice(buf[..amt].as_ref()) {
        Ok(msg) => ReadType::MessageRead((msg, who)),
        Err(e) => {
            eprintln!("Error: {:?}", e);
            eprintln!("Coudn't read message: {:?}", String::from_utf8_lossy(&buf[..amt]));
            ReadType::WrongRead
        }
    }
}

fn accept_player(socket: &UdpSocket, side: Side) -> std::net::SocketAddr {

    loop {
        let (msg, who) = match get_message(socket) {
            ReadType::MessageRead((msg, who)) => (msg, who),
            _ => continue,
        };

        match msg {
            Message::Join => {
                let response = Message::Ok(side);
                let response = serde_json::to_string(&response).unwrap();
                socket.send_to(response.as_bytes(), &who)
                    .expect("Couldn't send response");
                return who;
            }
            _ => {
                let response = Message::Taken;
                let response = serde_json::to_string(&response).unwrap();
                socket.send_to(response.as_bytes(), &who).map_err(|_| (eprintln!("Couldn't send response")));
            }
        }
    }
}

fn main() {
    // Bind the socket to an address and port
    let socket = UdpSocket::bind(format!("{}:{}", IP, PORT))?;
    println!("Listening on {}:{}", IP, PORT);
    
    socket.set_read_timeout(None)
        .expect("set_read_timeout call failed");
    let player_left_addr = accept_player(&socket, Side::Left);
    let player_right_addr = accept_player(&socket, Side::Right);
    socket.set_read_timeout(0)
        .expect("set_read_timeout call failed");




    let interval = std::time::Duration::from_millis(1000 / TICKSPERSECOND);
    let mut tick = 0;
    let left_last_move = PlayerMove { 0, dir: Direction { up: false, down: false } };
    let right_last_move = PlayerMove { 0, dir: Direction { up: false, down: false } };

    let mut game_state = GameState::new(tick, Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0));

    loop {
        let start = Instant::now();

        loop {
            let (msg, who) = match get_message(&socket) {
                ReadType::MessageRead((msg, who)) => (msg, who),
                ReadType::AllRead => break,
                _ => continue,
            };

            match msg {
                Message::Move(player_move) => {
                    if who == player_left_addr {
                        println!("Left player moved: {:?}", direction);
                        if player_move.tick > left_last_move.tick {
                            left_last_move = player_move;
                        }
                    } else if who == player_right_addr {
                        println!("Right player moved: {:?}", direction);
                        if player_move.tick > right_last_move.tick {
                            right_last_move = player_move;
                        }
                    } else {
                        // can this even happen?
                        println!("Unexpected player moved: {:?}", direction);
                    }
                }
                _ => {
                    println!("Unexpected message: {:?}", msg);
                }
            }
        }

        // Update game state


        // Send game state to players
        socket.send_to(serde_json::to_vec(Message::State(game_state)), player_left_addr)
            .expect("Couldn't send game state to left player");
        socket.send_to(serde_json::to_vec(Message::State(game_state)), player_right_addr)
            .expect("Couldn't send game state to right player");

        game_state.tick += 1;

        // Sleep for the remaining time to maintain the interval
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
    }
}