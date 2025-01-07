use super::paddle_like::PaddleLike;
use super::{ball::BallAbstract, configuration::FromConfiguration};
use ggez::{glam, graphics};

const _HIT_COLOR: graphics::Color = graphics::Color::GREEN;

pub struct Paddle<E: PaddleLike> {
    pub position: glam::Vec2,
    pub vertical_range: f32,
    pub bounding_area: (f32, f32),
    pub speed: f32,
    pub velocity: f32,
    color: graphics::Color,
    paddle_like: E,
}

impl<E: PaddleLike> Paddle<E> {
    pub fn new(
        x: f32,
        y: f32,
        color: graphics::Color,
        paddle_like: E,
        height: f32,
        bounding_area: (f32, f32),
        speed: f32,
    ) -> Self {
        Self {
            position: glam::vec2(x, y),
            vertical_range: height / 2.0,
            bounding_area,
            speed,
            velocity: 0.0,
            color,
            paddle_like,
        }
    }

    pub fn bouncing(&self, ball: &BallAbstract) -> Option<glam::Vec2> {
        self.paddle_like.bouncing(self, ball)
    }

    pub fn set_position(&mut self, position: impl Into<glam::Vec2>) {
        self.position = position.into();
    }

    pub fn get_position(&self) -> glam::Vec2 {
        self.position
    }

    pub fn get_velocity(&self) -> f32 {
        self.velocity
    }

    pub fn get_color(&self) -> graphics::Color {
        self.color
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        self.paddle_like.draw(self, canvas);
    }

    pub fn reset(&mut self) {
        self.position = glam::vec2(self.position.x, self.bounding_area.1 / 2.0);
    }

    pub fn update(&mut self, dt: f32, up: bool, down: bool) {
        self.velocity = 0.0;
        if up {
            self.velocity -= self.speed;
        }
        if down {
            self.velocity += self.speed;
        }
        self.position.y += self.velocity * dt;
        if self.position.y - self.vertical_range < self.bounding_area.0 {
            self.position.y = self.bounding_area.0 + self.vertical_range;
        } else if self.position.y + self.vertical_range > self.bounding_area.1 {
            self.position.y = self.bounding_area.1 - self.vertical_range;
        }
    }
}

pub fn paddle_from_configuration<
    L: FromConfiguration + PaddleLike,
    R: FromConfiguration + PaddleLike,
>(
    config: &super::configuration::Configuration,
) -> (Paddle<L>, Paddle<R>) {
    (
        Paddle::new(
            config.paddle_width / 2.0,
            config.screen_height / 2.0,
            config.left_paddle_color,
            L::from_configuration(config),
            config.paddle_height,
            (0.0, config.screen_height),
            config.paddle_speed,
        ),
        Paddle::new(
            config.screen_width - config.paddle_width / 2.0,
            config.screen_height / 2.0,
            config.right_paddle_color,
            R::from_configuration(config),
            config.paddle_height,
            (0.0, config.screen_height),
            config.paddle_speed,
        ),
    )
}
