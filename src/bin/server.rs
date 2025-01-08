use projekt::{
    arguments::parse_server,
    common::messages::{get_message, Direction, Message, PlayerMove, ReadType, Side, Tick},
    configuration::{Configuration, FromConfiguration},
    messages::{send_message, send_safely, PlayerId},
    multiplayer_pong::MultiplayerPong,
    state::RoundResult,
};
use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::mpsc,
    thread::{self, JoinHandle},
    time::Instant,
};

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
                send_message(socket, &Message::Ok(side, 1), &who);
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
    socket
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let (sender, receiver) = mpsc::channel();
    thread::spawn(|| thread_starter(receiver));
    server(&socket, sender);
}

pub fn send_by_pipe(send: &mpsc::Sender<Message>, msg: Message) {
    if let Err(e) = send.send(msg) {
        println!("Error sending message: {:?}", e);
    }
}

pub fn recv_from_pipe(recv: &mpsc::Receiver<Message>) -> Option<Message> {
    match recv.try_recv() {
        Ok(msg) => Some(msg),
        Err(mpsc::TryRecvError::Empty) => None,
        Err(mpsc::TryRecvError::Disconnected) => {
            panic!("Disconnected from game thread");
        }
    }
}

fn single_game_thread(game: GameStarter) {
    let GameStarter {
        msg_recv,
        left_player_addr: player_left_addr,
        right_player_addr: player_right_addr,
        left_player_id,
        right_player_id,
        socket,
    } = game;

    send_safely(&socket, &Message::Ready, &player_left_addr);
    send_safely(&socket, &Message::Ready, &player_right_addr);

    let mut multiplayer_pong = MultiplayerPong::from_configuration(&Configuration::default());

    let interval = std::time::Duration::from_millis(1000 / TICKSPERSECOND);
    let mut left_last_move = PlayerInput::default();
    let mut right_last_move = PlayerInput::default();
    let mut score = (0, 0);

    let dt = 1.0 / TICKSPERSECOND as f32;
    let mut tick = 0;

    loop {
        let start = Instant::now();

        while let Some(message) = recv_from_pipe(&msg_recv) {
            match message {
                Message::EndingGame(player_id) => {
                    send_safely(&socket, &Message::EndingGame(player_id), &player_left_addr);
                    send_safely(&socket, &Message::EndingGame(player_id), &player_right_addr);
                    return;
                }
                Message::Move(player_move) => {
                    if player_move.player_id == left_player_id {
                        left_last_move.update(player_move);
                    } else if player_move.player_id == right_player_id {
                        right_last_move.update(player_move);
                    }
                }
                _ => {
                    panic!("Imposiible")
                }
            };
        }

        // Update game state
        let (round_result, game_state) = multiplayer_pong.multi_game_round(
            (left_last_move.up, left_last_move.down),
            (right_last_move.up, right_last_move.down),
            dt,
            tick,
        );

        // Send game state to players
        send_message(&socket, &Message::State(game_state), &player_left_addr);
        send_message(&socket, &Message::State(game_state), &player_right_addr);

        // Send score to players
        match round_result {
            RoundResult::LeftScored => {
                score.0 += 1;
                send_message(
                    &socket,
                    &Message::Score(score.0, score.1),
                    &player_left_addr,
                );
                send_message(
                    &socket,
                    &Message::Score(score.0, score.1),
                    &player_right_addr,
                );
            }
            RoundResult::RightScored => {
                score.1 += 1;
                send_message(
                    &socket,
                    &Message::Score(score.0, score.1),
                    &player_left_addr,
                );
                send_message(
                    &socket,
                    &Message::Score(score.0, score.1),
                    &player_right_addr,
                );
            }
            RoundResult::None => {}
        }

        tick += 1;

        // Sleep for the remaining time to maintain the interval
        let elapsed = start.elapsed();
        if elapsed < interval {
            thread::sleep(interval - elapsed);
        }
    }
}

type GameId = u32;

struct GameStarter {
    msg_recv: mpsc::Receiver<Message>,
    left_player_addr: SocketAddr,
    right_player_addr: SocketAddr,
    left_player_id: PlayerId,
    right_player_id: PlayerId,
    socket: UdpSocket,
}

impl GameStarter {
    pub fn game_id(&self) -> GameId {
        get_game_id(self.left_player_id)
    }
}

