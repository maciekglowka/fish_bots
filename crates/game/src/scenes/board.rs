use rogalik::prelude::*;

use crate::GameState;

#[derive(Default)]
pub(crate) struct Board {
    graphics_state: game_graphics::board::BoardGraphics,
}
impl<'a> Scene for Board {
    type Game = GameState;

    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game_logic::board::board_init(&mut game.logic_state);
        game_graphics::board::board_init(&mut self.graphics_state, &mut game.logic_state);
    }
    fn exit(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game_graphics::board::board_exit(&mut self.graphics_state, &game.logic_state);
        game_logic::board::board_exit(&mut game.logic_state);
    }

    fn update(
        &mut self,
        game: &mut Self::Game,
        context: &mut Context,
    ) -> Option<SceneChange<Self::Game>> {
        let input = crate::input::get_input_state(game.main_camera, context);
        if !game_graphics::board::board_draw(
            &mut self.graphics_state,
            &game.logic_state,
            context,
            &input,
        ) {
            game_logic::board::board_update(&mut game.logic_state);
        }
        // match self.logic_state.status {
        //     BoardStatus::Gameover => Some(SceneChange::Switch(Box::new(
        //         super::game_over::GameOver::default(),
        //     ))),
        //     BoardStatus::Descend => Some(SceneChange::Switch(Box::new(
        //         super::upgrade::Upgrade::default(),
        //     ))),
        //     BoardStatus::Running => None,
        // }
        None
    }
}
