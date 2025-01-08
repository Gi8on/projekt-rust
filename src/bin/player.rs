use ggez::event;
use projekt::{
    arguments::parse_player,
    common::messages::{get_message, send_message, Message, PlayerId, ReadType, Side},
    configuration::Configuration,
    messages::wait_for_message,
    paddle_like::RectangularPaddle,
    player_state::PlayerState,
};
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

fn connect_to_server(socket: &UdpSocket, server: &SocketAddr) -> (Side, PlayerId) {
    println!("Trying to connect to server");
    let join = Message::Join;
    loop {
        send_message(socket, &join, server);
        let (msg, who) = match get_message(socket) {
            ReadType::MessageRead(msg, who) => (msg, who),
            _ => continue,
        };

        if who != *server {
            println!("Unexpected message not from server from {:?}", who);
            continue;
        }

        match msg {
            Message::Ok(side, player_id) => {
                println!("Connected to server as {:?} player", side);
                return (side, player_id);
            }
            Message::Taken => {
                panic!("Game is full");
            }
            _ => {
                println!("wtf! Unexpected message: {:?}", msg);
            }
        }
    }
}

fn main() -> ggez::GameResult {
    let (ip, port, server_ip, server_port) = parse_player();

    let server_address_string: String = format!("{}:{}", server_ip, server_port);
    let server_address =
        SocketAddr::from_str(&server_address_string).expect("Couldn't parse server address");
    println!("Connecting to server at {}", server_address);

    // Bind the socket to an address and port
    let socket = UdpSocket::bind(format!("{}:{}", ip, port)).expect("couldn't bind to address");
    println!("Binded on {}:{}", ip, port);

    socket
        .set_read_timeout(None)
        .expect("set_read_timeout call failed");
    let (side, player_id) = connect_to_server(&socket, &server_address);

    println!("Waiting for game to start");
    wait_for_message(&socket, &server_address, Message::Ready);

    socket
        .set_nonblocking(true)
        .expect("set_nonblocking call failed");

    let config = Configuration::default();
    let screen_width = config.screen_width;
    let screen_height = config.screen_height;
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("multiplayer_pong", "marcin g")
        .window_setup(ggez::conf::WindowSetup::default().title("Multiplayer Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_width, screen_height))
        .build()?;
    let state = PlayerState::<RectangularPaddle, RectangularPaddle>::new(
        config,
        &mut ctx,
        side,
        socket,
        server_address,
        player_id,
    );
    // let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    event::run(ctx, event_loop, state);
}
