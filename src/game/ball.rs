use ggez::{
    glam,
    graphics::{self, Canvas},
    GameResult,
};

use super::configuration::{Configuration, FromConfiguration};
pub struct BallAbstract {
    pub initial_position: glam::Vec2,
    pub position: glam::Vec2,
    pub radius: f32,
    pub initial_velocity: glam::Vec2,
    pub velocity_vec: glam::Vec2,
    pub bounding_area: (f32, f32, f32, f32),
}

impl BallAbstract {
    pub fn new(
        x: f32,
        y: f32,
        radius: f32,
        initial_velocity: glam::Vec2,
        bounding_area: (f32, f32, f32, f32),
    ) -> Self {
        Self {
            initial_position: glam::vec2(x, y),
            position: glam::vec2(x, y),
            radius,
            initial_velocity,
            velocity_vec: initial_velocity,
            bounding_area,
        }
    }

    pub fn reset(&mut self) {
        self.position = self.initial_position;
        self.velocity_vec = self.initial_velocity;
    }

    pub fn get_position(&self) -> glam::Vec2 {
        self.position
    }

    pub fn get_radius(&self) -> f32 {
        self.radius
    }

    pub fn get_velocity(&self) -> glam::Vec2 {
        self.velocity_vec
    }

    pub fn set_velocity(&mut self, velocity: glam::Vec2) {
        self.velocity_vec = velocity;
    }

    pub fn set_position(&mut self, position: impl Into<glam::Vec2>) {
        self.position = position.into();
    }

    pub fn clamp_velocity(&mut self, max_speed: f32, min_speed: f32) {
        let speed = self.velocity_vec.length();
        if speed > max_speed {
            self.velocity_vec = self.velocity_vec.normalize() * max_speed;
        } else if speed < min_speed {
            self.velocity_vec = self.velocity_vec.normalize() * min_speed;
        }
    }

    pub fn update_different(&mut self, dt: f32) -> GameResult<Option<bool>> {
        let mut bb = self.bounding_area;
        bb.1 += self.radius;
        bb.3 -= self.radius;

        self.position += dt * self.velocity_vec;
        let y1 = bb.1 - self.position.y;
        let y2 = self.position.y - bb.3;
        if y1 > 0.0 {
            self.velocity_vec.y = -self.velocity_vec.y;
            self.position.y = bb.1 + y1
        } else if y2 > 0.0 {
            self.velocity_vec.y = -self.velocity_vec.y;
            self.position.y = bb.3 - y2;
        }
        let x1 = bb.0 - self.position.x;
        let x2 = self.position.x - bb.2;
        if x1 > 0.0 {
            return Ok(Some(true));
        } else if x2 > 0.0 {
            return Ok(Some(false));
        }
        Ok(None)
    }
}

impl FromConfiguration for BallAbstract {
    fn from_configuration(config: &Configuration) -> Self {
        Self::new(
            config.screen_width / 2.0,
            config.screen_height / 2.0,
            config.ball_radius,
            config.ball_initial_velocity,
            (0.0, 0.0, config.screen_width, config.screen_height),
        )
    }
}

pub struct Ball {
    pub ball_abstract: BallAbstract,
    ball_mesh: graphics::Mesh,
}

// bb.1=> +----------+  y
//        |          |  |
//        |          |  |
// bb.3=> +----------+  \/
//        /\         /\
//        bb.0       bb.2
//     x -------------->

impl Ball {
    pub fn new(
        x: f32,
        y: f32,
        radius: f32,
        initial_velocity: glam::Vec2,
        ball_color: graphics::Color,
        bounding_area: (f32, f32, f32, f32),
        ctx: &ggez::Context,
    ) -> Self {
        Self {
            ball_abstract: BallAbstract {
                initial_position: glam::vec2(x, y),
                position: glam::vec2(x, y),
                radius,
                initial_velocity,
                velocity_vec: initial_velocity,
                bounding_area,
            },
            ball_mesh: graphics::Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                glam::vec2(0.0, 0.0),
                radius,
                0.1,
                ball_color,
            )
            .unwrap(),
        }
    }

    // pub fn update(&mut self, dt: f32) {
    //     let bb = self.bounding_area;
    //     if self.position.y - self.radius < bb.1 || self.position.y + self.radius > bb.2 {
    //         self.velocity_vec.y = -self.velocity_vec.y;
    //     }
    //     if self.position.x - self.radius < bb.0 || self.position.x + self.radius > bb.2 {
    //         self.velocity_vec.x = -self.velocity_vec.x;
    //     }
    //     self.position += dt * self.velocity_vec;
    // }

    pub fn reset(&mut self) {
        self.ball_abstract.reset()
    }

    pub fn get_position(&self) -> glam::Vec2 {
        self.ball_abstract.get_position()
    }

    pub fn get_radius(&self) -> f32 {
        self.ball_abstract.get_radius()
    }

    pub fn get_velocity(&self) -> glam::Vec2 {
        self.ball_abstract.get_velocity()
    }

    pub fn set_velocity(&mut self, velocity: glam::Vec2) {
        self.ball_abstract.set_velocity(velocity);
    }

    pub fn set_position(&mut self, position: impl Into<glam::Vec2>) {
        self.ball_abstract.set_position(position);
    }

    // costlier but more accurate version of update
    pub fn update_different(&mut self, dt: f32) -> GameResult<Option<bool>> {
        self.ball_abstract.update_different(dt)
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.ball_mesh, self.ball_abstract.position);
    }

    pub fn from_configuration(config: &Configuration, ctx: &ggez::Context) -> Self {
        Self::new(
            config.screen_width / 2.0,
            config.screen_height / 2.0,
            config.ball_radius,
            config.ball_initial_velocity,
            config.ball_color,
            (0.0, 0.0, config.screen_width, config.screen_height),
            ctx,
        )
    }
}
