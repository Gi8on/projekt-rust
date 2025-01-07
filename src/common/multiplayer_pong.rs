use crate::ball::BallAbstract;
use crate::configuration::FromConfiguration;
use crate::paddle::paddle_from_configuration;
use crate::state::{game_frame, Input, RoundResult};
use crate::{paddle::Paddle, paddle_like::RectangularPaddle};

use super::messages::{GameState, Tick};

pub struct MultiplayerPong {
    pub ball: BallAbstract,
    pub left_paddle: Paddle<RectangularPaddle>,
    pub right_paddle: Paddle<RectangularPaddle>,
}

impl FromConfiguration for MultiplayerPong {
    fn from_configuration(config: &crate::configuration::Configuration) -> Self {
        let ball = BallAbstract::from_configuration(config);
        let (left_paddle, right_paddle) = paddle_from_configuration(config);
        Self::new(ball, left_paddle, right_paddle)
    }
}

impl MultiplayerPong {
    pub fn new(
        ball: BallAbstract,
        left_paddle: Paddle<RectangularPaddle>,
        right_paddle: Paddle<RectangularPaddle>,
    ) -> Self {
        Self {
            ball,
            left_paddle,
            right_paddle,
        }
    }

    pub fn multi_game_round(
        &mut self,
        input_left: (bool, bool),
        input_right: (bool, bool),
        dt: f32,
        tick: Tick,
    ) -> (RoundResult, GameState) {
        let input = Input::new(input_left.0, input_left.1, input_right.0, input_right.1);
        // println!("{:?}", input);
        let rr = game_frame(
            &mut self.ball,
            &mut self.left_paddle,
            &mut self.right_paddle,
            dt,
            &input,
        );
        let game_state = GameState::new(
            tick,
            self.ball.get_position().into(),
            self.left_paddle.get_position().into(),
            self.right_paddle.get_position().into(),
        );
        (rr, game_state)
    }
}
