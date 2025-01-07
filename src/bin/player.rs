use ggez::event;
use projekt::{
    common::messages::{get_message, send_message, Message, ReadType, Side}, configuration::Configuration, messages::wait_for_message, paddle_like::RectangularPaddle, player_state::PlayerState
};
use std::{
    net::{SocketAddr, UdpSocket},
    str::FromStr,
};

const PORT: usize = 8070;
const IP: &str = "0.0.0.0";
const SERVER_PORT: usize = 8080;
const SERVER_IP: &str = "127.0.0.1";

fn connect_to_server(socket: &UdpSocket, server: &SocketAddr) -> Side {
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
            Message::Ok(side) => {
                println!("Connected to server as {:?} player", side);
                return side;
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
    let server_address_string: String = format!("{}:{}", SERVER_IP, SERVER_PORT);
    let server_address =
        SocketAddr::from_str(&server_address_string).expect("Couldn't parse server address");
    println!("Connecting to server at {}", server_address);

    // Bind the socket to an address and port
    let socket = UdpSocket::bind(format!("{}:{}", IP, PORT)).expect("couldn't bind to address");
    println!("Binded on {}:{}", IP, PORT);

    socket
        .set_read_timeout(None)
        .expect("set_read_timeout call failed");
    let side = connect_to_server(&socket, &server_address);

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
    );
    // let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    event::run(ctx, event_loop, state);
}
