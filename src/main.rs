use ggez::GameResult;

mod game;

use game::pong::pong;

fn main() -> GameResult {
    pong()
}
