use serde::{Deserialize, Serialize};

use ggez::{
    glam,
    graphics::{self, Canvas},
    GameResult,
};

#[derive(Serialize, Deserialize, Debug)]
pub struct BallData {
    position: (f32, f32),
    velocity: (f32, f32),
}

pub struct Ball {
    initial_position: glam::Vec2,
    position: glam::Vec2,
    radius: f32,
    initial_velocity: glam::Vec2,
    velocity_vec: glam::Vec2,
    bounding_area: (f32, f32, f32, f32),
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
            initial_position: glam::vec2(x, y),
            position: glam::vec2(x, y),
            radius,
            initial_velocity,
            velocity_vec: initial_velocity,
            bounding_area,
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

    // costlier but more accurate version of update
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

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.ball_mesh, self.position);
    }

    pub fn serialize(&self) -> BallData {
        BallData {
            position: self.position.into(),
            velocity: self.velocity_vec.into(),
        }
    }
}
