use super::ball::Ball;
use ggez::{glam, graphics};

const PADDLE_TO_BALL_SPEEDUP: f32 = 0.1;
const HIT_COLOR: graphics::Color = graphics::Color::GREEN;
pub struct Paddle {
    position: glam::Vec2,
    half_height: f32,
    half_width: f32,
    color: graphics::Color,
    bounding_area: (f32, f32),
    speed: f32,
    velocity: f32,
}

impl Paddle {
    pub fn new(
        x: f32,
        y: f32,
        width: f32,
        color: graphics::Color,
        height: f32,
        bounding_area: (f32, f32),
        speed: f32,
    ) -> Self {
        Self {
            position: glam::vec2(x, y),
            half_height: height / 2.0,
            half_width: width / 2.0,
            color,
            bounding_area,
            speed,
            velocity: 0.0,
        }
    }

    pub fn bouncing(&self, ball: &Ball) -> Option<glam::Vec2> {
        let ball_pos = ball.get_position();
        let r = ball.get_radius();
        let mut ball_vel = ball.get_velocity();
        if (ball_pos.x - self.position.x).abs() < r + self.half_width
            && (ball_pos.y - self.position.y).abs() < self.half_height
        {
            ball_vel.x = -ball_vel.x;
            ball_vel += glam::Vec2::splat(self.velocity * self.velocity.signum() * ball_vel.y.signum()) * PADDLE_TO_BALL_SPEEDUP;
            return Some(ball_vel);
        }
        None
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
        if self.position.y - self.half_height < self.bounding_area.0 {
            self.position.y = self.bounding_area.0 + self.half_height;
        } else if self.position.y + self.half_height > self.bounding_area.1 {
            self.position.y = self.bounding_area.1 - self.half_height;
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
                graphics::DrawParam::new()
                    .dest(self.position - glam::vec2(self.half_width, self.half_height))
                    .scale(2.0 * glam::vec2(self.half_width, self.half_height))
                    .color(self.color),
            );
    }
}
