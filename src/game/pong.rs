use super::{configuration::Configuration, paddle_like::RectangularPaddle};
use ggez::{event, GameResult};

use super::state::State;

pub fn pong() -> GameResult {
    let config = Configuration::default();
    let screen_width = config.screen_width;
    let screen_height = config.screen_height;
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("pong", "marcin g")
        .window_setup(ggez::conf::WindowSetup::default().title("Pong"))
        .window_mode(ggez::conf::WindowMode::default().dimensions(screen_width, screen_height))
        .build()?;
    let state = State::<RectangularPaddle, RectangularPaddle>::new(config, &mut ctx);
    // let mut c = conf::Conf::new();
    // c.window_mode(ggez::conf::WindowMode::default().dimensions(800.0, 600.0));
    event::run(ctx, event_loop, state);
}
