use super::ball::BallAbstract;
use super::configuration::Configuration;
use super::configuration::FromConfiguration;
use super::paddle::Paddle;
use ggez::{glam, graphics};

pub trait PaddleLike {
    fn draw(&self, paddle: &Paddle<impl PaddleLike>, canvas: &mut graphics::Canvas);
    fn bouncing(&self, paddle: &Paddle<impl PaddleLike>, ball: &BallAbstract)
        -> Option<glam::Vec2>;
}

pub struct RectangularPaddle {
    half_height: f32,
    half_width: f32,
    paddle_to_ball_speedup: f32,
}

impl RectangularPaddle {
    pub fn new(half_height: f32, half_width: f32, paddle_to_ball_speedup: f32) -> Self {
        Self {
            half_height,
            half_width,
            paddle_to_ball_speedup,
        }
    }
}

impl FromConfiguration for RectangularPaddle {
    fn from_configuration(config: &Configuration) -> Self {
        Self::new(
            config.paddle_height / 2.0,
            config.paddle_width / 2.0,
            config.paddle_to_ball_speedup,
        )
    }
}

impl PaddleLike for RectangularPaddle {
    fn draw(&self, paddle: &Paddle<impl PaddleLike>, canvas: &mut graphics::Canvas) {
        canvas.draw(
            &graphics::Quad,
            graphics::DrawParam::new()
                .dest(paddle.get_position() - glam::vec2(self.half_width, self.half_height))
                .scale(2.0 * glam::vec2(self.half_width, self.half_height))
                .color(paddle.get_color()),
        );
    }

    fn bouncing(
        &self,
        paddle: &Paddle<impl PaddleLike>,
        ball: &BallAbstract,
    ) -> Option<glam::Vec2> {
        let ball_pos = ball.get_position();
        let r = ball.get_radius();
        let mut ball_vel = ball.get_velocity();
        let my_pos = paddle.get_position();
        let my_vel = paddle.get_velocity();
        let y_hit = ball_pos.y - my_pos.y;
        if (ball_pos.x - my_pos.x).abs() < r + self.half_width && y_hit.abs() < self.half_height {
            ball_vel.x = -ball_vel.x;
            ball_vel += glam::Vec2::splat(my_vel * my_vel.signum() * ball_vel.y.signum())
                * self.paddle_to_ball_speedup;
            let frac = ball_vel.x.signum() * y_hit / self.half_height;
            ball_vel = glam::Vec2::from_angle(std::f32::consts::FRAC_PI_4 * frac).rotate(ball_vel);
            // println!("ball vel {}", ball_vel);
            return Some(ball_vel);
        }
        None
    }
}
