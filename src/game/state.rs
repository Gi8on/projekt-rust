use std::f32::consts::SQRT_2;

use super::paddle_like::PaddleLike;
use ggez::{graphics, Context, GameError, GameResult};

const DESIRED_FPS: u32 = 20;

const SCREEN_COLOR: graphics::Color = graphics::Color::BLACK;

use super::ball::{Ball, BallAbstract};
use super::configuration::{Configuration, FromConfiguration};
use super::paddle::paddle_from_configuration;
use super::paddle::Paddle;

#[derive(Debug, Clone, Copy)]
pub struct Input {
    left_up: bool,
    left_down: bool,
    right_up: bool,
    right_down: bool,
}

impl Input {
    pub fn new(left_up: bool, left_down: bool, right_up: bool, right_down: bool) -> Self {
        Self {
            left_up,
            left_down,
            right_up,
            right_down,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Game {
    left_score: u32,
    right_score: u32,
    _timer: f32,
}

pub struct State<L: PaddleLike, R: PaddleLike> {
    paddle_left: Paddle<L>,
    paddle_right: Paddle<R>,
    input: Input,
    ball: Ball,
    game: Game,
}

impl<L: PaddleLike + FromConfiguration, R: PaddleLike + FromConfiguration> State<L, R> {
    pub fn new(config: Configuration, ctx: &mut Context) -> Self {
        let (paddle_left, paddle_right) = paddle_from_configuration(&config);
        Self {
            paddle_left,
            paddle_right,
            ball: Ball::from_configuration(&config, ctx),
            input: Input {
                left_up: false,
                left_down: false,
                right_up: false,
                right_down: false,
            },
            game: Game {
                left_score: 0,
                right_score: 0,
                _timer: 0.0,
            },
        }
    }
}

fn bouncing<L: PaddleLike, R: PaddleLike>(
    ball: &mut BallAbstract,
    paddle_left: &Paddle<L>,
    paddle_right: &Paddle<R>,
) {
    let v1 = paddle_left.bouncing(ball);
    let v2 = paddle_right.bouncing(ball);
    if let Some(v) = v1 {
        ball.set_velocity(v);
    } else if let Some(v) = v2 {
        ball.set_velocity(v);
    }
    ball.clamp_velocity(100.0*SQRT_2, 400.0*SQRT_2);
}

pub enum RoundResult {
    LeftScored,
    RightScored,
    None,
}

pub fn game_frame<L: PaddleLike, R: PaddleLike>(
    ball: &mut BallAbstract,
    paddle_left: &mut Paddle<L>,
    paddle_right: &mut Paddle<R>,
    dt: f32,
    input: &Input,
) -> RoundResult {
    if let Ok(Some(right_scored)) = ball.update_different(dt) {
        ball.reset();
        paddle_left.reset();
        paddle_right.reset();
        if right_scored {
            return RoundResult::RightScored;
        } else {
            return RoundResult::LeftScored;
        }
    }
    paddle_left.update(dt, input.left_up, input.left_down);
    paddle_right.update(dt, input.right_up, input.right_down);
    bouncing(ball, paddle_left, paddle_right);
    RoundResult::None
}

impl<L: PaddleLike, R: PaddleLike> ggez::event::EventHandler<GameError> for State<L, R> {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let dt :f32 = ctx.time.delta();
        let dt = 1.0 / DESIRED_FPS as f32;
        // let mut num_of_updates = 0;
        while ctx.time.check_update_time(DESIRED_FPS) {
            // println!("dt: {}", dt);
            match game_frame(
                &mut self.ball.ball_abstract,
                &mut self.paddle_left,
                &mut self.paddle_right,
                dt,
                &self.input,
            ) {
                RoundResult::LeftScored => {
                    self.game.left_score += 1;
                    println!(
                        "Left scored! left: {}, right: {}",
                        self.game.left_score, self.game.right_score
                    );
                }
                RoundResult::RightScored => {
                    self.game.right_score += 1;
                    println!(
                        "Right scored! left: {}, right: {}",
                        self.game.left_score, self.game.right_score
                    );
                }
                RoundResult::None => (),
            }
            // num_of_updates += 1;
            // if num_of_updates > 1 {
            //    println!("num of updates: {}", num_of_updates);
            //}
        }
        Ok(())
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
        // println!("key pressed: {:?}", keyinput.keycode);
        match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::W) => self.input.left_up = true,
            Some(ggez::input::keyboard::KeyCode::S) => self.input.left_down = true,
            Some(ggez::input::keyboard::KeyCode::Up) => self.input.right_up = true,
            Some(ggez::input::keyboard::KeyCode::Down) => self.input.right_down = true,
            _ => (),
        }
        Ok(())
    }

    fn key_up_event(
        &mut self,
        _ctx: &mut Context,
        keyinput: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        // println!("key released: {:?}", keyinput.keycode);
        match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::W) => self.input.left_up = false,
            Some(ggez::input::keyboard::KeyCode::S) => self.input.left_down = false,
            Some(ggez::input::keyboard::KeyCode::Up) => self.input.right_up = false,
            Some(ggez::input::keyboard::KeyCode::Down) => self.input.right_down = false,
            _ => (),
        }
        Ok(())
    }
}
