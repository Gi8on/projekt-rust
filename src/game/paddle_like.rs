use super::ball::Ball;
use ggez::{glam, graphics};

pub trait Paddle_like {
    pub fn draw();
    pub fn update();
    pub fn bouncing();
}