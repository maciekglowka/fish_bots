use rogalik::prelude::*;

use game_graphics::input::ButtonState;

use crate::GameState;

#[derive(Default)]
pub(crate) struct GameOver;

impl<'a> Scene for GameOver {
    type Game = GameState;

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        let _ = context.graphics.draw_text(
            "default",
            "Game Over :(",
            Vector2f::ZERO,
            0,
            16.,
            SpriteParams::default(),
        );

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            return Some(SceneChange::Pop);
        }
        None
    }
}
