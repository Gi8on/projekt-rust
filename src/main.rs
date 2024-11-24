use ggez::{
    conf,
    event,
    graphics,
    GameError,
    GameResult,
    Context
}

type sdt = f32; // scrren dimension type

const SCREEN_WIDTH : sdt = 800.0;
const SCREEN_HEIGHT : sdt = 600.0;
const SCREEN_DIMS : (sdt, sdt) = (SCREEN_WIDTH, SCREEN_HEIGHT);

const PADDLE_WIDTH :  sdt = 10.0;
const PADDLE_HEIGHT : sdt = 0.3 * SCREEN_HEIGHT;

struct Vector {
    x: sdt,
    y: sdt,
}

impl Vector {
    fn new(x: sdt, y: sdt) -> Self {
        Self {x, y}
    }
}

struct Ball {
    position: Vector,
    velocity_vec: Vector,
}

impl Ball {
    fn new(x: sdt, y: sdt) -> Self {
        Self {
            position: Vector::new(x, y),
            velocity_vec: Vector::new(1.0, 0.0),
        }
    }

    fn update(&mut self, dt: sdt) {
        self.position.x += dt * self.velocity_vec.x;
        self.position.y += dt * self.velocity_vec.y;
    }
}

struct Paddle {
    position: Vector,
}

impl Paddle {
    fn new (x: sdt, y: sdt) -> Self {
        Self {
            position: Vector::new(x, y),
        }
    }

    fn move_vertical(&mut self, dy: sdt) {
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
    let state = State {dt: std::time::Duration::new(0, 0)};
    let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    let (ctx, event_loop) = ContextBuilder::new("pong", "marcin g")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(SCREEN_WIDTH, SCREEN_HEIGHT))
        .build()?;
    event::run(ctx, event_loop, state);
}

impl ggez::event::EventHandler<GameError> for State {

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.dt = ctx.time.delta();
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, graphics::Color::from([0.1, 0.1, 0.1, 1.0]));

        canvas.finish(ctx)?;
    }
}