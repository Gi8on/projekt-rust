use super::ball::Ball;
use super::paddle_like::PaddleLike;
use ggez::{glam, graphics};

const _HIT_COLOR: graphics::Color = graphics::Color::GREEN;

pub struct Paddle<E: PaddleLike> {
    position: glam::Vec2,
    vertical_range: f32,
    color: graphics::Color,
    paddle_like: E,
    bounding_area: (f32, f32),
    speed: f32,
    velocity: f32,
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
            color,
            paddle_like,
            bounding_area,
            speed,
            velocity: 0.0,
        }
    }

    pub fn bouncing(&self, ball: &Ball) -> Option<glam::Vec2> {
        self.paddle_like.bouncing(self, ball)
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
}
