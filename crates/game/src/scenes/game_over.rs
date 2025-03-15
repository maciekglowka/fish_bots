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
        let bounds = game_graphics::get_viewport_bounds(context);
        let _ = context.graphics.draw_text(
            "default",
            &format!("You've got {} points!", game.logic_state.score),
            Vector2f::new(bounds.0.x + 16., bounds.1.y - 24.),
            0,
            8.,
            SpriteParams::default(),
        );

        let input = crate::input::get_input_state(game.main_camera, context);
        if input.click == ButtonState::Released {
            return Some(SceneChange::Pop);
        }
        None
    }
}
