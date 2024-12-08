use ggez::{glam, graphics};

pub struct Paddle {
    position: glam::Vec2,
    height: f32,
    width: f32,
    color: graphics::Color,
    bounding_area: (f32, f32),
    speed: f32,
}

impl Paddle {
    pub fn new(
        x: f32,
        y: f32,
        paddle_width: f32,
        paddle_color: graphics::Color,
        paddle_height: f32,
        bounding_area: (f32, f32),
        speed: f32,
    ) -> Self {
        Self {
            position: glam::vec2(x, y),
            height: paddle_height,
            width: paddle_width,
            color: paddle_color,
            bounding_area,
            speed,
        }
    }

    pub fn update(&mut self, dt: f32, up: bool, down: bool) {
        let mut velocity = 0.0;
        if up {
            velocity -= self.speed;
        }
        if down {
            velocity += self.speed;
        }
        self.position.y += velocity * dt;
        if self.position.y - self.height / 2.0 < self.bounding_area.0 {
            self.position.y = self.bounding_area.0 + self.height / 2.0;
        } else if self.position.y + self.height / 2.0 > self.bounding_area.1 {
            self.position.y = self.bounding_area.1 - self.height / 2.0;
        }
    }

    pub fn draw(&self, ctx: &mut ggez::Context, canvas: &mut graphics::Canvas) {
        let rect = graphics::Rect::new(
            self.position.x - self.width / 2.0,
            self.position.y - self.height / 2.0,
            self.width,
            self.height,
        );
        let mesh = graphics::Mesh::new_rectangle(ctx, graphics::DrawMode::fill(), rect, self.color)
            .unwrap();
        canvas.draw(&mesh, graphics::DrawParam::default())
    }
}
