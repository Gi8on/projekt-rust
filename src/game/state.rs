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

pub struct State {
    paddle_left: Paddle,
    paddle_right: Paddle,
    input: Input,
    ball: Ball,
}

impl State {
    pub fn new(config: Configuration, ctx: &mut Context) -> Self {
        Self {
            paddle_left: Paddle::new(
                0.0,
                config.screen_height / 2.0,
                config.paddle_width,
                config.left_paddle_color,
                config.paddle_height,
                (0.0, config.screen_height),
                config.paddle_speed,
            ),
            paddle_right: Paddle::new(
                config.screen_width,
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
        }
    }

    // fn bouncing(&mut self) {
    //     let ball = &mut self.ball;
    //     self.paddle_left.bouncing(ball);
    //     self.paddle_right.bouncing(ball);
    // }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let dt :f32 = ctx.time.delta();
        let dt = 1.0 / DESIRED_FPS as f32;
        // let mut num_of_updates = 0;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.ball.update_different(dt);
            self.paddle_left
                .update(dt, self.input.left_up, self.input.left_down);
            self.paddle_right
                .update(dt, self.input.right_up, self.input.right_down);
            // self.bouncing();
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
        self.paddle_left.draw(ctx, &mut canvas);
        self.paddle_right.draw(ctx, &mut canvas);

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
