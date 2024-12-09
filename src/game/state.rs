use ggez::{graphics, Context, GameError, GameResult};

const DESIRED_FPS: u32 = 20;

const SCREEN_COLOR: graphics::Color = graphics::Color::BLACK;

use super::ball::Ball;
use super::paddle::Paddle;
use super::pong::Configuration;

struct Input {
    left_up: bool,
    left_down: bool,
    right_up: bool,
    right_down: bool,
}

struct Game {
    left_score: u32,
    right_score: u32,
    timer: f32,
}

pub struct State {
    paddle_left: Paddle,
    paddle_right: Paddle,
    input: Input,
    ball: Ball,
    game: Game,
}

impl State {
    pub fn new(config: Configuration, ctx: &mut Context) -> Self {
        Self {
            paddle_left: Paddle::new(
                config.paddle_width / 2.0,
                config.screen_height / 2.0,
                config.paddle_width,
                config.left_paddle_color,
                config.paddle_height,
                (0.0, config.screen_height),
                config.paddle_speed,
            ),
            paddle_right: Paddle::new(
                config.screen_width - config.paddle_width / 2.0,
                config.screen_height / 2.0,
                config.paddle_width,
                config.right_paddle_color,
                config.paddle_height,
                (0.0, config.screen_height),
                config.paddle_speed,
            ),
            ball: Ball::new(
                config.screen_width / 2.0,
                config.screen_height / 2.0,
                config.ball_radius,
                config.ball_initial_velocity,
                config.ball_color,
                (0.0, 0.0, config.screen_width, config.screen_height),
                ctx,
            ),
            input: Input {
                left_up: false,
                left_down: false,
                right_up: false,
                right_down: false,
            },
            game: Game {
                left_score: 0,
                right_score: 0,
                timer: 0.0,
            },
        }
    }

    fn bouncing(&mut self) {
        let ball = &mut self.ball;
        let v1 = self.paddle_left.bouncing(ball);
        let v2 = self.paddle_right.bouncing(ball);
        if let Some(v) = v1 {
            ball.set_velocity(v);
        } else if let Some(v) = v2 {
            ball.set_velocity(v);
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let dt :f32 = ctx.time.delta();
        let dt = 1.0 / DESIRED_FPS as f32;
        // let mut num_of_updates = 0;
        while ctx.time.check_update_time(DESIRED_FPS) {
            if let Ok(Some(right_scored)) = self.ball.update_different(dt) {
                println!("ball out of bounds!!!");
                if (right_scored) {
                    self.game.right_score += 1;
                    println!("right scored!!!")
                } else {
                    self.game.left_score += 1;
                    println!("left scored!!!!");
                }
                println!("game score: left player's points: {}, rigth player's points: {}", self.game.left_score, self.game.right_score);
                self.ball.reset();
            }   
            self.paddle_left
                .update(dt, self.input.left_up, self.input.left_down);
            self.paddle_right
                .update(dt, self.input.right_up, self.input.right_down);
            self.bouncing();
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