enum InterThreadMessage {
    StartGame(GameStarter),
    EndGame(GameId),
}

// separate thread for starting thread cos it possibly takes long and we don't want to block other games
fn thread_starter(recv: mpsc::Receiver<InterThreadMessage>) {
    let mut games: HashMap<GameId, JoinHandle<_>> = HashMap::new();
    loop {
        match recv.recv() {
            Ok(msg) => match msg {
                InterThreadMessage::StartGame(game_starter) => {
                    games.insert(
                        game_starter.game_id(),
                        thread::spawn(move || {
                            single_game_thread(game_starter);
                        }),
                    );
                }
                InterThreadMessage::EndGame(game_id) => {
                    let game = games.remove(&game_id).unwrap();
                    if let Err(e) = game.join() {
                        // don't want to panic here
                        println!("Game thread panicked: {:?}", e);
                    }
                }
            },
            Err(_) => {
                println!("Error receiving message in thread starter");
            }
        }
    }
}

//
pub fn get_game_id(player_id: PlayerId) -> GameId {
    player_id / 2
}

pub struct Players {
    players_addr: HashMap<PlayerId, SocketAddr>,
    pub game_thread_communication: HashMap<GameId, mpsc::Sender<Message>>,
}


impl Players {
    pub fn new() -> Self {
        Self {
            players_addr: HashMap::new(),
            game_thread_communication: HashMap::new(),
        }
    }

    pub fn get_player_addr(&self, player_id: PlayerId) -> Option<&SocketAddr> {
        self.players_addr.get(&player_id)
    }

    pub fn num_players(&self) -> usize {
        self.players_addr.len()
    }

    pub fn add_player(&mut self, addr: SocketAddr) -> PlayerId {
        let player_id = self.num_players() as PlayerId;
        self.players_addr.insert(player_id, addr);
        player_id
    }

    pub fn is_ready(&self) -> Option<(PlayerId, PlayerId)> {
        let n = self.num_players() as PlayerId;
        if n % 2 == 0 {
            Some((n - 1, n - 2))
        } else {
            None
        }
    }

    pub fn add_game(&mut self, game_id: GameId, msg_send: mpsc::Sender<Message>) {
        self.game_thread_communication.insert(game_id, msg_send);
    }

    pub fn send_to_game(&self, game_id: GameId, msg: Message) {
        send_by_pipe(
            self.game_thread_communication
                .get(&game_id)
                .expect("Game not found"),
            msg,
        );
    }
}

fn server(socket: &UdpSocket, to_game_starter: mpsc::Sender<InterThreadMessage>) {
    socket
        .set_read_timeout(None)
        .expect("set_read_timeout call failed");
    let mut players: Players = Players::new();
    loop {
        let (msg, who) = match get_message(socket) {
            ReadType::MessageRead(msg, who) => (msg, who),
            _ => continue,
        };
        match msg {
            Message::Join => {
                let player_id = players.add_player(who);
                send_message(socket, &Message::Ok(Side::Left, player_id), &who);

                if let Some((left, right)) = players.is_ready() {
                    let game_id = get_game_id(left);
                    let (msg_send, msg_recv) = mpsc::channel();

                    to_game_starter
                        .send(InterThreadMessage::StartGame(GameStarter {
                            msg_recv,
                            left_player_addr: *players.get_player_addr(left).unwrap(),
                            right_player_addr: *players.get_player_addr(right).unwrap(),
                            left_player_id: left,
                            right_player_id: right,
                            socket: socket.try_clone().unwrap(),
                        }))
                        .expect("Error sending message to game starter");
                    players.add_game(game_id, msg_send);
                }
            }
            Message::EndingGame(player_id) => {
                let game_id = get_game_id(player_id);
                to_game_starter
                    .send(InterThreadMessage::EndGame(game_id))
                    .expect("Error sending message to game starter");
                players.send_to_game(game_id, Message::EndingGame(player_id));
                println!("Player {:?} ended the game", player_id);
            }
            Message::Move(player_move) => {
                players.send_to_game(
                    get_game_id(player_move.player_id),
                    Message::Move(player_move),
                );
                println!("Player {:?} moved: {:?}", who, player_move);
            }
            _ => {
                println!("Unexpected message: {:?}", msg);
            }
        }
    }
}
