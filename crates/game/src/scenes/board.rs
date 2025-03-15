use rogalik::prelude::*;

use crate::GameState;

#[derive(Default)]
pub(crate) struct Board {
    graphics_state: game_graphics::board::BoardGraphics,
}
impl Board {
    #[cfg(not(target_arch = "wasm32"))]
    fn get_code(&self) -> String {
        String::new()
    }
    #[cfg(target_arch = "wasm32")]
    fn get_code(&self) -> String {
        crate::web::get_bot_code()
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn update_output(&self, _s: String) {
        // TODO
    }
    #[cfg(target_arch = "wasm32")]
    fn update_output(&self, s: String) {
        crate::web::write_output(s);
    }
    #[cfg(not(target_arch = "wasm32"))]
    fn get_config(&self) -> game_logic::Config {
        game_logic::Config::default()
    }
    #[cfg(target_arch = "wasm32")]
    fn get_config(&self) -> game_logic::Config {
        crate::web::get_config()
    }
}
impl<'a> Scene for Board {
    type Game = GameState;

    fn enter(&mut self, game: &mut Self::Game, _context: &mut rogalik::engine::Context) {
        game_logic::board::board_init(&mut game.logic_state, self.get_code(), self.get_config());
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
            if game.logic_state.done {
                return Some(SceneChange::Switch(Box::new(
                    super::game_over::GameOver::default(),
                )));
            }
            game_logic::board::board_update(&mut game.logic_state);
            for s in game.logic_state.console.as_ref().unwrap().read() {
                self.update_output(s);
            }
        }

        if self.graphics_state.reload {
            return Some(SceneChange::Pop);
        }
        None
    }
}
