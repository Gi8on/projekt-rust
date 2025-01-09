use ggez::{glam, graphics};

pub const SCREEN_WIDTH: f32 = 800.0;
pub const SCREEN_HEIGHT: f32 = 600.0;
pub const _SCREEN_DIMS: (f32, f32) = (SCREEN_WIDTH, SCREEN_HEIGHT);

const PADDLE_WIDTH: f32 = 10.0;
const PADDLE_HEIGHT: f32 = 0.3 * SCREEN_HEIGHT;
const PADDLE_SPEED: f32 = 200.0;

const PADDLE_TO_BALL_SPEEDUP: f32 = 1.05;

const LEFT_PADDLE_COLOR: graphics::Color = graphics::Color::RED;
const RIGHT_PADDLE_COLOR: graphics::Color = graphics::Color::BLUE;

const INITIAL_BALL_VELOCITY: glam::Vec2 = glam::vec2(200.0, 200.0);
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
    pub paddle_to_ball_speedup: f32,
    pub left_paddle_color: ggez::graphics::Color,
    pub right_paddle_color: ggez::graphics::Color,
}

impl Default for Configuration {
    fn default() -> Self {
        Self {
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
            paddle_to_ball_speedup: PADDLE_TO_BALL_SPEEDUP,
        }
    }
}

pub trait FromConfiguration {
    fn from_configuration(config: &Configuration) -> Self;
}
