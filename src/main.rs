use ggez::{
    event, glam::{self, *}, graphics, Context, GameError, GameResult
};
type Sdt = f32; // Scren dimension type

const SCREEN_WIDTH : Sdt = 800.0;
const SCREEN_HEIGHT : Sdt = 600.0;
const _SCREEN_DIMS : (Sdt, Sdt) = (SCREEN_WIDTH, SCREEN_HEIGHT);

const PADDLE_WIDTH :  Sdt = 10.0;
const PADDLE_HEIGHT : Sdt = 0.3 * SCREEN_HEIGHT;
const _PADDLE_SPEED : Sdt = 100.0;

const INITIAL_BALL_VELOCITY : glam::Vec2 = glam::vec2(500.0, 500.0);
const BALL_RADIUS : Sdt = 50.0;
const BALL_COLOR : graphics::Color = graphics::Color::WHITE;
const BALL_EXACTNESS : f32 = 0.1;

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
}

struct Paddle {
    position: glam::Vec2,
    velocity: Sdt,
    height: Sdt,
    width: Sdt,
}

impl Paddle {
    fn new (x: Sdt, y: Sdt) -> Self {
        Self {
            position: glam::vec2(x, y),
            velocity: 0.0,
            height: PADDLE_HEIGHT,
            width: PADDLE_WIDTH,
        }
    }

    fn move_vertical(&mut self, dy: Sdt) {
        self.position.y += dy;
    }
}

struct State {
    paddle_left: Paddle,
    paddle_right: Paddle,
    ball: Ball,
}

impl State {
    fn new() -> Self {
        Self {
            paddle_left: Paddle::new(PADDLE_WIDTH, SCREEN_HEIGHT / 2.0),
            paddle_right: Paddle::new(SCREEN_WIDTH - PADDLE_WIDTH, SCREEN_HEIGHT / 2.0),
            ball: Ball::new(SCREEN_WIDTH / 2.0, SCREEN_HEIGHT / 2.0),
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
            self.ball.update(dt);
            //self.paddle_left.update(dt);
            //self.paddle_right.update(dt);
            num_of_updates += 1;
            if num_of_updates > 1 {
                println!("num of updates: {}", num_of_updates);
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

        let circle = ggez::graphics::Mesh::new_circle(
            ctx,
            ggez::graphics::DrawMode::fill(),
            self.ball.position,
            self.ball.radius,
            BALL_EXACTNESS,
            BALL_COLOR,
        )?;
        canvas.draw(&circle, graphics::DrawParam::default());
        canvas.finish(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }
}