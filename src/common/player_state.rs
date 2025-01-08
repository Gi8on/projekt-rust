use std::net::{SocketAddr, UdpSocket};

use super::messages::{
    get_message, send_message, send_safely, Direction, GameState, PlayerMove, ReadType, Side,
};
use crate::configuration::{Configuration, FromConfiguration};
use crate::game::{ball::Ball, paddle::Paddle, paddle_like::PaddleLike};
use crate::messages::{Message, PlayerId};
use crate::paddle::paddle_from_configuration;
use ggez::{graphics, Context, GameError, GameResult};

const DESIRED_FPS: u32 = 20;

const SCREEN_COLOR: graphics::Color = graphics::Color::BLACK;

struct Game {
    player_id: PlayerId,
    game_state: GameState,
    left_score: u32,
    right_score: u32,
    _timer: f32,
}

pub struct PlayerState<L: PaddleLike, R: PaddleLike> {
    paddle_left: Paddle<L>,
    paddle_right: Paddle<R>,
    ball: Ball,
    game: Game,
    _side: Side,
    tick: u32,
    socket: UdpSocket,
    dest_addr: SocketAddr,
}

impl<L: PaddleLike, R: PaddleLike> PlayerState<L, R> {
    pub fn get_player_id(&self) -> PlayerId {
        self.game.player_id
    }
}

impl<L: PaddleLike + FromConfiguration, R: PaddleLike + FromConfiguration> PlayerState<L, R> {
    pub fn new(
        config: Configuration,
        ctx: &mut Context,
        side: Side,
        socket: UdpSocket,
        dest_addr: SocketAddr,
        player_id: PlayerId,
    ) -> Self {
        let (paddle_left, paddle_right) = paddle_from_configuration(&config);
        Self {
            paddle_left,
            paddle_right,
            ball: Ball::from_configuration(&config, ctx),
            game: Game {
                player_id,
                game_state: GameState::from_configuration(&config),
                left_score: 0,
                right_score: 0,
                _timer: 0.0,
            },
            _side: side,
            tick: 0,
            socket,
            dest_addr,
        }
    }
}

impl<L: PaddleLike, R: PaddleLike> ggez::event::EventHandler<GameError> for PlayerState<L, R> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let dt :f32 = ctx.time.delta();
        let _dt = 1.0 / DESIRED_FPS as f32;
        // let mut num_of_updates = 0;
        while ctx.time.check_update_time(DESIRED_FPS) {
            loop {
                match get_message(&self.socket) {
                    ReadType::AllRead => break,
                    ReadType::WrongRead => continue,
                    ReadType::MessageRead(msg, addr) => {
                        if addr == self.dest_addr {
                            match msg {
                                Message::State(gs) => {
                                    self.game.game_state.update(gs);
                                }
                                Message::Score(left, right) => {
                                    self.game.left_score = left;
                                    self.game.right_score = right;
                                    println!("Scoree! left: {}, right: {}", left, right);
                                }
                                Message::EndingGame(_) => {
                                    println!("Ending game");
                                    std::process::exit(0);
                                }
                                _ => {
                                    eprintln!("Unexpected message: {:?}", msg);
                                }
                            }
                        }
                    }
                }
            }

            self.paddle_left
                .set_position(self.game.game_state.left_paddle);
            self.paddle_right
                .set_position(self.game.game_state.right_paddle);
            self.ball.set_position(self.game.game_state.ball);

            // num_of_updates += 1;
            // if num_of_updates > 1 {
            //    println!("num of updates: {}", num_of_updates);
            //}
        }
        Ok(())
    }

    fn quit_event(&mut self, _ctx: &mut Context) -> Result<bool, GameError> {
        send_safely(
            &self.socket,
            &Message::EndingGame(self.get_player_id()),
            &self.dest_addr,
        );
        Ok(true)
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, SCREEN_COLOR);

        self.ball.draw(&mut canvas);
        self.paddle_left.draw(&mut canvas);
        self.paddle_right.draw(&mut canvas);

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: ggez::input::keyboard::KeyInput,
        _repeat: bool,
    ) -> GameResult {
        println!("key pressed: {:?}", keyinput.keycode);
        if let Some(dir) = match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::Up) => Some(Direction::Up(true)),
            Some(ggez::input::keyboard::KeyCode::Down) => Some(Direction::Down(true)),
            _ => None,
        } {
            let move_msg = Message::Move(PlayerMove {
                player_id: self.get_player_id(),
                tick: self.tick,
                dir,
            });
            self.tick += 1;
            send_message(&self.socket, &move_msg, &self.dest_addr);
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        println!("key released: {:?}", keyinput.keycode);
        if let Some(dir) = match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::Up) => Some(Direction::Up(false)),
            Some(ggez::input::keyboard::KeyCode::Down) => Some(Direction::Down(false)),
            _ => None,
        } {
            let move_msg = Message::Move(PlayerMove {
                player_id: self.get_player_id(),
                tick: self.tick,
                dir,
            });
            // sending two times, because it is crucial that server receives the message
            // 'key down event' is fired multiple times so it is not nedded there
            self.tick += 1;
            send_message(&self.socket, &move_msg, &self.dest_addr);
            self.tick += 1;
            send_message(&self.socket, &move_msg, &self.dest_addr);
        }
        Ok(())
    }
}
