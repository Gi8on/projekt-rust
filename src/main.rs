use ggez::{
    event, glam::{self, *}, graphics, Context, GameError, GameResult
};
type Sdt = f32; // Scren dimension type

const SCREEN_WIDTH : Sdt = 800.0;
const SCREEN_HEIGHT : Sdt = 600.0;
const _SCREEN_DIMS : (Sdt, Sdt) = (SCREEN_WIDTH, SCREEN_HEIGHT);

const PADDLE_WIDTH :  Sdt = 10.0;
const PADDLE_HEIGHT : Sdt = 0.3 * SCREEN_HEIGHT;
const PADDLE_SPEED : Sdt = 100.0;

const INITIAL_BALL_VELOCITY : glam::Vec2 = glam::vec2(500.0, 500.0);
const BALL_RADIUS : Sdt = 15.0;
const BALL_COLOR : graphics::Color = graphics::Color::WHITE;
const BALL_EXACTNESS : f32 = 0.1;

const INNER_LEFT_PADDING : Sdt = BALL_RADIUS;
const INNER_RIGHT_PADDING : Sdt = SCREEN_WIDTH - BALL_RADIUS;
const INNER_TOP_PADDING : Sdt = BALL_RADIUS;
const INNER_BOTTOM_PADDING : Sdt = SCREEN_HEIGHT - BALL_RADIUS;

const LEFT_PADDLE_COLOR : graphics::Color = graphics::Color::RED;
const RIGHT_PADDLE_COLOR : graphics::Color = graphics::Color::BLUE;

const SCREEN_COLOR : graphics::Color = graphics::Color::BLACK;

const DESIRED_FPS : u32 = 60;

struct Ball {
    position: glam::Vec2,
    radius: Sdt,
    velocity_vec: glam::Vec2,
}

impl Ball {
    fn new(x: Sdt, y: Sdt) -> Self {
        Self {
            position: glam::vec2(x, y),
            radius: BALL_RADIUS,
            velocity_vec: INITIAL_BALL_VELOCITY
        }
    }

    fn update(&mut self, dt: Sdt) {
        if self.position.y - self.radius < 0.0 || self.position.y + self.radius > SCREEN_HEIGHT {
            self.velocity_vec.y = -self.velocity_vec.y;
        }
        if self.position.x - self.radius < 0.0 || self.position.x + self.radius > SCREEN_WIDTH {
            self.velocity_vec.x = -self.velocity_vec.x;
        }
        self.position += dt * self.velocity_vec;
    }

    // costlier but more accurate version of update
    fn update_different(&mut self, dt: Sdt) {
        self.position += dt * self.velocity_vec;
        let y1 = INNER_TOP_PADDING - self.position.y;
        let y2 = self.position.y - INNER_BOTTOM_PADDING;
        if y1 > 0.0 {
            self.velocity_vec.y = -self.velocity_vec.y;
            self.position.y = INNER_TOP_PADDING + y1
        } else if y2 > 0.0 {
            self.velocity_vec.y = -self.velocity_vec.y;
            self.position.y = INNER_BOTTOM_PADDING - y2;
        }
        let x1 = INNER_LEFT_PADDING - self.position.x;
        let x2 = self.position.x - INNER_RIGHT_PADDING;
        if x1 > 0.0 {
            self.velocity_vec.x = -self.velocity_vec.x;
            self.position.x = INNER_LEFT_PADDING + x1
        } else if x2 > 0.0 {
            self.velocity_vec.x = -self.velocity_vec.x;
            self.position.x = INNER_RIGHT_PADDING - x2;
        }
    }
}

struct Paddle {
    position: glam::Vec2,
    height: Sdt,
    width: Sdt,
}

impl Paddle {
    fn new (x: Sdt, y: Sdt) -> Self {
        Self {
            position: glam::vec2(x, y),
            height: PADDLE_HEIGHT,
            width: PADDLE_WIDTH,
        }
    }

    fn update(&mut self, dt: Sdt, up: bool, down: bool) {
        let mut velocity = 0.0;
        if up {
            velocity -= PADDLE_SPEED;
        }
        if down {
            velocity += PADDLE_SPEED;
        }
        self.position.y += velocity * dt;
        if self.position.y - self.height / 2.0 < 0.0 {
            self.position.y = self.height / 2.0;
        } else if self.position.y + self.height / 2.0 > SCREEN_HEIGHT {
            self.position.y = SCREEN_HEIGHT - self.height / 2.0;
        }
    }
}

struct Input {
    left_up: bool,
    left_down: bool,
    right_up: bool,
    right_down: bool,
}

struct State {
    paddle_left: Paddle,
    paddle_right: Paddle,
    input: Input,
    ball: Ball,
}

impl State {
    fn new() -> Self {
        Self {
            paddle_left: Paddle::new(PADDLE_WIDTH, SCREEN_HEIGHT / 2.0),
            paddle_right: Paddle::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0),
            ball: Ball::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
            input: Input {
                left_up: false,
                left_down: false,
                right_up: false,
                right_down: false,
            }
        }
    }
}

fn main() -> GameResult{
    let state = State::new();
    // let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = ggez::ContextBuilder::new("pong", "marcin g")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        // let dt :f32 = ctx.time.delta();
        let dt = 1.0 / DESIRED_FPS as f32;
        let mut num_of_updates = 0;
        while ctx.time.check_update_time(DESIRED_FPS) {
            self.ball.update_different(dt);
            self.paddle_left.update(dt, self.input.left_up, self.input.left_down);
            self.paddle_right.update(dt, self.input.right_up, self.input.right_down);
            num_of_updates += 1;
            if num_of_updates > 1 {
                println!("num of updates: {}", num_of_updates);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, SCREEN_COLOR);

        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            self.ball.position,
            self.ball.radius,
            BALL_EXACTNESS,
            BALL_COLOR,
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());

        let paddle_left = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                self.paddle_left.position.x - self.paddle_left.width / 2.0,
                self.paddle_left.position.y - self.paddle_left.height / 2.0,
                self.paddle_left.width,
                self.paddle_left.height,
            ),
            LEFT_PADDLE_COLOR,
        )?;

        let paddle_right = ggez::graphics::Mesh::new_rectangle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            ggez::graphics::Rect::new(
                self.paddle_right.position.x - self.paddle_right.width / 2.0,
                self.paddle_right.position.y - self.paddle_right.height / 2.0,
                self.paddle_right.width,
                self.paddle_right.height,
            ),
            RIGHT_PADDLE_COLOR,
        )?;

        canvas.draw(&paddle_left, graphics::DrawParam::default());
        canvas.draw(&paddle_right, graphics::DrawParam::default());

        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(&mut self, _ctx: &mut Context, keyinput: ggez::input::keyboard::KeyInput,  _repeat: bool) -> GameResult {
        match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::W) => self.input.left_up = true,
            Some(ggez::input::keyboard::KeyCode::S) => self.input.left_down = true,
            Some(ggez::input::keyboard::KeyCode::Up) => self.input.right_up = true,
            Some(ggez::input::keyboard::KeyCode::Down) => self.input.right_down = true,
            _ => (),
        }
        Ok(())
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keyinput: ggez::input::keyboard::KeyInput) -> GameResult {
        match keyinput.keycode {
            Some(ggez::input::keyboard::KeyCode::W) => self.input.left_up = false,
            Some(ggez::input::keyboard::KeyCode::S) => self.input.left_down = false,
            Some(ggez::input::keyboard::KeyCode::Up) => self.input.right_up = false,
            Some(ggez::input::keyboard::KeyCode::Down) => self.input.right_down = false,
            _ => (),
        }
        Ok(())
    }
}