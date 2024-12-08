use ggez::{
    glam,
    graphics::{self, Canvas},
};

pub struct Ball {
    position: glam::Vec2,
    radius: f32,
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
            position: glam::vec2(x, y),
            radius,
            velocity_vec: initial_velocity,
            bounding_area,
            ball_mesh: graphics::Mesh::new_circle(
                ctx,
                ggez::graphics::DrawMode::fill(),
                glam::vec2(0.0,0.0),
                radius,
                1.0, 
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

    pub fn _get_position(&self) -> glam::Vec2 {
        self.position
    }

    // costlier but more accurate version of update
    pub fn update_different(&mut self, dt: f32) {
        let mut bb = self.bounding_area;
        bb.0 += self.radius;
        bb.1 += self.radius;
        bb.2 -= self.radius;
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
            self.velocity_vec.x = -self.velocity_vec.x;
            self.position.x = bb.0 + x1
        } else if x2 > 0.0 {
            self.velocity_vec.x = -self.velocity_vec.x;
            self.position.x = bb.2 - x2;
        }
    }

    pub fn draw(&self, canvas: &mut Canvas) {
        canvas.draw(&self.ball_mesh, self.position);
    }
}
