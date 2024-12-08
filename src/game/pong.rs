use ggez::{event, glam, graphics, GameResult};

use super::state::State;

pub const SCREEN_WIDTH: f32 = 1600.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const _SCREEN_DIMS: (f32, f32) = (SCREEN_WIDTH, SCREEN_HEIGHT);

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 0.3 * SCREEN_HEIGHT;
const PADDLE_SPEED: f32 = 200.0;

const LEFT_PADDLE_COLOR: graphics::Color = graphics::Color::RED;
const RIGHT_PADDLE_COLOR: graphics::Color = graphics::Color::BLUE;

const INITIAL_BALL_VELOCITY: glam::Vec2 = glam::vec2(500.0, 500.0);
const BALL_RADIUS: f32 = 15.0;
const BALL_COLOR: graphics::Color = graphics::Color::WHITE;

pub struct Configuration {
    pub screen_width: f32,
    pub screen_height: f32,
    pub ball_radius: f32,
    pub ball_color: ggez::graphics::Color,
    pub ball_initial_velocity: glam::Vec2,
    pub paddle_width: f32,
    pub paddle_height: f32,
    pub paddle_speed: f32,
    pub left_paddle_color: ggez::graphics::Color,
    pub right_paddle_color: ggez::graphics::Color,
}

pub fn pong() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("pong", "marcin g")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    let state = State::new(
        Configuration {
            screen_width: SCREEN_WIDTH,
            screen_height: SCREEN_HEIGHT,
            ball_radius: BALL_RADIUS,
            ball_color: BALL_COLOR,
            ball_initial_velocity: INITIAL_BALL_VELOCITY,
            paddle_width: PADDLE_WIDTH,
            paddle_height: PADDLE_HEIGHT,
            paddle_speed: PADDLE_SPEED,
            left_paddle_color: LEFT_PADDLE_COLOR,
            right_paddle_color: RIGHT_PADDLE_COLOR,
        },
        &mut ctx,
    );
    // let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    event::run(ctx, event_loop, state);
}
